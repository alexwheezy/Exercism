use anyhow::Error;
use std::{collections::HashSet, fs::File, io::Read};

#[derive(Debug)]
pub struct Flags<'a> {
    args: HashSet<&'a str>,
}

impl<'a> Flags<'a> {
    pub fn new(flags: &[&'a str]) -> Self {
        Self {
            args: flags.iter().copied().collect(),
        }
    }

    pub fn is_line_numbers(&self) -> bool {
        self.args.contains("-n")
    }

    pub fn is_list_files_only(&self) -> bool {
        self.args.contains("-l")
    }

    pub fn is_case_sensitive(&self) -> bool {
        self.args.contains("-i")
    }

    pub fn is_invert_match(&self) -> bool {
        self.args.contains("-v")
    }

    pub fn is_entire_line(&self) -> bool {
        self.args.contains("-x")
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let pattern_normalized = if flags.is_case_sensitive() {
        pattern.to_lowercase()
    } else {
        pattern.to_owned()
    };

    let mut results = vec![];
    let mut contents = String::with_capacity(128);

    for &file in files {
        File::open(file)?.read_to_string(&mut contents)?;

        for (line, content) in contents.lines().enumerate() {
            let content_normalized = if flags.is_case_sensitive() {
                content.to_lowercase()
            } else {
                content.to_owned()
            };

            let matched = if flags.is_entire_line() {
                content_normalized == pattern_normalized
            } else {
                content_normalized.contains(&pattern_normalized)
            };

            let matched = if flags.is_invert_match() {
                !matched
            } else {
                matched
            };

            if matched {
                if flags.is_list_files_only() {
                    results.push(file.to_owned());
                    break;
                }

                let mut output = String::new();

                if files.len() > 1 {
                    output.push_str(&format!("{file}:"));
                }

                if flags.is_line_numbers() {
                    output.push_str(&format!("{}:", line + 1));
                }

                output.push_str(content);
                results.push(output);
            }
        }
        contents.clear();
    }
    Ok(results)
}
