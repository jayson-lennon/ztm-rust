use std::collections::HashMap;

#[derive(Debug)]
struct ConfigSection<'a> {
    name: &'a str,
    data: HashMap<&'a str, String>,
}

impl<'a> ConfigSection<'a> {
    pub fn insert(&mut self, key: &'a str, value: String) {
        self.data.insert(key, value);
    }
}

#[derive(Debug)]
struct Configuration<'a> {
    sections: HashMap<&'a str, ConfigSection<'a>>,
}

impl<'a> Configuration<'a> {
    pub fn new() -> Self {
        Self {
            sections: HashMap::new(),
        }
    }

    pub fn get_section(&self, name: &str) -> Option<&ConfigSection<'a>> {
        self.sections.get(name)
    }

    pub fn add_section(&mut self, name: &'a str) {
        let section = ConfigSection {
            name,
            data: HashMap::new(),
        };
        self.sections.insert(name, section);
    }

    pub fn insert(&mut self, section_name: &'a str, key: &'a str, value: String) {
        let section = self.sections.entry(section_name).or_insert(ConfigSection {
            name: section_name,
            data: HashMap::new(),
        });
        section.insert(key, value);
    }
}

fn add(lhs: usize, rhs: usize) -> usize {
    lhs + rhs
}

fn main() {
    let mut config = Configuration::new();
    // [section]
    // key=value;
    // key2=value2;
    // [next_section]
    // a=b;
}
