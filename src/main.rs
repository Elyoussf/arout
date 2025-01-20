#![warn(clippy::all, clippy::pedantic)]
mod editor;

fn main(){
    let mut  ed = editor::Editor::new();
    ed.run();
}