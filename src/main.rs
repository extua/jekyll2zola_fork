use std::fs;
use std::error::Error;

fn readfile(file_path: &str) -> Result<String, Box<dyn Error>> {
    println!("file path {}", file_path);
    let filestring: String = fs::read_to_string(file_path)?;
        println!("file string read");
        Ok(filestring)
    }

fn main() {
    let file_path: &str = "tests/2024-11-20-jekyll_test_page.md";
    let file_to_string: String = match readfile(file_path) {
        Ok(v) => v,
        Err(e) => panic!("file not found at {file_path}, error returned: {e}"),
      };
    print!("{}",file_to_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_the_file_to_string() {
        let file_path: &str = "tests/2024-11-20-jekyll_test_page.md";
        let file_to_string = readfile(file_path).expect("unable to read file");
        let test_file_string: String = "---
layout: page
date: 2024-11-20
title: Jekyll test page
subtitle: This is a subtitle
author: test author
location: Oxford
tags: [test post, jekyll, zola, rust]
---

This is a test jekyll page".to_string();
        assert_eq!(file_to_string, test_file_string);
    }
}