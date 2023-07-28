use std::fmt::Debug;

use getset::Getters;

#[derive(Getters)]
pub struct Title {
    #[getset(get = "pub")]
    content: String,
    #[getset(get = "pub")]
    level: u32,
}

impl Title {
    pub fn new(title_content: &str, level: u32) -> Self {
        Title {
            content: title_content.to_string(),
            level
        }
    }
}

impl Debug for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for _ in 0..self.level {
            s.push('#');
        }
        s.push(' ');
        s.push_str(&self.content);
        write!(f, "{}", s)
    }
}

#[derive(Debug, Getters)]
pub struct Block {
    #[getset(get = "pub")]
    title: Box<Title>
    // input: Vec<Statement>,
    // output: Vec<Statement>,
}

impl Block {
    pub fn new(title: Box<Title>) -> Self {
        Block {
            title
            // input: Vec::new(),
            // output: Vec::new(),
        }
    }
}

#[derive(Debug, Getters)]
pub struct Comment {
    #[getset(get = "pub")]
    content: String,
}

impl Comment {
    pub fn new(content: &str) -> Self {
        Comment {
            content: content.to_string(),
        }
    }
}