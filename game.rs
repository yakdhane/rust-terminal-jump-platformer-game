
use crate::player::Player;
use crate::platform::Platform;
use std::io::{stdout, Write};
use termion::clear;
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use std::time::{Duration, Instant};

pub struct Game {
    pub player: Player,
    pub platforms: Vec<Platform>,
    pub stdout: RawTerminal<std::io::Stdout>,
    pub score: u32,
}

impl Game {
    pub fn new(stdout: RawTerminal<std::io::Stdout>) -> Game {
        let player = Player::new();
        let platforms = vec![
            Platform::new(0, 20, 20),
            Platform::new(30, 15, 20),
            Platform::new(60, 10, 20),
            Platform::new(90, 5, 20),
        ];
        Game {
            player,
            platforms,
            stdout,
            score: 0,
        }
    }

    pub fn run(&mut self) {
        let mut last_frame = Instant::now();
        loop {
            let delta = last_frame.elapsed();
            last_frame = Instant::now();
            self.handle_input();
            self.update(delta);
            self.draw();
            std::thread::sleep(Duration::from_millis(16));
        }
    }

    fn handle_input(&mut self) {
        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('q') => std::process::exit(0),
                Key::Char(' ') => self.player.jump(),
                _ => {}
            }
        }
    }

    fn update(&mut self, delta: Duration) {
        self.player.update(delta);
        if self.player.y < 0 {
            std::process::exit(0);
        }
        self.score += 1;
        for platform in self.platforms.iter_mut() {
            platform.update(delta);
            if platform.intersects(&self.player) {
                self.player.land_on(platform);
            }
        }
    }

    fn draw(&mut self) {
        write!(self.stdout, "{}", clear::All).unwrap();
        let (x, y) = stdout().cursor_pos().unwrap();
        write!(self.stdout, "{}Score: {}", cursor::Goto(1, 1), self.score).unwrap();
        write!(self.stdout, "{}{}", cursor::Goto(1, 2), self.player).unwrap();
        for platform in self.platforms.iter() {
            write!(self.stdout, "{}", platform).unwrap();
        }
        self.stdout.flush().unwrap();
    }
}
