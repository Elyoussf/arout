#![warn(clippy::all, clippy::pedantic)]
mod editor;

fn main(){
    let mut  ed = editor::Editor::new();
    let _ =ed.run();
}