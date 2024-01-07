
use std::cmp::Reverse;

use crate::types::Category;

use super::ui::StatefulListCounter;

#[derive(Debug)]
enum SortStyle {
    Alphabetical,
    Size,
}

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub categories: Vec<Category>,
    pub category_list: StatefulListCounter,
    pub doc_list: StatefulListCounter,
    pub left_is_true: bool,
    pub download: bool,
    pub print: bool,
    sort_style: SortStyle,
}

impl App {
    pub fn new(mut categories: Vec<Category>) -> Self {
        categories.sort_unstable_by_key(|cat| cat.name.clone());
        categories[0].documents.sort_unstable_by_key(|doc| doc.name.clone());
        let mut category_list = StatefulListCounter::new(categories.len());
        category_list.set_selected(0);
        let mut doc_list = StatefulListCounter::new(categories[0].documents.len());
        doc_list.set_selected(0);
        
        App{
            should_quit: false,
            categories,
            category_list,
            doc_list,
            left_is_true: true,
            download: false,
            print: false,
            sort_style: SortStyle::Alphabetical
        }
    }

    fn sort_docs(&mut self) {
        let index = self.category_list.selected();
        match self.sort_style {
            SortStyle::Alphabetical => {
                self.categories[index].documents.sort_unstable_by_key(|doc| doc.name.clone());
            },
            SortStyle::Size => {
                self.categories[index].documents.sort_unstable_by_key(|doc| Reverse(doc.size));
            },
        }
    }

    fn sort_categories(&mut self) {
        match self.sort_style {
            SortStyle::Alphabetical => {
                self.categories.sort_unstable_by_key(|cat| cat.name.clone());
            },
            SortStyle::Size => {
                self.categories.sort_unstable_by_key(|cat| Reverse(cat.enabled_size()));
            },
        }
    }

    pub fn next(&mut self) {
        if self.left_is_true {
            self.category_list.next();
            let selected = self.category_list.selected();
            self.reset_doc_list(selected);
        } else {
            self.doc_list.next();
        }
    }

    pub fn previous(&mut self) {
        if self.left_is_true {
            self.category_list.previous();
            let selected = self.category_list.selected();
            self.reset_doc_list(selected);
        } else {
            self.doc_list.previous()
        }
    }

    pub fn home(&mut self) {
        if self.left_is_true {
            self.category_list.set_selected(0);
            self.reset_doc_list(0);
        } else {
            self.doc_list.set_selected(0);
        }
    }

    pub fn end(&mut self) {
        if self.left_is_true {
            let last = self.categories.len() - 1;
            self.category_list.set_selected(last);
            self.reset_doc_list(last);
        } else {
            let index = self.category_list.selected();
            let last = self.categories[index].documents.len() - 1;
            self.doc_list.set_selected(last);
        }
    }

    pub fn toggle(&mut self) {
        if self.left_is_true {
            let index = self.category_list.selected();
            self.categories[index].enabled = !self.categories[index].enabled;
        } else {
            let doc_index = self.doc_list.selected();
            let cat_index = self.category_list.selected();
            self.categories[cat_index].documents[doc_index].enabled = 
                !self.categories[cat_index].documents[doc_index].enabled;
            
        }
    }

    pub fn toggle_all(&mut self) {
        if self.left_is_true {
            for category in &mut self.categories {
                category.enabled = !category.enabled;
            }
        } else {
            let cat_index = self.category_list.selected();
            for doc in &mut self.categories[cat_index].documents {
                doc.enabled = !doc.enabled;
            }
        }
    }

    pub fn reset(&mut self) {
        if self.left_is_true {
            for category in &mut self.categories {
                category.enabled = true;
            }
        } else {
            let cat_index = self.category_list.selected();
            for doc in &mut self.categories[cat_index].documents {
                doc.enabled = true;
            }
        }
    }

    pub fn left(&mut self) {
        self.left_is_true = true;
    }

    pub fn right(&mut self) {
        self.left_is_true = false;
    }

    fn reset_doc_list(&mut self, index: usize) {
        self.doc_list = StatefulListCounter::new(self.categories[index].documents.len());
        self.doc_list.set_selected(0);
        self.sort_docs();
    }

    pub fn toggle_sort_style(&mut self) {
        match self.sort_style {
            SortStyle::Alphabetical => self.sort_style = SortStyle::Size,
            SortStyle::Size => self.sort_style = SortStyle::Alphabetical,
        }
        self.sort_categories();
        self.sort_docs();
    }

    #[allow(unused)]
    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    
}
