

use std::io::{self,stdin, Read};
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
fn main(){
    enable_raw_mode().unwrap();
    for byte in stdin().bytes(){
        let ch = byte.unwrap() as char;
        if ch == 'q'{
            println!("Quitting");
            disable_raw_mode().unwrap();
            break;
        }
        println!("You entered :{} ",ch);
    }
}