use std::fmt::Display;

#[derive(PartialEq, Hash, Clone)]
pub struct Acronym {
    components: Vec<String>,
}

impl Acronym {
    pub fn new() -> Self {
        Self { components: vec![] }
    }

    pub fn from(components: Vec<String>) -> Self {
        Self { components }
    }

    pub fn generate_acronym(&self) -> String {
        self.components.iter().fold(String::new(), |acc, x| {
            acc + x
                .chars()
                .next()
                .unwrap()
                .to_uppercase()
                .to_string()
                .as_str()
        })
    }

    pub fn get_components_string(&self) -> String {
        self.components.join(" ")
    }

    pub fn get_components(&self) -> &Vec<String> {
        &self.components
    }
}

impl Default for Acronym {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Acronym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.generate_acronym(),
            self.components
                .iter()
                .fold(String::new(), |acc, x| acc + " " + x.as_str())
        )
    }
}

impl Eq for Acronym {}
