
use crossterm::event;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use crossterm::event::{read,Event::Key,KeyCode::Char};
pub struct Editor{

}


impl Editor{
    pub fn New() -> Self{
        Editor{}
    }

    pub fn Run(&self){
        enable_raw_mode().unwrap();
        // for byte in stdin().bytes(){

        //     match byte{
        //         Ok(b)=>{
        //             let ch = b as char;
            
        //             if ch.is_control(){ // Characters like \0 , \n , \t which are not printable
        //                 println!("Binary : {0:08b} , ASCII : {0:#03}",b);
        //             }else{
        //                 println!("Binary : {0:08b} , ASCII : {0:#03} , Character : {1:#?}",b,ch);
        //             }
        //             println!("\r");
        //             if ch == 'q'{
        //                 println!("Quitting \r");
        //                 disable_raw_mode().unwrap();
        //                 break;
        //             }
        //         },
        //         Err(e)=>println!("Error : {} ",e)
        //     }
        // }

        loop{
            match read(){
                Ok(Key(event))=>{
                    match event.code{
                        Char(c)=>{
                            if c == 'q'{
                                println!("Thanks for quitting\r");
                                break;
                            }
                            
                        },
                        _=>{}
                    }
                },
                Err(e)=>{println!("The error : {}",e)},
                _=>{}
            }
        }
        disable_raw_mode().unwrap();

    }
}