use std::{
    fs::{File, OpenOptions},
    io::Read,
};

use color_eyre::{Result, eyre::Ok};

use crate::app::{App, AppProperties};

#[derive(Debug)]
pub struct TextFile {
    pub inname: String,
    pub wfile: Option<File>,
    pub tmp_name: String,
    pub tmpfile: Option<File>,
    pub diff_name: String,
    pub dfile: Option<File>,
    pub original_text: String,
}

// rw, do nothing with files, cant write anyway
// ow, Overwrite original file with new file, if false then create new file (1 is create new file)
// wb, Write-back enable. If true then write to final file on every save. If false then write to temp file and write to final on exit

impl TextFile {
    pub fn new() -> Self {
        Self {
            inname: String::new(),
            tmpfile: None,
            wfile: None,
            dfile: None,
            diff_name: String::new(),
            tmp_name: String::new(),
            original_text: String::new(),
        }
    }

    pub fn setup(&mut self, prop: &AppProperties) -> Result<()> {
        if self.inname.is_empty() | prop.rw {
            return Ok(());
        }

        if prop.df {
            self.diff_name = self.inname.clone() + ".diff";
            match File::create_new(self.diff_name.clone()) {
                Result::Ok(f) => self.dfile = Some(f),
                Err(e) => return Err(color_eyre::eyre::eyre!("DiffFile already exists: {e}")),
            }
        }

        if !prop.ow {
            match OpenOptions::new().write(true).open(self.inname.clone()) {
                Result::Ok(f) => self.wfile = Some(f),
                Err(e) => return Err(color_eyre::eyre::eyre!("Could not open file: {e}")),
            }

            if !prop.wb {
                self.tmp_name = self.inname.clone() + ".swp";
                match File::create_new(self.tmp_name.clone()) {
                    Result::Ok(f) => self.tmpfile = Some(f),
                    Err(e) => return Err(color_eyre::eyre::eyre!("SwapFile already exists: {e}")),
                }
            }
        }
        Ok(())
    }
}

pub fn parse_conf(input: Vec<String>, app: &mut App) -> Result<()> {
    if input.len() < 2 {
        return Ok(());
    } else if input.len() > 3 {
        return Err(color_eyre::eyre::eyre!(
            "Cannot currently open more than one file"
        ));
    }

    // Args are present
    if input[1].chars().nth(0).unwrap() == '-' {
        let args = input[1].clone();
        let mut chars = args.chars();
        chars.next();
        let args = chars.as_str();
        for c in args.chars() {
            match c {
                'r' => {
                    app.prop.enable_read_only();
                }
                'd' => {
                    app.prop.enable_diff();
                }
                'o' => {
                    app.prop.enable_new_file();
                }
                'w' => {
                    app.prop.enable_write_back();
                }
                e => return Err(color_eyre::eyre::eyre!("Invalid argument: {e}")),
            }
        }
        if input.len() == 3 {
            app.f.inname = input[2].clone();
        }
    } else if input.len() > 2 {
        return Err(color_eyre::eyre::eyre!(
            "Cannot currently open more than one file"
        ));
    } else {
        app.f.inname = input[1].clone();
    }

    Ok(())
}

pub fn load_file(app: &mut App) -> Result<()> {
    let mut file = File::open(app.f.inname.clone())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    app.text.raw_text.pop();
    for l in contents.split('\n') {
        let ln: Vec<char> = l.chars().collect();
        let mut l: String = ln.clone().into_iter().collect();
        l.push('\n');
        app.text.raw_text.push(ln);
        app.f.original_text.push_str(l.as_str());
        app.text.position.max_line_number += 1;
    }
    app.text.position.max_character = app.text.raw_text[0].len();
    app.text.position.max_line_number -= 1;
    Ok(())
}

pub fn store_file() {}
