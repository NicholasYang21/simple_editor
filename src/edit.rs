use color::Color;
use event::Key;
use input::TermRead;
use raw::IntoRawMode;
use screen::IntoAlternateScreen;
use std::io;
use std::io::{stdin, stdout, Write};
use termion::*;
use unicode_segmentation::UnicodeSegmentation;

use crate::file::{load_file, File};

struct Coord {
    x: usize,
    y: usize,
}

impl Default for Coord {
    fn default() -> Self {
        Self { x: 2, y: 1 }
    }
}

pub(super) struct Editor {
    should_exit: bool,
    terminal: Terminal,
    cursor_pos: Coord,
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
                cursor_pos: Coord::default(),
                file_name: "[Unnamed]".into(),
                file: File {
                    lines: Vec::new(),
                    dirty: false,
                },
            };
        }

        Self {
            should_exit: false,
            terminal,
            cursor_pos: Coord::default(),
            file: load_file(filename.as_str()),
            file_name: filename,
        }
    }

    pub fn run(&mut self) {
        let _alter = stdout().into_alternate_screen().unwrap();
        stdout().flush().unwrap();

        self.initialize();
        self.refresh();

        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            self.proc_key();
            self.refresh();

            if self.should_exit {
                self.terminal.show_cursor();
                return;
            }
        }
    }

    fn refresh(&mut self) {
        self.terminal
            .cursor(self.cursor_pos.x as u16, self.cursor_pos.y as u16);
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

        let temp = self.file.lines.clone();
        let mut temp2 = temp;

        if !self.file.lines.is_empty() {
            let x = temp2.pop().unwrap();

            for i in temp2 {
                self.terminal.print(format!("{}", i).as_str()).nl();
            }

            self.terminal.print(format!("{}", x).as_str());

            self.cursor_pos = Coord {
                x: self.file.lines.len(),
                y: x.len(),
            };
        }
    }

    fn proc_key(&mut self) {
        let key = self.read_key();

        if key.is_err() {
            panic!("Cannot read a key.");
        }

        let Coord { mut x, mut y } = self.cursor_pos;
        let text_height = self.file.lines.len();
        let text_width = if let Some(row) = self.file.lines.get(x - 2) {
            row.len()
        } else {
            1
        };

        match key.unwrap() {
            Key::Ctrl('q') => self.should_exit = true,
            Key::Up => {
                if self.file.is_empty() {
                    return;
                }

                if x > 2 {
                    x = x.saturating_sub(1);
                    let before = self.file.lines.get(x - 2).unwrap();

                    if y > before.len() {
                        y = before.len() + 1;
                    }
                }
            }

            Key::Down => {
                if self.file.is_empty() {
                    return;
                }

                if x < text_height {
                    x += 1;
                    let next = self.file.lines.get(x - 2).unwrap();

                    if y > next.len() {
                        y = next.len() + 1;
                    }
                }
            }

            Key::Left => {
                if self.file.is_empty() {
                    return;
                }

                if y > 1 {
                    y -= 1;
                } else if x > 2 {
                    x -= 1;
                    if let Some(before) = self.file.lines.get(x - 2) {
                        y = before.len() + 1
                    } else {
                        y = 0
                    }
                }
            }

            Key::Right => {
                if self.file.is_empty() {
                    return;
                }

                if y <= text_width {
                    y += 1;
                } else if x < text_height {
                    x += 1;
                    if self.file.lines.get(x - 2).is_some() {
                        y = 0
                    }
                }
            }

            Key::Home => y = 0,
            Key::End => x = text_width + 1,

            Key::PageUp => {
                unimplemented!()
            }

            Key::PageDown => {
                unimplemented!()
            }

            _ => (),
        }

        self.cursor_pos = Coord { x, y }
    }

    fn read_key(&self) -> io::Result<Key> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key;
            }
        }
    }
}

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
        print!("{}", cursor::Goto(y, x));
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
