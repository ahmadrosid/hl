pub struct StringBuilder {
    source: String
}

impl StringBuilder {
    pub fn new() -> Self {
        Self {
            source: String::new()
        }
    }

    fn endl(&mut self) {
        self.source.push('\n');
    }

    pub fn push(&mut self, s: char) {
        self.source.push(s);
    }

    pub fn push_str(&mut self, s: &str) {
        self.source.push_str(s);
    }

    pub fn push_tab(&mut self, tab: usize, s: &str) {
        for i in 0..tab {
            self.push('\t');
        }
        self.push_str(s);
    }

    pub fn push_strln(&mut self, s: &str) {
        self.push_str(s);
        self.endl();
    }

    pub fn push_tabln(&mut self, tab: usize, s: &str) {
        self.push_tab(tab, s,);
        self.endl();
    }

    pub fn to_string(self) -> String {
        self.source
    }
}
