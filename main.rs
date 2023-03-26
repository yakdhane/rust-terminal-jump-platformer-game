
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::event::Key;
use termion::input::TermRead;
use std::time::{Duration, Instant};
use std::thread;
use game::Game;

mod game;
mod player;
mod platform;

fn main() {
    let stdout = stdout().into_raw_mode().unwrap();
    let stdout = AlternateScreen::from(stdout);
    let mut game = Game::new(stdout);
    game.run();
}
