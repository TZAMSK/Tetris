use crate::engine::Engine;

pub struct Interface {
    engine: Engine,
}

impl Interface {
    pub fn run(engine: Engine) {
        let interface = Self { engine };

        // Replace this with actual game logic
        todo!("Run the game")
    }
}
