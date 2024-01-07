use humansize::WINDOWS;

#[derive(Debug)]
pub struct Document {
    pub name: String,
    pub path: String,
    size: usize,
    pub enabled: bool
}

impl Document {
    pub fn new(name: String, path: String, size: usize) -> Self {
        Document{name, path, size, enabled: true}
    }

    pub fn enabled_size(&self) -> usize {
        if self.enabled {
            self.size
        } else {
            0
        }
    }

    pub fn human_readable_size(&self) -> String {
        humansize::format_size(self.size, WINDOWS)
    }
}

#[derive(Debug)]
pub struct Category {
    pub name: String,
    pub documents: Vec<Document>,
    pub enabled: bool,
}

impl Category {
    pub fn new(name: String, documents: Vec<Document>) -> Self {
        Category{name, documents, enabled: true}
    }

    pub fn size(&self, enabled_only: bool) -> usize {
        let mut total = 0;
        for doc in &self.documents {
            if enabled_only {
                total += doc.enabled_size();
            } else {
                total += doc.size;
            }
        }
        total
    }

    pub fn enabled_size(&self) -> usize {
        if self.enabled {
            self.size(true)
        } else {
            0
        }
    }

    pub fn human_readable_size(&self, enabled_only: bool) -> String {
        humansize::format_size(self.size(enabled_only), WINDOWS)
    }
}
