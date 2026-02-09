use crossterm::terminal::{enable_raw_mode,disable_raw_mode, Clear, ClearType, size};
use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::style::Print;
use crossterm::queue;
use std::io::stdout;
use std::io::Error;

pub struct Size {
    #[allow(unused)]
    pub width: u16,
    pub height: u16,
}

pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal {

}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::cursor_to(&Position { x: 0, y: 0 })?;
        Ok(())
    }
    
    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn cursor_to(position: &Position) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(position.x, position.y))?;
        Ok(())
    }

    pub fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        Ok(Size { width, height })
    }

    pub fn print(text: &str) -> Result<(), std::io::Error> {
        queue!(stdout(), Print(text))?;
        Ok(())
    }
}