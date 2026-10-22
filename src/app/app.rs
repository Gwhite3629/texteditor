use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, ModifierKeyCode};
use ratatui::{DefaultTerminal, Frame, text::Line};

use color_eyre::{Result, eyre::WrapErr};

use crate::app::{
    AppProperties,
    AppState::{self, MOVE, MULTICOMMAND, SELECT, SIMPLECOMMAND, TEXT, UNSELECT},
    DynamicText, highlight, insert,
    text::Coordinate,
};

#[derive(Debug)]
pub struct App {
    pub status_text: Vec<Line<'static>>, // Placeholder for text object
    pub cmdbuf: Vec<KeyCode>,            // Current multi-key command buffer
    pub text: DynamicText,
    state: AppState,     // Current mutable app state
    prop: AppProperties, // Immutable session properties
    exit: bool,          // Is-exiting flag
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
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
                self.text.position.null_highlight();
                self.cmdbuf.clear();
                self.status_text.clear();
                self.status_text.push(Line::from("Command"));
                self.state = Default::default();
            }
            KeyCode::Char('h') => match self.state {
                SIMPLECOMMAND => self.text.move_left(), // Move cursor left
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('h')), // Append 'h' to multi-command
                SELECT => self.text.select_left(),      // Select to the left
                UNSELECT => self.text.unselect_left(),  // Unselect to the left
                MOVE => {}                              // Move selection to the left
                TEXT => self.text.push('h'),            // Append 'h' to text
            },
            KeyCode::Left => match self.state {
                SIMPLECOMMAND => self.text.move_left(), // Move cursor left
                MULTICOMMAND => {}                      // Append left arrow to command
                SELECT => self.text.select_left(),      // Select to the left
                UNSELECT => self.text.unselect_left(),  // Unselect to the left
                MOVE => {}                              // Move selection to the left
                TEXT => self.text.move_left(),          // Move cursor left
            },
            KeyCode::Char('j') => match self.state {
                SIMPLECOMMAND => self.text.move_down(), // Move cursor down
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('j')), // Append 'j' to multi-command
                SELECT => self.text.select_down(),      // Select to the down
                UNSELECT => self.text.unselect_down(),  // Unselect to the down
                MOVE => {}                              // Move selection to the down
                TEXT => self.text.push('j'),            // Append 'j' to text
            },
            KeyCode::Down => match self.state {
                SIMPLECOMMAND => self.text.move_down(), // Move cursor down
                MULTICOMMAND => {}                      // Append down arrow to command
                SELECT => self.text.select_down(),      // Select to the down
                UNSELECT => self.text.unselect_down(),  // Unselect to the down
                MOVE => {}                              // Move selection to the down
                TEXT => self.text.move_down(),          // Move cursor down
            }, // Move cursor down
            KeyCode::Char('k') => match self.state {
                SIMPLECOMMAND => self.text.move_up(), // Move cursor up
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('k')), // Append 'k' to multi-command
                SELECT => self.text.select_up(),      // Select to the up
                UNSELECT => self.text.unselect_up(),  // Unselect to the up
                MOVE => {}                            // Move selection to the up
                TEXT => self.text.push('k'),          // Append 'k' to text
            },
            KeyCode::Up => match self.state {
                SIMPLECOMMAND => self.text.move_up(), // Move cursor up
                MULTICOMMAND => {}                    // Append up arrow to command
                SELECT => self.text.select_up(),      // Select to the up
                UNSELECT => self.text.unselect_up(),  // Unselect to the up
                MOVE => {}                            // Move selection to the up
                TEXT => self.text.move_up(),          // Move cursor up
            }, // Move cursor up
            KeyCode::Char('l') => match self.state {
                SIMPLECOMMAND => self.text.move_right(), // Move cursor right
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('l')), // Append 'l' to multi-command
                SELECT => self.text.select_right(),      // Select to the right
                UNSELECT => self.text.unselect_right(),  // Unselect to the right
                MOVE => {}                               // Move selection to the right
                TEXT => self.text.push('l'),             // Append 'l' to text
            },
            KeyCode::Right => match self.state {
                SIMPLECOMMAND => self.text.move_right(), // Move cursor right
                MULTICOMMAND => {}                       // Append right arrow to command
                SELECT => self.text.select_right(),      // Select to the right
                UNSELECT => self.text.unselect_right(),  // Unselect to the right
                MOVE => {}                               // Move selection to the right
                TEXT => self.text.move_right(),          // Move cursor right
            }, // Move cursor right
            KeyCode::Char('m') => match self.state {
                SIMPLECOMMAND => {
                    self.cmdbuf.clear();
                    self.status_text.clear();
                    self.status_text.push(Line::from("MultiCommand"));
                    self.state = MULTICOMMAND;
                } // Go to multi-command mode
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('m')), // Append 'm' to multi-command
                TEXT => self.text.push('m'),                          // Append 'm' to text
                _ => {
                    highlight()?;
                    self.status_text.clear();
                    self.status_text.push(Line::from("Move"));
                    self.state = MOVE;
                } // Go to move mode
            },
            KeyCode::Char('v') => match self.state {
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('v')), // Append 'v' to multi-command
                TEXT => self.text.push('v'),                          // Append 'v' to text
                _ => {
                    highlight()?;
                    self.status_text.clear();
                    self.status_text.push(Line::from("Select"));
                    self.state = SELECT
                } // Enter select mode
            },
            KeyCode::Char('u') => match self.state {
                SIMPLECOMMAND => {}                                   // Do nothing
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('u')), // Append 'u' to multi-command
                TEXT => self.text.push('u'),                          // Append 'u' to text
                _ => {
                    highlight()?;
                    self.status_text.clear();
                    self.status_text.push(Line::from("UnSelect"));
                    self.state = UNSELECT;
                }
            },
            KeyCode::Char('e') => match self.state {
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('e')), // Append 'e' to multi-command
                TEXT => self.text.push('e'),                          // Append 'e' to text
                _ => {
                    insert()?;
                    self.status_text.clear();
                    self.status_text.push(Line::from("Text"));
                    self.state = TEXT
                }
            },
            KeyCode::Char('q') => match self.state {
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('q')), // Append 'q' to multi-command
                TEXT => self.text.push('q'),                          // Append 'q' to text
                _ => self.exit(),
            },
            KeyCode::Char('s') => match self.state {
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('s')), // Append 's' to multi-command
                TEXT => self.text.push('s'),                          // Append 's' to text
                _ => self.save()?,
            },
            KeyCode::Char('z') => match self.state {
                SIMPLECOMMAND => {}                                   // Undo latest command
                MULTICOMMAND => self.cmdbuf.push(KeyCode::Char('z')), // Append 'z' to multi-command
                SELECT => {}                                          // Undo latest select
                UNSELECT => {}                                        // Undo latest un-select
                MOVE => {}                                            // Undo latest move
                TEXT => self.text.push('z'),                          // Append 'z' to text
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
                ModifierKeyCode::LeftShift => match self.state {
                    SELECT => {
                        self.status_text.clear();
                        self.status_text.push(Line::from("UnSelect"));
                        self.state = UNSELECT;
                    }
                    _ => {} // Fall-through, capitalize next character
                },
                _ => {}
            },
            KeyCode::Enter => match self.state {
                SIMPLECOMMAND => {} // Move cursor down newline
                MULTICOMMAND => {
                    self.state = Default::default();
                    self.status_text.clear();
                    self.status_text.push(Line::from("Command"));
                    self.handle_command()?
                }
                SELECT => {}                  // Do nothing
                UNSELECT => {}                // Do nothing
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
                TEXT => {
                    self.text.pop();
                } // Remove previous character
                _ => {} // Do nothing
            },
            k => match self.state {
                MULTICOMMAND => self.cmdbuf.push(k), // Append k to command buf
                TEXT => self.text.push(k.to_string().pop().unwrap()), // Append k to text
                _ => {}                              // Do nothing
            },
        }

        Ok(())
    }

    fn handle_command(&mut self) -> Result<()> {
        self.cmdbuf.clear();
        Ok(())
    }

    fn save(&mut self) -> Result<()> {
        self.status_text.push(Line::from("Saving"));
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
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
            exit: false,
        }
    }
}
