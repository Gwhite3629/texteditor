pub mod app;

use std::{env, io::stdout};

use app::App;

use color_eyre::Result;
use crossterm::{
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    // Enable enhancement flags
    execute!(
        stdout(),
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
        )
    )?;

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal, args);
    ratatui::restore();
    execute!(stdout(), PopKeyboardEnhancementFlags)?;
    app_result
}
