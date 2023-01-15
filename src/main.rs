#![allow(unused)]

use fom::game;
use std::io;
use text_colorizer::{Color::*, Colorize};

fn main() {
    println!("\t++>> Fate of Mystria <<++");
    println!("\t...by JameyDev\n");
    game_loop();
}

fn game_loop() {
    let greeting = String::from("Greetings, hero. What is your name?").green();
    println!("{greeting}");
}
