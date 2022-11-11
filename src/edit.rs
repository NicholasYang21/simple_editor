use std::io;
use std::io::{stdin, stdout, Write};
use termion::color::Rgb;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
use termion::*;

struct Terminal {
    height: u16,
    weight: u16,
}

impl Terminal {
    fn default() -> io::Result<Self> {
        let (height, weight) = termion::terminal_size()?;
        Ok(Self { height, weight })
    }
}

impl Terminal {
    pub(super) fn alternate(&self) -> &Self {
        let _alter = stdout().into_alternate_screen().unwrap();
        self
    }

    pub(super) fn raw_mode(&self) -> &Self {
        let _stdout = stdout().into_raw_mode().unwrap();
        self
    }

    pub(super) fn clear_screen(&self) -> &Self {
        print!("{}", clear::All);
        print!("{}", cursor::Goto(1, 1));
        self
    }

    pub(super) fn print_fg(&self, content: &str, color: Rgb) -> &Self {
        let str = format!("{}{}", color::Fg(color), content);
        print!("{}", str);
        self
    }

    pub(super) fn cursor(&self, x: u16, y: u16) -> &Self {
        print!("{}", cursor::Goto(x, y));
        self
    }

    pub(super) fn print_bg(&self, content: &str, color: Rgb) -> &Self {
        let str = format!("{}{}", color::Bg(color), content);
        print!("{}", str);
        self
    }

    pub(super) fn newline(&self) -> &Self {
        println!();
        self
    }
}

pub(super) struct Editor {
    should_exit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn default() -> Self {
        let terminal = Terminal::default().expect("Error: failed to initialize the terminal.");
        Self {
            should_exit: false,
            terminal,
        }
    }

    pub fn run(&mut self) {
        self.terminal.clear_screen();
        self.terminal.alternate();

        for _ in 0..self.terminal.height {
            self.terminal
                .print_fg("~", Rgb(0x3b, 0x92, 0xe3))
                .newline();
        }

        self.terminal.print_fg("", Rgb(255, 255, 255));
        self.terminal.cursor(1, 1);

        stdout().flush().unwrap();

        self.terminal.raw_mode();

        loop {
            self.proc_key();

            if self.should_exit {
                break;
            }
        }
    }

    pub fn proc_key(&mut self) {
        let key = self.read_key();

        if key.is_err() {
            panic!("Cannot read a key.");
        }

        if let Key::Ctrl('q') = key.unwrap() {
            self.should_exit = true;
        }
    }

    pub fn read_key(&self) -> io::Result<Key> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key
            }
        }
    }

    pub fn _load_file(_path: &str) {}
}
