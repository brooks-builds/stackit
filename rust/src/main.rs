use ggez::event::winit_event::{Event, KeyboardInput, WindowEvent};
use ggez::{self, event, graphics, GameResult};
use slab::Slab;

const GAME_NAME: &str = "Stack It!";
const AUTHOR: &str = "_Bare";
const SCREEN_WIDTH: f32 = 1920.0;
const SCREEN_HEIGHT: f32 = 1080.0;
const WIN_WIDTH: f32 = SCREEN_WIDTH / 2.0;
const WIN_HEIGHT: f32 = SCREEN_HEIGHT / 2.0;
const BASE_UNIT_SIZE: f32 = 32.0;

struct StackIt<'s> {
    // All your game are belong to us
    ctx: &'s mut ggez::Context,
    platform: SquareActor,
    dropper: SquareActor,
    box_actors: Slab<SquareActor>,
    box_landed: Slab<SquareActor>,
}

type Vector2 = ggez::mint::Vector2<f32>;

enum Actor {
    Box,
    Platform,
    Dropper,
}

struct SquareActor {
    actor_type: Actor,
    rect: graphics::Rect,
    velocity: Vector2,
    color: graphics::Color,
    landed: bool,
    dead: bool,
}

impl SquareActor {
    fn new(actor_type: Actor) -> SquareActor {
        SquareActor {
            actor_type,
            rect: graphics::Rect {
                x: 0.,
                y: 0.,
                w: BASE_UNIT_SIZE,
                h: BASE_UNIT_SIZE,
            },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            color: graphics::Color::from_rgb(255, 255, 255),
            landed: false,
            dead: false,
        }
    }
}

impl StackIt<'_> {
    fn new(ctx: &mut ggez::Context) -> StackIt {
        // set default game state
        // load configs, images, etc
        // construct game objects/actors and set their start positions
        let (win_w, win_h) = graphics::drawable_size(ctx);

        let mut platform = SquareActor::new(Actor::Platform);
        platform.rect.w *= 5.0;
        platform.rect.x = win_w / 2.0 - platform.rect.w / 2.0;
        platform.rect.y = win_h - platform.rect.h;
        platform.velocity.x = 1.0;

        let mut dropper = SquareActor::new(Actor::Dropper);
        dropper.rect.w += 5.0;
        dropper.rect.h += 5.0;
        dropper.rect.x = win_w / 2.0 - dropper.rect.w / 2.0;
        dropper.velocity.x = 5.0;

        let box_actors: Slab<SquareActor> = Slab::with_capacity(100);
        let box_landed: Slab<SquareActor> = Slab::with_capacity(100);

        StackIt {
            ctx,
            platform,
            dropper,
            box_actors,
            box_landed,
        }
    }

    fn update(&mut self) -> GameResult {
        graphics::set_window_title(
            self.ctx,
            format!("FPS: {:.0}", ggez::timer::fps(self.ctx)).as_str(),
        );

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
    let mut pos = window
        .get_position()
        .expect("Failed to get window position for centering!");
    pos.x = SCREEN_WIDTH as f64 / 2.0 - WIN_WIDTH as f64 / 2.0;
    pos.y = SCREEN_HEIGHT as f64 / 2.0 - WIN_HEIGHT as f64 / 2.0;
    window.set_position(pos);
}

fn main() -> GameResult {
    // Context and mostly Window setup
    let window_mode = ggez::conf::WindowMode::default().dimensions(WIN_WIDTH, WIN_HEIGHT);
    let window_setup = ggez::conf::WindowSetup::default()
        .vsync(false)
        .title(GAME_NAME);
    let module_conf = ggez::conf::ModuleConf {
        gamepad: false,
        audio: false,
    };

    let context_builder = ggez::ContextBuilder::new(GAME_NAME, AUTHOR)
        .window_mode(window_mode)
        .window_setup(window_setup)
        .modules(module_conf);
    let (ctx, events_loop) = &mut context_builder.build()?;

    center_window(ctx);

    let game = &mut StackIt::new(ctx); // does this really need to be in here?

    // Game loop
    while game.ctx.continuing {
        game.ctx.timer_context.tick();

        // Window Events
        events_loop.poll_events(|p_event| {
            game.ctx.process_event(&p_event);

            if let Event::WindowEvent { event, .. } = p_event {
                match event {
                    WindowEvent::CloseRequested => event::quit(game.ctx),

                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(keycode),
                                .. // scancode: u32, state: ElementState, modifiers: ModifiersState
                            },
                        .. // device_id: DeviceId
                    } => match keycode {
                        event::KeyCode::Escape => event::quit(game.ctx),
                        event::KeyCode::D => { todo!("Drop a box") },
                        event::KeyCode::C => { todo!("Clear boxes") },
                        _ => { /* https://docs.rs/ggez/0.5.1/ggez/input/keyboard/enum.KeyCode.html */ }
                    },

                    WindowEvent::MouseInput {
                        button,
                        .. // device_id: DeviceID, state: ElementState, modifiers: ModifiersState
                    } => match button  {
                        ggez::input::mouse::MouseButton::Left => { todo!("Left Click") },
                        ggez::input::mouse::MouseButton::Right => { todo!("Right Click") },
                        _ => { /* Right, Middle, Other(u8) */ },
                    },

                    // WindowEvent::MouseWheel {
                    //     delta,
                    //     .. // device_id: DeviceID, phase: TouchPhase modifiers: ModifiersState
                    // } => {},

                    _ => { /* https://docs.rs/ggez/0.5.1/ggez/event/winit_event/enum.WindowEvent.html */ }
                }
            }
        });

        // Network Events
        // Wire in twitch bot and/or other sources of input

        // Update
        game.update()?;

        // Render
        game.render()?;

        // ggez::timer::yield_now();
        std::thread::sleep(std::time::Duration::from_nanos(1));
    }

    Ok(())
}
