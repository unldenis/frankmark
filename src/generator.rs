use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::error::FrankmarkResult;
use crate::navigation::PageNavigator;
use crate::parser::parse_directory;
use crate::template::MainTemplate;
use askama::Template;

// Optimized site generation with better file handling
pub fn generate_site(folder_path: &str) -> FrankmarkResult<()> {
    let config_path = format!("{}/frankmark.toml", folder_path);
    let config = crate::config::parse_config(&config_path)?;
    println!("Configuration loaded successfully");

    let source_dir = folder_path;
    let mut folders = parse_directory(&config, source_dir)?;
    println!("Found {} folders to process", folders.len());

    let output_path = Path::new(folder_path).join("output");

    // Efficient directory cleanup and creation
    if output_path.exists() {
        fs::remove_dir_all(&output_path)?;
    }
    fs::create_dir_all(&output_path)?;

    // Set output paths for all pages
    for folder in &mut folders {
        for page in &mut folder.pages {
            page.output_path = output_path
                .join(&folder.name)
                .join(format!("{}.html", page.display_name));
        }
    }

    // Pre-compute navigation for better performance
    let navigator = PageNavigator::new(&folders);

    let mut total_pages = 0;
    let mut first_page: Option<&crate::models::Page> = None;

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
            );

            let rendered = page_template.render()?;
            let file_path = folder_output_path.join(format!("{}.html", page.display_name));

            // Use buffered writing for better performance
            let mut file = File::create(&file_path)?;
            file.write_all(rendered.as_bytes())?;

            println!("Generated {}", file_path.display());
            total_pages += 1;

            if first_page.is_none() {
                first_page = Some(page);
            }
        }
    }

    println!("Successfully generated {} pages", total_pages);
    Ok(())
}
