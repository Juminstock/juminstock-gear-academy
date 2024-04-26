#![no_std]

use gstd::{};
use pebbles_game_io::*;

static mut PABBLES_GAME: Option<GameState> = None;

#[no_mangle]
extern "C" fn init() {
    // Code here
}

#[no_mangle]
extern "C" fn handle() {
    // Code here
}

#[no_mangle]
extern "C" fn state() {
    // Code here
}