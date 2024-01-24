use std::{fs, path::Path};

use anyhow::{Ok, Result};

pub struct MdParser;

impl MdParser {
    pub fn generate<T>(source: &T) -> Result<String>
    where
        T: AsRef<Path> 
    {
        let content = fs::read_to_string(source)?;
        let html = markdown::to_html(content.as_str());

        Ok(html)
    }
}