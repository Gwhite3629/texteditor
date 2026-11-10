use std::{
    fs::File,
    io::{Seek, Write},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, ModifierKeyCode};
use diffy::create_patch;
use ratatui::{DefaultTerminal, Frame, text::Line};

use color_eyre::{Result, eyre::WrapErr};

use crate::app::{
    AppProperties,
    AppState::{self, MOVE, MULTICOMMAND, SAVING, SELECT, SIMPLECOMMAND, TEXT},
    DynamicText,
    file::TextFile,
    highlight, insert, load_file, parse_conf,
    text::Coordinate,
    ui::keycode_to_text,
};

#[derive(Debug)]
pub struct App {
    pub status_text: Vec<Line<'static>>, // Placeholder for text object
    pub cmdbuf: Vec<KeyCode>,            // Current multi-key command buffer
    pub text: DynamicText,
    state: AppState,         // Current mutable app state
    pub prop: AppProperties, // Immutable session properties
    pub f: TextFile,
    exit: bool, // Is-exiting flag
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, cmd: Vec<String>) -> Result<()> {
        // Initial setup.
        parse_conf(cmd, self)?;
        if !self.f.inname.is_empty() {
            load_file(self)?;
        }
        self.f.setup(&self.prop)?;

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.set_cursor_position(self.text.pos() + Coordinate::from((1 as u16, 1 as u16)));
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Esc => {
                highlight()?;
                self.text.null_highlight();
                self.cmdbuf.clear();
                self.status_text.clear();
                self.status_text.push(Line::from("Command"));
                self.state = Default::default();
            }
            KeyCode::Char('h') => match self.state {
                SIMPLECOMMAND => self.text.move_left(), // Move cursor left
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('h')), // Append 'h' to multi-command
                SAVING => self.cmdbuf.push(KeyCode::Char('h')),
                SELECT => self.text.select_left(), // Select to the left
                MOVE => {}                         // Move selection to the left
                TEXT => self.text.push('h'),       // Append 'h' to text
            },
            KeyCode::Left => match self.state {
                SIMPLECOMMAND => self.text.move_left(), // Move cursor left
                MULTICOMMAND => {}                      // Append left arrow to command
                SAVING => {}
                SELECT => self.text.select_left(), // Select to the left
                MOVE => {}                         // Move selection to the left
                TEXT => self.text.move_left(),     // Move cursor left
            },
            KeyCode::Char('j') => match self.state {
                SIMPLECOMMAND => self.text.move_down(), // Move cursor down
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('j')), // Append 'j' to multi-command
                SAVING => self.cmdbuf.push(KeyCode::Char('j')),
                SELECT => self.text.select_down(), // Select to the down
                MOVE => {}                         // Move selection to the down
                TEXT => self.text.push('j'),       // Append 'j' to text
            },
            KeyCode::Down => match self.state {
                SIMPLECOMMAND => self.text.move_down(), // Move cursor down
                MULTICOMMAND => {}                      // Append down arrow to command
                SAVING => {}
                SELECT => self.text.select_down(), // Select to the down
                MOVE => {}                         // Move selection to the down
                TEXT => self.text.move_down(),     // Move cursor down
            }, // Move cursor down
            KeyCode::Char('k') => match self.state {
                SIMPLECOMMAND => self.text.move_up(), // Move cursor up
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('k')), // Append 'k' to multi-command
                SAVING => self.cmdbuf.push(KeyCode::Char('k')),
                SELECT => self.text.select_up(), // Select to the up
                MOVE => {}                       // Move selection to the up
                TEXT => self.text.push('k'),     // Append 'k' to text
            },
            KeyCode::Up => match self.state {
                SIMPLECOMMAND => self.text.move_up(), // Move cursor up
                MULTICOMMAND => {}                    // Append up arrow to command
                SAVING => {}
                SELECT => self.text.select_up(), // Select to the up
                MOVE => {}                       // Move selection to the up
                TEXT => self.text.move_up(),     // Move cursor up
            }, // Move cursor up
            KeyCode::Char('l') => match self.state {
                SIMPLECOMMAND => self.text.move_right(), // Move cursor right
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('l')), // Append 'l' to multi-command
                SAVING => self.cmdbuf.push(KeyCode::Char('l')),
                SELECT => self.text.select_right(), // Select to the right
                MOVE => {}                          // Move selection to the right
                TEXT => self.text.push('l'),        // Append 'l' to text
            },
            KeyCode::Right => match self.state {
                SIMPLECOMMAND => self.text.move_right(), // Move cursor right
                MULTICOMMAND => {}                       // Append right arrow to command
                SAVING => {}
                SELECT => self.text.select_right(), // Select to the right
                MOVE => {}                          // Move selection to the right
                TEXT => self.text.move_right(),     // Move cursor right
            }, // Move cursor right
            KeyCode::Char('m') => match self.state {
                SIMPLECOMMAND => {
                    self.cmdbuf.clear();
                    self.status_text.clear();
                    self.status_text.push(Line::from("MultiCommand"));
                    self.state = MULTICOMMAND;
                } // Go to multi-command mode
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('m')), // Append 'm' to multi-command
                SAVING => self.cmdbuf.push(KeyCode::Char('m')),
                TEXT => self.text.push('m'), // Append 'm' to text
                _ => {
                    highlight()?;
                    self.status_text.clear();
                    self.status_text.push(Line::from("Move"));
                    self.state = MOVE;
                } // Go to move mode
            },
            KeyCode::Char('v') => match self.state {
                SIMPLECOMMAND => {
                    highlight()?;
                    self.text.null_highlight();
                    self.status_text.clear();
                    self.status_text.push(Line::from("Select"));
                    self.state = SELECT
                }
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('v')), // Append 'v' to multi-command
                SAVING => self.cmdbuf.push(KeyCode::Char('v')),
                TEXT => self.text.push('v'), // Append 'v' to text
                _ => {
                    highlight()?;
                    self.status_text.clear();
                    self.status_text.push(Line::from("Select"));
                    self.state = SELECT
                } // Enter select mode
            },
            KeyCode::Char('e') => {
                if self.prop.rw {
                    self.status_text.clear();
                    self.status_text.push(Line::from("Cannot edit: READONLY"))
                } else {
                    match self.state {
                        MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('e')), // Append 'e' to multi-command
                        SAVING => self.cmdbuf.push(KeyCode::Char('e')),
                        TEXT => self.text.push('e'), // Append 'e' to text
                        _ => {
                            insert()?;
                            self.status_text.clear();
                            self.status_text.push(Line::from("Text"));
                            self.state = TEXT
                        }
                    }
                }
            }
            KeyCode::Char('q') => match self.state {
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('q')), // Append 'q' to multi-command
                SAVING => self.cmdbuf.push(KeyCode::Char('q')),
                TEXT => self.text.push('q'), // Append 'q' to text
                _ => self.exit()?,
            },
            KeyCode::Char('s') => match self.state {
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('s')), // Append 's' to multi-command
                SAVING => self.cmdbuf.push(KeyCode::Char('s')),
                TEXT => self.text.push('s'), // Append 's' to text
                _ => self.save()?,
            },
            KeyCode::Char('z') => match self.state {
                SIMPLECOMMAND => {}                                   // Undo latest command
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('z')), // Append 'z' to multi-command
                SAVING => self.cmdbuf.push(KeyCode::Char('z')),
                SELECT => {}                 // Undo latest select
                MOVE => {}                   // Undo latest move
                TEXT => self.text.push('z'), // Append 'z' to text
            },
            KeyCode::Modifier(modifier) => match modifier {
                ModifierKeyCode::LeftControl => match self.state {
                    _ => {
                        self.cmdbuf.clear();
                        self.status_text.clear();
                        self.status_text.push(Line::from("MultiCommand"));
                        self.state = MULTICOMMAND;
                    } // Go to multi-command
                },
                _ => {}
            },
            KeyCode::Enter => match self.state {
                SIMPLECOMMAND => {} // Move cursor down newline
                SAVING => self.save()?,
                MULTICOMMAND => {
                    self.state = Default::default();
                    self.status_text.clear();
                    self.status_text.push(Line::from("Command"));
                    self.handle_command()?
                }
                SELECT => {}                  // Do nothing
                MOVE => {}                    // Move selection down a newline
                TEXT => self.text.push('\n'), // Enter newline character
            },
            KeyCode::Char(' ') => match self.state {
                TEXT => self.text.push(' '), // Append ' ' to text
                _ => {}
            },
            KeyCode::Backspace => match self.state {
                MULTICOMMAND => {
                    if !self.cmdbuf.is_empty() {
                        self.cmdbuf.pop();
                    }
                } // Remove previous typed character
                SAVING => {
                    if !self.cmdbuf.is_empty() {
                        self.cmdbuf.pop();
                    }
                }
                TEXT => {
                    self.text.pop();
                } // Remove previous character
                _ => {} // Do nothing
            },
            k => match self.state {
                MULTICOMMAND => self.cmdbuf.push(k), // Append k to command buf
                SAVING => self.cmdbuf.push(k),
                TEXT => self.text.push(k.to_string().pop().unwrap()), // Append k to text
                _ => {}                                               // Do nothing
            },
        }

        Ok(())
    }

    fn handle_command(&mut self) -> Result<()> {
        self.cmdbuf.clear();
        Ok(())
    }

    // ow, Overwrite original file with new file, if false then create new file (1 is create new file)
    // wb, Write-back enable. If true then write to final file on every save. If false then write to temp file and write to final on exit
    fn save(&mut self) -> Result<()> {
        let t = self.text.to_bytes();

        let wfile_name: String;

        if self.f.wfile.is_none() {
            if self.state == SAVING {
                wfile_name = self
                    .cmdbuf
                    .iter()
                    .map(|k| keycode_to_text(k).pop().unwrap())
                    .collect();
                match File::create_new(wfile_name.clone()) {
                    Result::Ok(f) => self.f.wfile = Some(f),
                    Err(e) => return Err(color_eyre::eyre::eyre!("File already exists: {e}")),
                };
                if !self.prop.wb {
                    self.f.tmp_name = wfile_name.clone() + ".swp";
                    match File::create_new(self.f.tmp_name.clone()) {
                        Result::Ok(f) => self.f.tmpfile = Some(f),
                        Err(e) => {
                            return Err(color_eyre::eyre::eyre!("SwapFile already exists: {e}"));
                        }
                    }
                }
                if self.prop.df & self.f.dfile.is_none() {
                    // Write diff to file
                    self.f.diff_name = wfile_name + ".swp";
                    match File::create_new(self.f.diff_name.clone()) {
                        Result::Ok(f) => self.f.dfile = Some(f),
                        Err(e) => {
                            return Err(color_eyre::eyre::eyre!("DiffFile already exists: {e}"));
                        }
                    }
                }
                self.cmdbuf.clear();
                self.status_text.clear();
                self.status_text.push(Line::from("Command"));
                self.state = SIMPLECOMMAND;
            } else {
                self.cmdbuf.clear();
                self.status_text.clear();
                self.status_text.push(Line::from("Enter filename: "));
                self.state = SAVING;
                return Ok(());
            }
        }

        if self.prop.wb {
            self.f.wfile.as_ref().unwrap().rewind()?;
            self.f.wfile.as_ref().unwrap().set_len(0)?;
            match self.f.wfile.as_ref().unwrap().write_all(&t.to_vec()) {
                Result::Ok(_) => {}
                Err(e) => return Err(color_eyre::eyre::eyre!("Could not write to file: {e}")),
            }
        } else {
            self.f.tmpfile.as_ref().unwrap().rewind()?;
            self.f.tmpfile.as_ref().unwrap().set_len(0)?;
            match self.f.tmpfile.as_ref().unwrap().write_all(&t.to_vec()) {
                Result::Ok(_) => {}
                Err(e) => return Err(color_eyre::eyre::eyre!("Could not write to file: {e}")),
            }
        }

        if self.prop.df {
            // Write diff to file
            let text = self.text.to_string();
            let patch = create_patch(&self.f.original_text, &text);
            self.f.dfile.as_ref().unwrap().rewind()?;
            self.f.dfile.as_ref().unwrap().set_len(0)?;
            match self
                .f
                .dfile
                .as_ref()
                .unwrap()
                .write_all(&patch.to_string().as_bytes())
            {
                Result::Ok(_) => {}
                Err(e) => return Err(color_eyre::eyre::eyre!("Could not write to file: {e}")),
            }
        }

        self.status_text.push(Line::from("Saving"));
        Ok(())
    }

    fn exit(&mut self) -> Result<()> {
        self.exit = true;
        if self.f.wfile.is_none() {
            self.save()?;
        }
        if !self.prop.wb {
            let t = self.text.to_bytes();
            self.f.wfile.as_ref().unwrap().rewind()?;
            self.f.wfile.as_ref().unwrap().set_len(0)?;
            match self.f.wfile.as_ref().unwrap().write_all(&t.to_vec()) {
                Result::Ok(_) => {}
                Err(e) => return Err(color_eyre::eyre::eyre!("Could not write to file: {e}")),
            }
            std::fs::remove_file(self.f.tmp_name.clone())?;
        }
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            cmdbuf: Vec::default(),
            status_text: vec![Line::from("Command")], // Your custom default values
            text: DynamicText::default(),
            state: AppState::default(),
            prop: AppProperties::default(),
            f: TextFile::new(),
            exit: false,
        }
    }
}
