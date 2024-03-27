use termion::event::Key;
use termion::input::TermRead;
use termion::{clear, cursor, screen, style};

use std::io::{Read, Write};

use crate::pattern;

pub struct Game<'a> {
    pattern: pattern::Pattern,
    stdin: &'a mut dyn Read,
    stdout: &'a mut dyn Write,
}

impl Drop for Game<'_> {
    fn drop(&mut self) {
        write!(self.stdout, "{}{}", cursor::Show, screen::ToMainScreen).unwrap();
    }
}

impl Game<'_> {
    pub fn new<'a>(size: usize, stdin: &'a mut dyn Read, stdout: &'a mut dyn Write) -> Game<'a> {
        let mut g = Game {
            pattern: pattern::Pattern::new(size),
            stdin,
            stdout,
        };

        g.pattern.shuffle(30 * (size * size * size));
        g
    }

    pub fn start(&mut self) {
        write!(self.stdout, "{}", screen::ToAlternateScreen).unwrap();

        draw(self.stdout, &self.pattern);

        for evt in self.stdin.keys() {
            match evt {
                Ok(Key::Down) => {
                    self.pattern.up();
                    draw(self.stdout, &self.pattern);
                }
                Ok(Key::Up) => {
                    self.pattern.down();
                    draw(self.stdout, &self.pattern);
                }
                Ok(Key::Right) => {
                    self.pattern.left();
                    draw(self.stdout, &self.pattern);
                }
                Ok(Key::Left) => {
                    self.pattern.right();
                    draw(self.stdout, &self.pattern);
                }
                Ok(Key::Char('q')) => break,
                _ => {}
            }

            if self.pattern.is_in_order() {
                self.show_winning_screen();
                break;
            }
        }
    }

    pub fn show_winning_screen(&mut self) {
        draw(self.stdout, &self.pattern);
        write!(
            self.stdout,
            "\r\n\r\nYou have solved the puzzle! Press any key to continue.\r\n"
        )
        .unwrap();
        self.stdin.keys().next();
    }
}

pub fn draw(stdout: &mut dyn Write, pattern: &pattern::Pattern) {
    write!(
        stdout,
        "{}{}{}{}",
        cursor::Hide,
        clear::All,
        style::Reset,
        cursor::Goto(1, 1)
    )
    .unwrap();
    write!(
        stdout,
        "Use the arrow keys to move the tiles. Press 'q' to quit.\r\n\r\n"
    )
    .unwrap();

    stdout.write_all("╭────┬".as_bytes()).unwrap();

    for _i in 1..(pattern.size - 1) {
        stdout.write_all("────┬".as_bytes()).unwrap();
    }

    stdout.write_all("────╮\n\r".as_bytes()).unwrap();

    for row in 0..(pattern.size - 1) {
        for v in pattern.get_row(row).unwrap() {
            stdout.write_all("│".as_bytes()).unwrap();
            if *v == pattern.get_last_tile() as u8 {
                write!(stdout, "    ").unwrap();
            } else {
                write!(stdout, "{:3} ", v).unwrap();
            }
        }
        stdout.write_all("│\n\r".as_bytes()).unwrap();

        stdout.write_all("├────┼".as_bytes()).unwrap();

        for _i in 1..(pattern.size - 1) {
            stdout.write_all("────┼".as_bytes()).unwrap();
        }

        stdout.write_all("────┤\n\r".as_bytes()).unwrap();
    }

    for v in pattern.get_row(pattern.size - 1).unwrap() {
        stdout.write_all("│".as_bytes()).unwrap();
        if *v == pattern.get_last_tile() as u8 {
            write!(stdout, "    ").unwrap();
        } else {
            write!(stdout, "{:3} ", v).unwrap();
        }
    }

    stdout.write_all("│\n\r".as_bytes()).unwrap();

    stdout.write_all("╰────┴".as_bytes()).unwrap();

    for _i in 1..(pattern.size - 1) {
        stdout.write_all("────┴".as_bytes()).unwrap();
    }

    stdout.write_all("────╯\n\r".as_bytes()).unwrap();
}
