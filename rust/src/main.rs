use ggez::event::winit_event::{Event, KeyboardInput, WindowEvent};
use ggez::{self, event, graphics, GameResult};

const GAME_NAME: &str = "Stack It!";
const AUTHOR: &str = "_Bare";
const SCREEN_WIDTH: f32 = 1920.0;
const SCREEN_HEIGHT: f32 = 1080.0;
const WIN_WIDTH: f32 = SCREEN_WIDTH / 2.0;
const WIN_HEIGHT: f32 = SCREEN_HEIGHT / 2.0;

struct StackIt<'s> {
    // All your game are belong to us
    ctx: &'s mut ggez::Context,
}

fn main() -> GameResult {
    // Context and mostly Window setup
    let window_mode = ggez::conf::WindowMode::default().dimensions(WIN_WIDTH, WIN_HEIGHT);

    let context_builder = ggez::ContextBuilder::new(GAME_NAME, AUTHOR).window_mode(window_mode);
    let (ctx, events_loop) = &mut context_builder.build()?;

    graphics::set_window_title(ctx, GAME_NAME);

    center_window(ctx);

    let game = &mut StackIt::new(ctx); // does this really need to be in here?

    // Game loop
    while game.ctx.continuing {
        game.ctx.timer_context.tick();

        // Input
        events_loop.poll_events(|event| {
            game.ctx.process_event(&event);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => event::quit(game.ctx),
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    } => match keycode {
                        event::KeyCode::Escape => event::quit(game.ctx),
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        });

        // Update
        game.update()?;

        // Render
        game.render()?;

        ggez::timer::yield_now();
    }

    Ok(())
}

impl StackIt<'_> {
    fn new(ctx: &mut ggez::Context) -> StackIt {
        // set default game state
        // load configs, images, etc
        StackIt { ctx }
    }

    fn update(&mut self) -> GameResult {
        Ok(())
    }

    fn render(&mut self) -> GameResult {
        graphics::clear(self.ctx, graphics::BLACK);
        graphics::present(self.ctx)?;

        Ok(())
    }
}

// there has to be a better way to do this
fn center_window(ctx: &mut ggez::Context) {
    let window = graphics::window(ctx);
    let mut pos = window.get_position().unwrap();
    pos.x = SCREEN_WIDTH as f64 / 2.0 - WIN_WIDTH as f64 / 2.0;
    pos.y = SCREEN_HEIGHT as f64 / 2.0 - WIN_HEIGHT as f64 / 2.0;
    window.set_position(pos);
}
