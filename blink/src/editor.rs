use crossterm::event::KeyEvent;
// use std::io::{self, Read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::event::{read, Event::Key, KeyCode::Char};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor {should_quit: false}
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:?}");
        }
        print!("Goodbye.\r\n");
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode();
        loop {
            if let Key(KeyEvent{
                code,  modifiers, kind, state
            }) = read()? {
                println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r"); 1
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => { 1
                        self.should_quit = true; 1
                    }
                    _ => (),
                }
            }
            if self.should_quit {
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
