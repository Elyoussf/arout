
use std::io::{stdout, Error, Write};

mod terminal;
use crossterm::{event::{read, Event::{self, Key}, KeyCode::{self, Char}, KeyEvent, KeyModifiers}, execute, queue, style::Print};
use terminal::Terminal;
pub struct Editor{
    should_quit: bool
}

impl Editor{
   
    pub fn new() -> Editor{
        Editor { should_quit: false }
    }


    pub fn run(&mut self) -> Result<(),Error>{
        
        let my_terminal = Terminal::new();
        my_terminal.start_up()?;
        self.draw_rows()?;
        loop{
            let event = read()?;
            self.listen_for_events(&event)?;
            Terminal::hide_cursor()?;

            self.refresh_screen()?;

            Terminal::show_cursor()?;
            
            if self.should_quit{
                break;
            }
        }
        Terminal::terminate_session()?;
        Ok(())
        
    }
    fn refresh_screen(&mut self) -> Result<(),Error>{
        if self.should_quit{
            Terminal::terminate_session()?;
        }else{
            self.draw_rows()?;  // This is very costy 
            Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }
    fn draw_rows(&mut self) ->Result<(),Error>{
        let height = Terminal::size()?.height;
        
        for i in 0..height{
            
            if i < height-1{
                queue!(stdout(),Print("~\r\n"))?;
            }
        }
        stdout().flush()?;
        Ok(())
        
    }

    fn listen_for_events(&mut self ,event : &Event)->Result<(),Error>{
        if let Key(KeyEvent{code,modifiers,kind,state}) = *event{
            match code {
                Char('q') if modifiers == KeyModifiers::CONTROL=>{
                    self.should_quit = true;
                },
                _=> {}
            }
        }
        Ok(())
    }
}