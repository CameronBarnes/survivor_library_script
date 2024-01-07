
use crate::types::Category;

use super::ui::StatefulListCounter;

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub categories: Vec<Category>,
    pub category_list: StatefulListCounter,
    pub doc_list: StatefulListCounter,
    pub left_is_true: bool,
}

impl App {
    pub fn new(categories: Vec<Category>) -> Self {
        let category_list = StatefulListCounter::new(categories.len());
        let doc_list = StatefulListCounter::new(categories[0].documents.len());
        
        App{should_quit: false, categories, category_list, doc_list, left_is_true: true}
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

    pub fn left(&mut self) {
        self.left_is_true = true;
    }

    pub fn right(&mut self) {
        self.left_is_true = false;
    }

    fn reset_doc_list(&mut self, index: usize) {
        self.doc_list = StatefulListCounter::new(self.categories[index].documents.len());
        self.doc_list.set_selected(0);
    }

    #[allow(unused)]
    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    
}
