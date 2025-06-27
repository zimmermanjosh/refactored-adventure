use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    Ended,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}

struct State {
    mode: GameMode,
}
impl State {
    fn new() -> State {
        State {
            mode: GameMode::Menu,
        }
    }
}
fn main() -> Result<(), std::boxed::Box<(dyn std::error::Error + Send + Sync + 'static)>> {
    let context_result = BTermBuilder::simple80x50()
        .with_title("Flappy Game")
        .build();

    match context_result {
        Ok(context) => main_loop(context, State::new()),
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}