

use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use crossterm::event::{read,Event::Key,KeyCode::Char,KeyEvent,KeyModifiers};
pub struct Editor{
    should_quit: bool
}


impl Editor{
    pub fn new() -> Self{
        Editor{should_quit: false}
    }

    pub fn run(&mut self){
        enable_raw_mode().unwrap();
      

        loop{
            if let Key(KeyEvent{code,modifiers,kind,state}) = read().unwrap(){

                match code {
                    Char('q') if  modifiers == KeyModifiers::CONTROL=>{
                        self.should_quit = true;
                    },
                    _=>{}
                }
            }
            if self.should_quit{
                print!("Thlla \r\n");
                break;
            }
        }
        disable_raw_mode().unwrap();

    }
}