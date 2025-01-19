

use std::io::{self,stdin, Read};
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
fn main(){
    enable_raw_mode().unwrap();
    for byte in stdin().bytes(){
        let b = byte.unwrap();
        let ch = b as char;
        
        if ch.is_control(){ // Characters like \0 , \n , \t which are not printable
            println!("Binary : {0:08b} , ASCII : {0:#03}",b);
        }else{
            println!("Binary : {0:08b} , ASCII : {0:#03} , Character : {1:#?}",b,ch);
        }

        if ch == 'q'{
            println!("Quitting");
            disable_raw_mode().unwrap();
            break;
        }
        println!("You entered :{} ",ch);
    }
}