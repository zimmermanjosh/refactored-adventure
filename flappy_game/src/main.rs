// ============================================================================
// External Crate Imports (std first, then external crates)
// ============================================================================

use std::error::Error;

use bracket_lib::prelude::*;

// ============================================================================
// Type Definitions
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameMode {
    Menu,
    Playing,
    Ended,
}

struct State {
    mode: GameMode,
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

// ============================================================================
// Constants
// ============================================================================

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const GAME_TITLE: &str = "Flappy Dragon";

// ============================================================================
// Main Entry Point
// ============================================================================

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let context = BTermBuilder::simple80x50()
        .with_title(GAME_TITLE)
        .build()?;

    main_loop(context, State::new())
}

// ============================================================================
// Game State Implementation
// ============================================================================


impl Player {
    fn new (x: i32, y: i32) -> Self { 
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }
}
impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }
    
    fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            0,
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@'),
        );
    }
    // TODO: Implement menu rendering
    fn render_menu(&self, ctx: &mut BTerm) {
        ctx.print_centered(SCREEN_HEIGHT / 2 - 2, "Welcome to Flappy Dragon!");
        ctx.print_centered(SCREEN_HEIGHT / 2 , "Press SPACE to start");
        ctx.print_centered(SCREEN_HEIGHT / 2 + 2, "Press R to restart");
        ctx.print_centered(SCREEN_HEIGHT / 2 + 4, "Press ESC to quit");
    }

    // TODO: Implement game rendering
    fn render_game(&self, ctx: &mut BTerm) {
        ctx.print(1, 1, "Game is running...");
        // TODO: Add player rendering
        // TODO: Add obstacle rendering
        // TODO: Add score rendering
    }

    // TODO: Implement game over screen
    fn render_game_over(&self, ctx: &mut BTerm) {
        ctx.print_centered(SCREEN_HEIGHT / 2, "Game Over!");
        ctx.print_centered(SCREEN_HEIGHT / 2 + 2, "Press R to restart");
    }

    // TODO: Implement input handling
    fn handle_input(&mut self, ctx: &mut BTerm) {
        match ctx.key {
            Some(key) => match key {
                VirtualKeyCode::Space => {
                    if self.mode == GameMode::Menu {
                        self.mode = GameMode::Playing;
                    }
                    // TODO: Add jump logic when playing
                }
                VirtualKeyCode::R => {
                    if self.mode == GameMode::Ended {
                        self.mode = GameMode::Menu;
                    }
                }
                VirtualKeyCode::Escape => {
                    ctx.quitting = true;
                }
                _ => {}
            },
            None => {}
        }
    }

    // TODO: Implement game physics update
    fn update_game(&mut self) {
        // TODO: Update player position
        // TODO: Update obstacles
        // TODO: Check collisions
        // TODO: Update score
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Clear screen
        ctx.cls();

        // Handle input
        self.handle_input(ctx);

        // Update game state
        if self.mode == GameMode::Playing {
            self.update_game();
        }

        // Render based on current mode
        match self.mode {
            GameMode::Menu => self.render_menu(ctx),
            GameMode::Playing => self.render_game(ctx),
            GameMode::Ended => self.render_game_over(ctx),
        }
    }
}