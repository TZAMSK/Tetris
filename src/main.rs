#![allow(dead_code)]

mod engine;
mod interface;
mod piece;

use engine::Engine;

fn main() {
    let engine = Engine::new();
    interface::run(engine)
}
