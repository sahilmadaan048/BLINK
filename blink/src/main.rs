#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

fn main() {
    let mut editor = Editor::default(); // Initialize an editor instance
    editor.run(); // Run the editor
}