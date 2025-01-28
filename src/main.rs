use std::collections::HashMap;
use std::env::args;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::fmt::Write;
use std::str::FromStr;
use std::convert::{TryInto,TryFrom};
use std:: path::Path;

use toml::{Value as Toml, value::Datetime};
use serde::{Serialize, Deserialize};
use serde_yaml::Value as Yaml;

#[derive(Copy, Clone)]
enum ParserState {
    Unknown,
    FrontMatter,
    Content,
}

struct Parser<'a> {
    file_path: &'a Path,
    state: ParserState,
    raw_front: String,
    front: Option<JekyllFront>,
    content: String,
}

#[derive(Debug)]
struct JekyllDoc {
    front: JekyllFront,
    content: String,
}

impl JekyllDoc {
    fn open_file<P: AsRef<Path>>(file_path: P) -> Option<Self> {
            Parser::new(&file_path).read().unwrap().into_jekyll()
    }
}

#[derive(Debug, Deserialize)]
struct JekyllFront {
    title: String,
    date: String,
    subtitle: String,
    author: String,
    #[serde(flatten)]
    extra: Option<HashMap<String, Toml>>,
}

#[derive(Debug, Serialize)]
struct ZolaFront {
    title: String,
    date: Datetime,
    description: String,
    author: String,
    #[serde(flatten)]
    extra: Option<HashMap<String, Toml>>,
}

impl TryInto<ZolaFront> for JekyllFront {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> Result<ZolaFront, Self::Error> {
        Ok(ZolaFront {
            title: self.title,
            date: Datetime::from_str(&self.date)?,
            description: self.subtitle,
            author: self.author,
            extra: self.extra,

        })
    }
}
impl TryInto<ZolaDoc> for JekyllDoc {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> Result<ZolaDoc, Self::Error> {
        Ok(ZolaDoc {
            front: self.front.try_into()?,
            content: self.content,
        })
    }
}

#[derive(Debug)]
struct ZolaDoc {
    front: ZolaFront,
    content: String,
}

impl TryInto<String> for ZolaDoc {
    type Error = Box<dyn std::error::Error>;
    fn try_into(self) -> Result<String, Self::Error> {
        Ok(format!("+++\n{}\n+++\n{}", toml::to_string(&self.front)?, self.content))
    }
}

impl<'a> Parser<'a> {
    fn new<P: AsRef<Path>>(file_path: &'a P) -> Self {
        Self {
            file_path:
            file_path.as_ref(),
            state: ParserState::Unknown,
            front: None,
            raw_front: String::new(),
            content: String::new(),
        }
    }

    fn read(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        use ParserState::*;

        let input = File::open(self.file_path)?;
        let file = BufReader::new(input);


        for line in file.lines() {
            match (self.state, line?.as_ref()) {
                (Unknown, "---") => self.state = FrontMatter ,
                (FrontMatter, "---") => self.state = Content ,
                (FrontMatter, line_content) => writeln!(&mut self.raw_front, "{}", line_content)?,
                (Content, line_content) => writeln!(&mut self.content, "{}", line_content)?,
                _ => {}
            }
        }

        self.front = serde_yaml::from_str(&self.raw_front)?;

        Ok(self)
    }

    fn into_jekyll(self) -> Option<JekyllDoc> {
        if let Some(front) = self.front {
            Some(JekyllDoc{front, content: self.content})
        } else {
            None
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    if let Some(file_path) = args().nth(1) {
        if let Some(jekyll) = JekyllDoc::open_file(&file_path) {
            let zola: ZolaDoc = jekyll.try_into()?;
            let doc: String = zola.try_into()?;
            println!("{}", doc);
        }
    } else {
        eprintln!("no input")
    }

    Ok(())
}
