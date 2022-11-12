use color::Color;
use event::Key;
use input::TermRead;
use raw::IntoRawMode;
use screen::IntoAlternateScreen;
use std::io;
use std::io::{stdin, stdout, Write};
use std::fs;
use termion::*;
use unicode_segmentation::UnicodeSegmentation;

struct Terminal {
    height: u16,
    width: u16,
}

impl Terminal {
    fn default() -> io::Result<Self> {
        let (width, height) = terminal_size()?;
        Ok(Self { height, width })
    }
}

#[derive(PartialEq, Eq)]
enum ColorOpt {
    Fg,
    Bg,
}

impl Terminal {
    pub(super) fn clear_screen(&self) -> &Self {
        print!("{}", clear::All);
        print!("{}", cursor::Goto(1, 1));
        self
    }

    pub(super) fn set_color<C>(&self, option: ColorOpt, color: C) -> &Self
    where
        C: Color,
    {
        if option == ColorOpt::Fg {
            print!("{}", color::Fg(color));
        } else {
            print!("{}", color::Bg(color));
        }

        self
    }

    pub(super) fn reset_color(&self) -> &Self {
        self.set_color(ColorOpt::Fg, color::Reset)
            .set_color(ColorOpt::Bg, color::Reset)
    }

    pub(super) fn print(&self, content: &str) -> &Self {
        print!("{}", content);
        stdout().flush().unwrap();

        self
    }

    pub(super) fn cursor(&self, x: u16, y: u16) -> &Self {
        print!("{}", cursor::Goto(x, y));
        stdout().flush().unwrap();

        self
    }

    pub(super) fn nl(&self) -> &Self {
        println!();
        self
    }

    pub(super) fn hide_cursor(&self) -> &Self {
        print!("{}", cursor::Hide);
        stdout().flush().unwrap();

        self
    }

    pub(super) fn show_cursor(&self) -> &Self {
        print!("{}", cursor::Show);
        stdout().flush().unwrap();

        self
    }
}

pub(super) struct Editor {
    should_exit: bool,
    terminal: Terminal,
    file_name: String,
    file: File,
}

impl Editor {
    pub fn new(filename: String) -> Self {
        let terminal = Terminal::default().expect("Error: failed to initialize the terminal.");

        if filename.is_empty() {
            return Self {
                should_exit: false,
                terminal,
                file_name: "[Unnamed]".into(),
                file: File {},
            };
        }

        Self {
            should_exit: false,
            terminal,
            file: load_file(filename.as_str()),
            file_name: filename,
        }
    }

    pub fn run(&mut self) {
        let _alter = stdout().into_alternate_screen().unwrap();
        stdout().flush().unwrap();

        self.initialize();

        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            self.proc_key();

            if self.should_exit {
                self.terminal.show_cursor();
                return;
            }
        }
    }

    fn initialize(&mut self) {
        self.terminal.clear_screen();
        self.terminal.set_color(ColorOpt::Fg, color::Reset);

        let len = self.file_name.graphemes(true).count();

        if self.terminal.width <= 1 {
            self.terminal
                .set_color(ColorOpt::Bg, color::White)
                .set_color(ColorOpt::Fg, color::Black)
                .print(".")
                .reset_color();
        }

        if len > self.terminal.width as usize {
            let mut temp = self.file_name.chars().collect::<Vec<char>>();
            let diff = len - self.terminal.width as usize;
            temp.truncate(len - diff - 3);

            let content = String::from_iter(temp);

            self.terminal
                .set_color(ColorOpt::Bg, color::White)
                .set_color(ColorOpt::Fg, color::Black)
                .print(("...".to_string() + content.as_str()).as_str())
                .reset_color();
        } else {
            let front = (self.terminal.width as usize - len) / 2;
            let back = self.terminal.width as usize - front - len;

            let mut front_spaces: String = String::new();
            let mut back_spaces: String = String::new();

            for _ in 0..front {
                front_spaces.push(' ');
            }
            for _ in 0..back {
                back_spaces.push(' ');
            }

            self.terminal
                .set_color(ColorOpt::Bg, color::White)
                .set_color(ColorOpt::Fg, color::Black)
                .print(format!("{}{}{}", front_spaces, self.file_name, back_spaces).as_str())
                .nl()
                .reset_color();
        }

        self.terminal.cursor(1, 1);
        self.terminal.hide_cursor();
    }

    fn proc_key(&mut self) {
        let key = self.read_key();

        if key.is_err() {
            panic!("Cannot read a key.");
        }

        if let Key::Ctrl('q') = key.unwrap() {
            self.should_exit = true;
        }
    }

    fn read_key(&self) -> io::Result<Key> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key;
            }
        }
    }
}

fn load_file(path: &str) -> File {
    let buffer = fs::read_to_string(path);

    if buffer.is_err() {
        return File {}
    }

    unimplemented!()
}

struct File {

}

struct Row {
    content: String,
}
