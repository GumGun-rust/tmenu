/*
use signal_hook::consts::signal::SIGWINCH;

fn main() {
    println!("Hello, world!");
}

fn call (mut signals:Signals) {
    for sig in signals.forever() {
        println!("Received signal {:?}", sig);
    }
}
    

use signal_hook::{consts::SIGWINCH, iterator::Signals};
use std::{error::Error, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let signals = Signals::new([SIGWINCH])?;

    thread::spawn(move || call(signals));// Turning the function into a closure
    
    // Following code does the actual work, and can be interrupted by pressing
    // Ctrl-C. As an example: Let's wait a few seconds.
    
    Ok(())
}

*/

extern crate termion;

use termion::{color, color::Bg, style};
use std::thread;
use std::time::Duration;
use std::io;
use std::io::Read;
use std::io::{Write, stdout, Stdout, stdin};
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::async_stdin;
use termion::event::parse_event;
use termion::input::TermRead;
use termion::input::TermReadEventsAndRaw;
use std::io::{Error, ErrorKind};


fn main() {
    let commands = vec!["hola".to_string(), "adios".to_string()];
    /*
    println!("{}Red", color::Fg(color::Red));
    println!("{}Blue", color::Fg(color::Blue));
    println!("{}Blue'n'Bold{}", style::Bold, style::Reset);
    print!("{}Stuff", termion::cursor::Goto(1, 1));
    println!("{}Just plain italic", style::Italic);
    print!("{}Stuff", termion::cursor::Goto(10, 10));
    */
    eprintln!("hola");
    let (mut stdout, mut as_stdin) = startup().unwrap();
    //let mut stdin = stdin();
    let mut buffer = String::new();
    let mut counter = 0;
    let mut input = [0u8;64+5];
    loop {
        render_commands(&mut buffer, &commands, 0);
        let inputs = as_stdin.read(&mut input[..64]).unwrap();
        let mut jumps = 0;
        
        for index in 0..inputs {
            if jumps>0 {
                jumps -= 1;
                continue;
            }
            let holder = input[index];
            let mut temp_iter = input[index+1..index+5].iter().enumerate().map(|(iter, a)|if inputs-1>iter+index {Ok(*a)} else {Err(Error::new(ErrorKind::Other, ":("))});
            for clon in temp_iter.clone() {
                eprint!("{:?}", clon);
            }
            eprintln!();
            let event = parse_event(holder, &mut temp_iter);
            jumps = 4 - temp_iter.len() ;
            
            eprintln!("\rhola {} {} {:?}", temp_iter.len(), counter, event);
        }
        
        /*
        let mut iter = input[..inputs].into_iter();
        let mut holder = iter.next();
        while let Some(item) = holder {
            let mut result_input = iter.map(|next|Ok(*next));
            let event = parse_event(*item, &mut result_input);
            eprintln!("hola {} {:?}", counter, event);
            holder = iter.next();
        }
        */
        /*
        for ev in stdin().events_and_raw() {
            eprintln!("{ev:?}");
        }
        */
        counter += 1;
        stdout.write(buffer.as_bytes()).unwrap();
        thread::sleep(Duration::from_secs(4));
    }
}

fn clear(buffer:&mut String) {
    buffer.clear();
    buffer.push_str(&format!("{}", termion::clear::All));
    buffer.clear();
}


fn startup() -> Result<(RawTerminal<Stdout>, termion::AsyncReader), ()>{
    let stdout = stdout().into_raw_mode().map_err(|_|())?;
    let mut stdin = async_stdin();
    Ok((stdout, stdin))
}

fn render_commands(buffer:&mut String, command_list:&Vec<String>, current:usize) -> Result<(),()> {
    buffer.push_str(&format!("{}", termion::cursor::Goto(1, 1)));
    for (index, command) in command_list.iter().enumerate() {
        if index == current {
            buffer.push_str(&format!("{}{}{}\n\r", Bg(color::Blue), command, Bg(color::Reset)));
        } else {
            buffer.push_str(command);
            buffer.push_str("\n\r");
        }
    }
    Ok(())
}
