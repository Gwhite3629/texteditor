use std::io;

use color_eyre::Result;
use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, SetCursorStyle},
    execute,
};

pub fn insert() -> Result<()> {
    execute!(io::stdout(), EnableBlinking, SetCursorStyle::BlinkingBar)?;
    Ok(())
}
pub fn highlight() -> Result<()> {
    execute!(io::stdout(), DisableBlinking, SetCursorStyle::SteadyBlock)?;
    Ok(())
}
