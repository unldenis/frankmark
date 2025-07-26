use crate::utils;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Folder {
    pub name: String,
    pub pages: Vec<Page>,
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

#[derive(Debug)]
pub struct Page {
    pub output_path: PathBuf, // Path to the rendered HTML file
    pub id: String,
    pub display_name: String,
    pub content: String,
    pub folder_name: String, // Direct reference to folder name
    pub headings: Vec<Heading>,
}

#[derive(Debug)]
pub struct Heading {
    pub text: String,
    pub level: u8,
    pub id: String,
}

impl Page {
    pub fn new(
        output_path: PathBuf,
        full_name: String,
        display_name: String,
        content: String,
        folder_name: String,
        headings: Vec<Heading>,
    ) -> Self {
        // Use deterministic ID based on content hash for better performance
        let id = utils::generate_deterministic_id(&full_name);
        Self {
            output_path,
            id,
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
