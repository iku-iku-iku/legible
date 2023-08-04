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

#[derive(Getters)]
pub struct InputArg {
    #[getset(get = "pub")]
    name: String,
}

impl InputArg {
    pub fn new(name: &str) -> Self {
        InputArg {
            name: name.to_string(),
        }
    }
}

impl Debug for InputArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}`", self.name)
    }
}

#[derive(Getters)]
pub struct Block {
    #[getset(get = "pub")]
    title: Box<Title>,
    #[getset(get = "pub")]
    input: Vec<Box<InputArg>>,
    // output: Vec<Statement>,
}

impl Block {
    pub fn new(title: Box<Title>) -> Self {
        Block {
            title,
            input: Vec::new(),
            // output: Vec::new(),
        }
    }

    pub fn new_with_inputs(title: Box<Title>, input: Vec<Box<InputArg>>) -> Self {
        Block {
            title,
            input,
            // output: Vec::new(),
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("{:?}", self.title);
        for (idx, x) in self.input.iter().enumerate() {
            s.push_str(&format!("\n{}. {:?}", idx + 1, x));
        }
        write!(f, "{}", s)
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

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Compound(Box<Expr>, Operator, Box<Expr>)
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Getters)]
pub struct Template {
    #[getset(get = "pub")]
    words: Vec<String>,
    params: Vec<(String, u32)>
}

impl Template {
    pub fn new(template_str: &str) -> Self {
        Template {
            words: vec![],
            params: vec![]
        }
    }
}