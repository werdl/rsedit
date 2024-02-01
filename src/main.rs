extern crate termion;

use termion::{event::Key, raw::RawTerminal};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{stdin, stdout, Stdin, Stdout, Write};

struct TextContent {
    rows: Vec<String>, // vector of lines
    hl_codes: Vec<Vec<u8>>, // vector of highlight codes, for each line
}

struct Editor {
    text_content: TextContent,
    cursor_x: usize,
    cursor_y: usize,
    screen_rows: usize,
    screen_cols: usize,
    row_offset: usize,
    col_offset: usize,
    stdin: Stdin,
    stdout: RawTerminal<Stdout>,
}

impl Editor {
    fn status_code(&mut self, s: &str) {
        write!(self.stdout,
               "{}{}{}",
               termion::cursor::Goto(1, 1),
               termion::clear::CurrentLine,
               s)
                .unwrap();
        self.stdout.flush().unwrap();
    }

    fn init(&mut self) {
        write!(
            self.stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        ).unwrap();
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut global_editor = Editor {
        text_content: TextContent {
            rows: vec![],
            hl_codes: vec![],
        },
        cursor_x: 0,
        cursor_y: 0,
        screen_rows: 0,
        screen_cols: 0,
        row_offset: 0,
        col_offset: 0,
        stdin: stdin,
        stdout: stdout,
    };

    global_editor.init();

    global_editor.status_code("Hello, world!");

    loop {
        let c = stdin.keys().next().unwrap().unwrap();
        match c {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => println!("×"),
            _ => {}
        }
        global_editor.stdout.flush().unwrap();
    }
}

    // write!(stdout,
    //        "{}{}q to exit. Type stuff, use alt, and so on.{}",
    //        termion::clear::All,
    //        termion::cursor::Goto(1, 1),
    //        termion::cursor::Hide)
    //         .unwrap();
    // stdout.flush().unwrap();

    // for c in stdin.keys() {
    //     write!(stdout,
    //            "{}{}",
    //            termion::cursor::Goto(1, 1),
    //            termion::clear::CurrentLine)
    //             .unwrap();

    //     match c.unwrap() {
    //         Key::Char('q') => break,
    //         Key::Char(c) => println!("{}", c),
    //         Key::Alt(c) => println!("^{}", c),
    //         Key::Ctrl(c) => println!("*{}", c),
    //         Key::Esc => println!("ESC"),
    //         Key::Left => println!("←"),
    //         Key::Right => println!("→"),
    //         Key::Up => println!("↑"),
    //         Key::Down => println!("↓"),
    //         Key::Backspace => println!("×"),
    //         _ => {}
    //     }
    //     stdout.flush().unwrap();
    // }

    // write!(stdout, "{}", termion::cursor::Show).unwrap();
