use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, ModifierKeyCode};
use ratatui::{DefaultTerminal, Frame, text::Line};

use color_eyre::{Result, eyre::WrapErr};

use crate::app::{
    AppProperties,
    AppState::{self, EDIT, MULTICOMMAND, SIMPLECOMMAND},
};

#[derive(Debug, Default)]
pub struct App {
    pub text: Vec<Line<'static>>, // Placeholder for text object
    cmdbuf: Vec<KeyCode>,         // Current multi-key command buffer
    state: AppState,              // Current mutable app state
    prop: AppProperties,          // Immutable session properties
    exit: bool,                   // Is-exiting flag
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
        match self.state {
            SIMPLECOMMAND => match key_event.code {
                KeyCode::Esc => {
                    self.text.clear();
                    self.text.push(Line::from("Command"));
                    self.state = Default::default();
                }
                KeyCode::Modifier(modifier) => match modifier {
                    ModifierKeyCode::LeftControl => {
                        self.cmdbuf.clear();
                        self.text.clear();
                        self.text.push(Line::from("MultiCommand"));
                        self.state = MULTICOMMAND;
                    } // Begin multi-key command
                    _ => {}
                },
                KeyCode::Char('q') => self.exit(), // Run exit function
                KeyCode::Char('e') => {
                    self.text.clear();
                    self.text.push(Line::from("Edit"));
                    self.state = EDIT
                }
                KeyCode::Char('s') => self.save(),
                KeyCode::Char('h') | KeyCode::Left => {} // Move cursor left
                KeyCode::Char('j') | KeyCode::Down => {} // Move cursor down
                KeyCode::Char('k') | KeyCode::Up => {}   // Move cursor up
                KeyCode::Char('l') | KeyCode::Right => {} // Move cursor right
                _ => {}
            },
            MULTICOMMAND => match key_event.code {
                KeyCode::Esc => {
                    self.text.clear();
                    self.text.push(Line::from("Command"));
                    self.state = Default::default()
                }
                KeyCode::Enter => {
                    self.state = Default::default();
                    return self.handle_command();
                } // Resolve multi-key command
                k => self.cmdbuf.push(k),
            },

            EDIT => match key_event.code {
                KeyCode::Esc => {
                    self.text.clear();
                    self.text.push(Line::from("Command"));
                    self.state = Default::default()
                }
                // Return to default state
                _ => {}
            },
        }
        Ok(())
    }

    fn handle_command(&self) -> Result<()> {
        todo!()
    }

    fn save(&mut self) {
        todo!()
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
