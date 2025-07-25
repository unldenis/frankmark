mod config;
mod error;
mod utils;

use std::{
    env,
    fs::{self, File},
    io::Write,
};

use askama::Template; // bring trait in scope

use config::parse_config;
use error::FrankmarkResult;

use crate::{config::Config, error::FrankmarkError};

#[derive(Template)] // this will generate the code...
#[template(path = "main.html")] // using the template in this path, relative
// to the `templates` dir in the crate root
struct MainTemplate<'a> {
    // the name of the struct can be anything
    title: String,
    github_url: String,
    folders: &'a Vec<Folder>,
    current_page: &'a Page,

    previous_page: Option<&'a Page>,
    next_page: Option<&'a Page>,
    is_root: bool,
}

impl<'a> MainTemplate<'a> {
    fn new(
        config: &Config,
        folders: &'a Vec<Folder>,
        current_page: &'a Page,
        previous_page: Option<&'a Page>,
        next_page: Option<&'a Page>,
        is_root: bool,
    ) -> Self {
        Self {
            title: "Test Page".to_string(),
            github_url: config
                .package
                .github_url
                .clone()
                .unwrap_or("https://github.com/unldenis/frankmark".to_string()),
            folders: folders,
            current_page: current_page,
            previous_page: previous_page,
            next_page: next_page,
            is_root: is_root,
        }
    }

    pub fn get_folder_by_page(&self, page: &Page) -> &Folder {
        self.folders
            .iter()
            .find(|f| f.pages.iter().any(|p| p.id == page.id))
            .unwrap()
    }

    pub fn get_page_url(&self, page: &Page) -> String {
        let folder = self.get_folder_by_page(page);
        if self.is_root {
            format!("{}/{}.html", folder.name, page.display_name)
        } else {
            format!("../{}/{}.html", folder.name, page.display_name)
        }
    }

    pub fn is_current_page_folder(&self, folder: &Folder) -> String {
        let current_folder = self.get_folder_by_page(self.current_page);

        if current_folder.name == folder.name {
            return "uk-open".to_string();
        }

        "".to_string()
    }
}

#[derive(Debug)]
struct Folder {
    name: String,
    pages: Vec<Page>,
}

impl Folder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            pages: Vec::new(),
        }
    }

    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }
}

// Global navigation functions that work across all folders
fn get_global_next_page<'a>(folders: &'a [Folder], current_page: &Page) -> Option<&'a Page> {
    let mut found_current = false;

    for folder in folders {
        for page in &folder.pages {
            if found_current {
                return Some(page);
            }
            if page.id == current_page.id {
                found_current = true;
            }
        }
    }

    None
}

fn get_global_previous_page<'a>(folders: &'a [Folder], current_page: &Page) -> Option<&'a Page> {
    let mut previous_page: Option<&'a Page> = None;

    for folder in folders {
        for page in &folder.pages {
            if page.id == current_page.id {
                return previous_page;
            }
            previous_page = Some(page);
        }
    }

    None
}

#[derive(Debug)]
struct Page {
    id: String,
    #[allow(dead_code)]
    full_name: String,
    display_name: String,
    content: String,
}

impl Page {
    pub fn new(full_name: String, display_name: String, content: String) -> Self {
        Self {
            id: utils::generate_id(10),
            full_name,
            display_name,
            content,
        }
    }

    pub fn is_active(&self, current_page: &Page) -> bool {
        self.id == current_page.id
    }
}

fn parse_directory(config: &Config, config_folder_path: &str) -> FrankmarkResult<Vec<Folder>> {
    let mut folders = Vec::new();

    // Get all directories in the config folder
    let config_folder_entries: Vec<_> = fs::read_dir(config_folder_path)?
        .collect::<std::result::Result<Vec<_>, std::io::Error>>()?;

    // for each directory in the config
    for (folder_name, folder_pages) in config.directories.iter() {
        // Find the corresponding folder in the filesystem
        let folder_entry = config_folder_entries.iter().find(|entry| {
            if let Ok(metadata) = entry.metadata() {
                metadata.is_dir() && entry.file_name().to_string_lossy() == *folder_name
            } else {
                false
            }
        });

        let folder_entry = match folder_entry {
            Some(entry) => entry,
            None => {
                eprintln!(
                    "Warning: Folder '{}' not found in filesystem, skipping",
                    folder_name
                );
                continue;
            }
        };

        let folder_path = folder_entry.path();
        let mut folder = Folder::new(folder_name.to_string());

        // Check if all pages from config exist and parse them
        for page_name in folder_pages.iter() {
            let page_file_path = folder_path.join(format!("{}.md", page_name));

            if !page_file_path.exists() {
                println!("Page {} not found in folder {}", page_name, folder_name);
                continue;
            }

            // Read content from the markdown file
            let content = match fs::read_to_string(&page_file_path) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to read page '{}': {}, using default content",
                        page_name, e
                    );
                    format!("# {}", page_name)
                }
            };

            // Convert markdown to FrankenUi HTML
            let content =
                markdown::to_html_frankenui_with_options(&content, &markdown::Options::gfm())
                    .map_err(|e| FrankmarkError::MarkdownError(e))?;

            let page = Page::new(page_name.to_string(), page_name.to_string(), content);
            folder.add_page(page);
        }

        if !folder.pages.is_empty() {
            let page_count = folder.pages.len();
            folders.push(folder);
            println!("Added folder: {} with {} pages", folder_name, page_count);
        }
    }

    Ok(folders)
}

fn generate_site(folder_path: &str) -> FrankmarkResult<()> {
    // Construct the config file path
    let config_path = format!("{}/frankmark.toml", folder_path);

    // Parse the configuration file
    let config = parse_config(&config_path)?;
    println!("✓ Configuration loaded successfully");

    // Access the parsed data
    println!("GitHub URL: {:?}", config.package.github_url);
    println!("Directories: {:?}", config.directories);

    // Use the provided folder path as the source directory
    let source_dir = folder_path;

    // try to parse the source directory
    let folders = parse_directory(&config, source_dir)?;
    println!("✓ Found {} folders to process", folders.len());

    // Build output in the same directory as the config file
    let output_path = format!("{}/output", folder_path);

    // create the folder if it doesn't exist
    fs::create_dir_all(&output_path)?;

    // delete contents of the folder
    if fs::metadata(&output_path).is_ok() {
        fs::remove_dir_all(&output_path)?;
    }
    fs::create_dir_all(&output_path)?;

    let mut total_pages = 0;
    let mut first_page: Option<&Page> = None;
    for folder in folders.iter() {
        // create the folder if it doesn't exist
        fs::create_dir_all(format!("{}/{}", output_path, folder.name))?;

        for page in folder.pages.iter() {
            let page_template = MainTemplate::new(
                &config,
                &folders,
                page,
                get_global_previous_page(&folders, page),
                get_global_next_page(&folders, page),
                false, // not root for subfolder pages
            ); // instantiate your struct
            let rendered = page_template.render()?; // then render it.

            // write to file
            let file_name = format!("{}/{}/{}.html", output_path, folder.name, page.display_name);
            let mut file = File::create(file_name.clone())?;
            file.write_all(rendered.as_bytes())?;

            println!("✓ Generated {}", file_name);
            total_pages += 1;

            if first_page.is_none() {
                first_page = Some(page);
            }
        }
    }
    if let Some(first_page) = first_page {
        let page_template = MainTemplate::new(
            &config,
            &folders,
            first_page,
            None,
            get_global_next_page(&folders, first_page),
            true, // is root for index.html
        );
        let rendered = page_template.render()?;
        let mut file = File::create(format!("{}/index.html", output_path))?;
        file.write_all(rendered.as_bytes())?;
        println!("✓ Generated index.html");
    }

    println!("✓ Successfully generated {} pages", total_pages);
    Ok(())
}

// Add a wrapper to handle errors gracefully
fn run(folder_path: &str) -> FrankmarkResult<()> {
    generate_site(folder_path)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let folder_path = if args.len() > 1 {
        &args[1]
    } else {
        "docs/example" // default fallback
    };

    if let Err(e) = run(folder_path) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
