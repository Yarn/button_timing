
use crossterm::event::{read, Event};
use crossterm::event::{KeyEvent, KeyCode};

use std::time::{Instant, Duration};

use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

fn print_events() -> crossterm::Result<Vec<(char, Duration)>> {
    let mut events: Vec<(char, Duration)> = Vec::new();
    
    let start = Instant::now();
    
    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => {
                // println!("{:?}\r", event);
                let key: KeyCode = event.code;
                // if key == KeyCode::Char('q') {
                if key == KeyCode::Esc {
                    break;
                }
                if let KeyCode::Char(c) = key {
                    // println!("{}\r", c);
                    let new_event = (c, start.elapsed());
                    // println!("{:?}\r", new_event);
                    events.push(new_event);
                }
            }
            Event::Mouse(event) => println!("{:?}\r", event),
            Event::Resize(width, height) => println!("New size {}x{}\r", width, height),
        }
    }
    Ok(events)
}

fn write_file(events: &Vec<(char, Duration)>) -> std::io::Result<()> {
    
    let f = File::create("./out.csv")?;
    let mut b = BufWriter::new(f);
    
    for (c, dur) in events.iter() {
        let x = format!("{},{}\n", c, dur.as_millis());
        b.write_all(x.as_bytes())?;
    }
    b.flush()?;
    
    Ok(())
}

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    let events = print_events().unwrap();
    
    write_file(&events).unwrap();
}
