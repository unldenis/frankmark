use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use markdown::mdast::Node;

use crate::config::Config;
use crate::error::FrankmarkResult;
use crate::models::{Folder, Heading, Page};

// Optimized directory parsing with batch operations
pub fn parse_directory(config: &Config, config_folder_path: &str) -> FrankmarkResult<Vec<Folder>> {
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
            let mdast: Node = markdown::to_mdast(&content, &markdown::ParseOptions::gfm()).unwrap();

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
                PathBuf::new(), // Will be set later in generate_site
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

pub fn read_headings(mdast: &Node) -> Vec<Heading> {
    let mut headings = Vec::new();
    visit(&mdast, |node| {
        if let Node::Heading(heading) = node {
            match heading.children.first() {
                Some(text) => {
                    if let Node::Text(text) = text {
                        headings.push(Heading {
                            text: text.value.clone(),
                            level: heading.depth,
                            id: slug::slugify(&text.value),
                        });
                    } else {
                        eprintln!("Warning: Heading has no text");
                    }
                }
                None => {
                    eprintln!("Warning: Heading has no children text");
                }
            }
        }
    });
    headings
}

/// Visit.
pub fn visit<Visitor>(node: &Node, visitor: Visitor)
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
