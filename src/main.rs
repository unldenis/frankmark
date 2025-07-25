mod config;
mod error;
mod utils;

use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use askama::Template;

use config::parse_config;
use error::FrankmarkResult;
use markdown::mdast::Node;

use crate::config::{Book, Config};

#[derive(Template)]
#[template(path = "main.html")]
struct MainTemplate<'a> {
    book: &'a Book,
    folders: &'a Vec<Folder>,
    current_page: &'a Page,
    previous_page: Option<&'a Page>,
    next_page: Option<&'a Page>,
    is_root: bool,
}

impl<'a> MainTemplate<'a> {
    fn new(
        book: &'a Book,
        folders: &'a Vec<Folder>,
        current_page: &'a Page,
        previous_page: Option<&'a Page>,
        next_page: Option<&'a Page>,
        is_root: bool,
    ) -> Self {
        Self {
            book,
            folders,
            current_page,
            previous_page,
            next_page,
            is_root,
        }
    }

    pub fn get_page_url(&self, page: &Page) -> String {
        if self.is_root {
            format!("{}/{}.html", page.folder_name, page.display_name)
        } else {
            format!("../{}/{}.html", page.folder_name, page.display_name)
        }
    }

    pub fn is_current_page_folder(&self, folder: &Folder) -> String {
        if self.current_page.folder_name == folder.name {
            "uk-open".to_string()
        } else {
            String::new()
        }
    }

    pub fn has_previous_and_next_pages(&self) -> bool {
        self.previous_page.is_some() && self.next_page.is_some()
    }

    pub fn get_page_display_name(&self, page: &Page) -> String {
        if page.folder_name == self.current_page.folder_name {
            page.display_name.clone()
        } else {
            format!("{}/{}", page.folder_name, page.display_name)
        }
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

// Optimized navigation with pre-computed page order
struct PageNavigator<'a> {
    all_pages: Vec<&'a Page>,
    page_to_index: HashMap<&'a str, usize>,
}

impl<'a> PageNavigator<'a> {
    fn new(folders: &'a [Folder]) -> Self {
        let mut all_pages = Vec::new();
        let mut page_to_index = HashMap::new();

        for folder in folders {
            for page in &folder.pages {
                page_to_index.insert(page.id.as_str(), all_pages.len());
                all_pages.push(page);
            }
        }

        Self {
            all_pages,
            page_to_index,
        }
    }

    fn get_next_page(&self, current_page: &Page) -> Option<&'a Page> {
        let current_index = self.page_to_index.get(current_page.id.as_str())?;
        let next_index = current_index + 1;
        if next_index < self.all_pages.len() {
            Some(self.all_pages[next_index])
        } else {
            None
        }
    }

    fn get_previous_page(&self, current_page: &Page) -> Option<&'a Page> {
        let current_index = self.page_to_index.get(current_page.id.as_str())?;
        if *current_index > 0 {
            Some(self.all_pages[current_index - 1])
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Page {
    id: String,
    #[allow(dead_code)]
    full_name: String,
    display_name: String,
    content: String,
    folder_name: String, // Direct reference to folder name
    headings: Vec<Heading>,
}

#[derive(Debug)]
struct Heading {
    text: String,
    level: u8,
}

impl Page {
    pub fn new(
        full_name: String,
        display_name: String,
        content: String,
        folder_name: String,
        headings: Vec<Heading>,
    ) -> Self {
        // Use deterministic ID based on content hash for better performance
        let id = utils::generate_deterministic_id(&full_name);
        Self {
            id,
            full_name,
            display_name,
            content,
            folder_name,
            headings,
        }
    }

    pub fn is_active(&self, current_page: &Page) -> bool {
        self.id == current_page.id
    }
}

// Optimized directory parsing with batch operations
fn parse_directory(config: &Config, config_folder_path: &str) -> FrankmarkResult<Vec<Folder>> {
    let mut folders = Vec::new();
    let config_folder_path = Path::new(config_folder_path);

    // Pre-allocate capacity for better performance
    folders.reserve(config.directories.len());

    // Get all directories in the config folder once
    let config_folder_entries: Vec<_> = fs::read_dir(config_folder_path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .metadata()
                .map(|metadata| metadata.is_dir())
                .unwrap_or(false)
        })
        .collect();

    // Create a lookup map for faster directory finding
    let mut dir_lookup: HashMap<String, PathBuf> = HashMap::new();
    for entry in &config_folder_entries {
        if let Ok(name) = entry.file_name().into_string() {
            dir_lookup.insert(name, entry.path());
        }
    }

    // Process each configured directory
    for (folder_name, folder_pages) in &config.directories {
        let folder_path = match dir_lookup.get(folder_name) {
            Some(path) => path,
            None => {
                eprintln!(
                    "Warning: Folder '{}' not found in filesystem, skipping",
                    folder_name
                );
                continue;
            }
        };

        let mut folder = Folder::new(folder_name.clone());
        folder.pages.reserve(folder_pages.len());

        // Batch read all markdown files for this folder
        let mut page_contents = Vec::new();
        for page_name in folder_pages {
            let page_file_path = folder_path.join(format!("{}.md", page_name));

            if !page_file_path.exists() {
                eprintln!("Page {} not found in folder {}", page_name, folder_name);
                continue;
            }

            match fs::read_to_string(&page_file_path) {
                Ok(content) => page_contents.push((page_name.clone(), content)),
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to read page '{}': {}, using default content",
                        page_name, e
                    );
                    page_contents.push((page_name.clone(), format!("# {}", page_name)));
                }
            }
        }

        // Process all markdown content in batch
        for (page_name, content) in page_contents {

            let mdast : Node = markdown::to_mdast(&content, &markdown::ParseOptions::gfm()).unwrap();

            let headings = read_headings(&mdast);
            println!("Headings for {}: {:?}", page_name, headings);
            
            // Convert markdown to FrankenUi HTML
            let html_content =
                match markdown::to_html_frankenui_with_options(&content, &markdown::Options::gfm())
                {
                    Ok(html) => html,
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to parse markdown for '{}': {}",
                            page_name, e
                        );
                        format!("<h1>{}</h1>", page_name)
                    }
                };

            let page = Page::new(
                page_name.clone(),
                page_name,
                html_content,
                folder_name.clone(),
                headings,
            );
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

fn read_headings(mdast: &Node) -> Vec<Heading> {
    let mut headings = Vec::new();
    visit(&mdast, |node| {
        if let Node::Heading(heading) = node {
            match heading.children.first() {
                Some(text) => {
                    if let Node::Text(text) = text {
                        headings.push(Heading {
                            text: text.value.clone(),
                            level: heading.depth,
                        });
                    } else {
                        eprintln!("Warning: Heading has no text");
                    }
                },
                None => {
                    eprintln!("Warning: Heading has no children text");
                },
            }
        }
    });
    headings
}


// Optimized site generation with better file handling
fn generate_site(folder_path: &str) -> FrankmarkResult<()> {
    let config_path = format!("{}/frankmark.toml", folder_path);
    let config = parse_config(&config_path)?;
    println!("✓ Configuration loaded successfully");

    let source_dir = folder_path;
    let folders = parse_directory(&config, source_dir)?;
    println!("✓ Found {} folders to process", folders.len());

    let output_path = Path::new(folder_path).join("output");

    // Efficient directory cleanup and creation
    if output_path.exists() {
        fs::remove_dir_all(&output_path)?;
    }
    fs::create_dir_all(&output_path)?;

    // Pre-compute navigation for better performance
    let navigator = PageNavigator::new(&folders);

    let mut total_pages = 0;
    let mut first_page: Option<&Page> = None;

    // Generate all pages efficiently
    for folder in &folders {
        let folder_output_path = output_path.join(&folder.name);
        fs::create_dir_all(&folder_output_path)?;

        for page in &folder.pages {
            let page_template = MainTemplate::new(
                &config.book,
                &folders,
                page,
                navigator.get_previous_page(page),
                navigator.get_next_page(page),
                false,
            );

            let rendered = page_template.render()?;
            let file_path = folder_output_path.join(format!("{}.html", page.display_name));

            // Use buffered writing for better performance
            let mut file = File::create(&file_path)?;
            file.write_all(rendered.as_bytes())?;

            println!("✓ Generated {}", file_path.display());
            total_pages += 1;

            if first_page.is_none() {
                first_page = Some(page);
            }
        }
    }

    // Generate index page
    if let Some(first_page) = first_page {
        let page_template = MainTemplate::new(
            &config.book,
            &folders,
            first_page,
            None,
            navigator.get_next_page(first_page),
            true,
        );
        let rendered = page_template.render()?;
        let mut file = File::create(output_path.join("index.html"))?;
        file.write_all(rendered.as_bytes())?;
        println!("✓ Generated index.html");
    }

    println!("✓ Successfully generated {} pages", total_pages);
    Ok(())
}

fn run(folder_path: &str) -> FrankmarkResult<()> {
    generate_site(folder_path)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let folder_path = args.get(1).map(|s| s.as_str()).unwrap_or("demo");

    if let Err(e) = run(folder_path) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}


/// Visit.
fn visit<Visitor>(node: &Node, visitor: Visitor)
where
    Visitor: FnMut(&Node),
{
    visit_impl(node, visitor);
}

/// Internal implementation to visit.
fn visit_impl<Visitor>(node: &Node, mut visitor: Visitor) -> Visitor
where
    Visitor: FnMut(&Node),
{
    visitor(node);

    if let Some(children) = node.children() {
        let mut index = 0;
        while index < children.len() {
            let child = &children[index];
            visitor = visit_impl(child, visitor);
            index += 1;
        }
    }

    visitor
}