#[derive(Debug)]
pub struct Page {
    pub text: Option<String>,
    pub photo_paths: Vec<String>,
}

impl Page {
    pub fn builder() -> PageBuilder {
        PageBuilder::new()
    }
}

pub struct PageBuilder {
    pub text: Option<String>,
    pub photo_paths: Vec<String>,
}

impl PageBuilder {
    pub fn new() -> PageBuilder {
        PageBuilder {
            text: Option::None,
            photo_paths: vec![]
        }
    }

    pub fn text(&mut self, text: Option<String>) -> &mut Self {
        self.text = text;
        self
    }

    pub fn photo_paths(&mut self, photo_paths: Vec<String>) -> &mut Self {
        self.photo_paths = photo_paths;
        self
    }

    pub fn build(&mut self) -> Page {
        Page {
            text: self.text.clone(),
            photo_paths: self.photo_paths.clone()
        }
    }
}