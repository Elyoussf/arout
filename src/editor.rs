
use std::io::Error;

mod terminal;
use crossterm::event::{read, Event::{self, Key}, KeyCode::{self, Char}, KeyEvent, KeyModifiers};
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
        Editor::draw_rows(self)?;
        loop{
            let event = read()?;
            self.listen_for_events(&event)?;
            if self.should_quit{
                break;
            }
        }
        Terminal::terminate_session()?;
        Ok(())
        
    }

    fn draw_rows(&mut self) ->Result<(),Error>{
        let height = Terminal::size()?.1;
        
        for i in 0..height{
            print!("~");
            if i < height-1{
                print!("\r\n");
            }
        }
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