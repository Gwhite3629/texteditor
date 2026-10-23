use std::{fs::File, io::Read};

use color_eyre::{Result, eyre::Ok};

use crate::app::App;

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
            app.fname = input[2].clone();
        }
    } else if input.len() > 2 {
        return Err(color_eyre::eyre::eyre!(
            "Cannot currently open more than one file"
        ));
    } else {
        app.fname = input[1].clone();
    }

    Ok(())
}

pub fn load_file(app: &mut App) -> Result<()> {
    let mut file = File::open(app.fname.clone())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    app.text.raw_text.pop();
    for l in contents.split('\n') {
        let ln = l.chars().collect();
        app.text.raw_text.push(ln);
        app.text.position.max_line_number += 1;
    }
    app.text.position.max_character = app.text.raw_text[0].len();
    app.text.position.max_line_number -= 1;
    Ok(())
}

pub fn store_file() {}
