use crate::models::{Folder, Page};
use std::collections::HashMap;

// Optimized navigation with pre-computed page order
pub struct PageNavigator<'a> {
    all_pages: Vec<&'a Page>,
    page_to_index: HashMap<&'a str, usize>,
}

impl<'a> PageNavigator<'a> {
    pub fn new(folders: &'a [Folder]) -> Self {
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

    pub fn get_next_page(&self, current_page: &Page) -> Option<&'a Page> {
        let current_index = self.page_to_index.get(current_page.id.as_str())?;
        let next_index = current_index + 1;
        if next_index < self.all_pages.len() {
            Some(self.all_pages[next_index])
        } else {
            None
        }
    }

    pub fn get_previous_page(&self, current_page: &Page) -> Option<&'a Page> {
        let current_index = self.page_to_index.get(current_page.id.as_str())?;
        if *current_index > 0 {
            Some(self.all_pages[current_index - 1])
        } else {
            None
        }
    }
}
