use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::Event;
use crossterm::event::{read,Event::Key,KeyCode::Char,KeyEvent,KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType,size};
use std::io::stdout;
use std::io::Error;

pub struct Terminal{}
pub struct Size{
    pub height : u16,
    pub width : u16
}
impl Terminal{
    pub fn new() -> Terminal{
        Terminal{}
    }
    

    pub fn clear_screen() -> Result<(), Error>{
        let mut  stdout = stdout();
        execute!(stdout,Clear(ClearType::All))
    }
    pub fn start_up(self) -> Result<(),Error>{
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)
    }
    pub fn terminate_session() -> Result<(),Error>{
        disable_raw_mode()?;
        Self::clear_screen()
        
    }
    
    pub fn move_cursor_to(x : u16, y : u16) -> Result<(),Error>{
        execute!(stdout(),MoveTo(x,y))?;
        Ok(())
    }
    pub fn size() -> Result<Size,Error>{
        let res = size()?;
        Ok(Size{
            height: res.1,
            width : res.0
        })
    }
    pub fn hide_cursor() -> Result<(),Error>{
        execute!(stdout(),Hide)
    }
    pub fn show_cursor() -> Result<(),Error>{
        execute!(stdout(),Show)
    }
}