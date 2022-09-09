extern crate termion;

use ctrlc;
use elementary_ca::Field;
use std::io::{stdout, Write};
use std::sync::mpsc;
use std::{thread, time};
use termion::{cursor, screen::*};

const HELP_MSG: &str = "elementary-ca [<command> [<argument>]]\n   \
    -h | --help\t\tsee this message\n   \
    -r | --rule\t\tset the ruleset\t\t\t\tu8\t90\n   \
    -s | --speed\t\tset the speed\t\t\t\tu32\t30\n   \
    -w | --width\t\tset the width\t\t\t\tusize\twindow width\n  \
    -wr | --wrap-around\tshould the CA array wrap around\t\tbool\ttrue\
    ";

fn main() {
    let args = Args::parse();
    if let None = args {
        return;
    }
    let mut args = args.unwrap();

    let (tx, rx) = mpsc::channel();
    ctrlc::set_handler(move || tx.send(()).unwrap()).unwrap();
    let mut screen = AlternateScreen::from(stdout());
    write!(
        screen,
        "{}{}{}",
        cursor::Hide,
        termion::clear::All,
        cursor::Goto(1, 1)
    )
    .unwrap();
    if args.width < 1 {
        args.width = termion::terminal_size().unwrap_or((32, 0)).0.into();
    }
    let mut field = Field::new(args.width, args.ruleset, args.wrap_around);
    field.set_state(args.width / 2, 1);

    loop {
        if let Ok(()) = rx.try_recv() {
            print!("{}", cursor::Show);
            return;
        }
        field.print_states();
        println!();
        field.apply_rules();
        field.swap_states();
        // println!("{0}{0}{0}{0}{0}{0}", time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs());
        thread::sleep(time::Duration::from_millis(args.speed as u64));
    }
}

struct Args {
    ruleset: u8,
    speed: u32,
    width: usize,
    wrap_around: bool,
}

impl Args {
    //it's a mess but it's working
    // TODO:
    // improve this
    fn parse() -> Option<Args> {
        let mut args = std::env::args().skip(1);
        let mut ruleset = 90;
        let mut speed = 30;
        let mut width = 0;
        let mut wrap_around = true;
        if args.len() < 1 {
            return Some(Args {
                ruleset,
                speed,
                width,
                wrap_around,
            });
        }
        while args.len() > 0 {
            let arg = args.next().unwrap();
            match &arg as &str {
                "-h" | "--help" => {
                    eprintln!("{}", HELP_MSG);
                    return None;
                }
                "-r" | "--rule" => {
                    let next = args.next().unwrap_or("".to_owned());
                    match next.parse::<u8>() {
                        Ok(value) => ruleset = value,
                        Err(_) => {
                            println!("invalid rule");
                            return None;
                        }
                    }
                }
                "-s" | "--speed" => {
                    let next = args.next().unwrap_or("".to_owned());
                    match next.parse::<u32>() {
                        Ok(value) => speed = 1000 / value,
                        Err(_) => {
                            println!("invalid speed");
                            return None;
                        }
                    }
                }
                "-w" | "--width" => {
                    let next = args.next().unwrap_or("".to_owned());
                    match next.parse::<std::num::NonZeroUsize>() {
                        Ok(value) => width = value.into(),
                        Err(_) => {
                            println!("invalid width");
                            return None;
                        }
                    }
                }
                "-wr" | "--wrap-around" => {
                    let next = args.next().unwrap_or("".to_owned());
                    match next.parse::<bool>() {
                        Ok(value) => wrap_around = value,
                        Err(_) => {
                            println!("invalid value");
                            return None;
                        }
                    }
                }
                _ => {
                    println!("unknown argument \"{}\"", &arg);
                    return None;
                }
            }
        }
        Some(Args {
            ruleset,
            speed,
            width,
            wrap_around,
        })
    }
}
