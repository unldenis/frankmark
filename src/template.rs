use crate::config::Book;
use crate::models::{Folder, Page};
use askama::Template;

#[derive(Template)]
#[template(path = "main.html")]
pub struct MainTemplate<'a> {
    pub book: &'a Book,
    pub folders: &'a Vec<Folder>,
    pub current_page: &'a Page,
    pub previous_page: Option<&'a Page>,
    pub next_page: Option<&'a Page>,
}

impl<'a> MainTemplate<'a> {
    pub fn new(
        book: &'a Book,
        folders: &'a Vec<Folder>,
        current_page: &'a Page,
        previous_page: Option<&'a Page>,
        next_page: Option<&'a Page>,
    ) -> Self {
        Self {
            book,
            folders,
            current_page,
            previous_page,
            next_page,
        }
    }

    pub fn get_relative_path_url(&self, page: &Page) -> String {
        // Calculate relative path from current page's directory to target page
        let current_dir = self.current_page.output_path.parent().unwrap();
        let relative_path = pathdiff::diff_paths(&page.output_path, current_dir).unwrap();
        relative_path.to_string_lossy().into_owned()
    }

    pub fn get_first_page_url(&self) -> String {
        if let Some(first_folder) = self.folders.first() {
            if let Some(first_page) = first_folder.pages.first() {
                return self.get_relative_path_url(first_page);
            }
        }
        String::new()
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
