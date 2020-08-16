use ggez::event::winit_event::{ElementState, Event, KeyboardInput, WindowEvent};
use ggez::{self, event, graphics, GameResult};
use rand::{self, Rng};
use slab::Slab;
use std::{thread, time};

// putting some of this here for ease of editing until handled better
const GAME_NAME: &str = "Stack It!";
const AUTHOR: &str = "_Bare";
const SCREEN_WIDTH: f32 = 1920.0; // assumption until we pull it from somewhere, ggez doesn't expose it
const SCREEN_HEIGHT: f32 = 1080.0;
const WIN_WIDTH: f32 = SCREEN_WIDTH / 2.0;
const WIN_HEIGHT: f32 = SCREEN_HEIGHT / 2.0;
const BASE_UNIT_SIZE: f32 = 64.0;
const PLATFORM_WIDTH_MUL: f32 = 5.0;
const PLATFORM_SPEED: f32 = 1.0;
const DROPPER_SIZE_MUL: f32 = 1.25;
const DROPPER_SPEED: f32 = 5.0;
const BOX_IMAGES: [&str; 7] = [
    "/ferris_64.png",
    "/ferris_party_64.png",
    "/ferris_question_64.png",
    "/ferris_smile_64.png",
    "/ferris_thinking_64.png",
    "/ferris_worried_64.png",
    "/unsafe_ferris_64.png",
];

struct StackIt<'s> {
    // All your game are belong to us
    ctx: &'s mut ggez::Context,
    platform: SquareActor,
    dropper: SquareActor,
    box_actors: Slab<SquareActor>,
    box_landed: Slab<SquareActor>,
    random: rand::rngs::ThreadRng,
    box_images: Vec<graphics::Image>,
}

type Vector2 = ggez::nalgebra::Vector2<f32>;
type Point2 = ggez::nalgebra::Point2<f32>;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Actor {
    Box,
    Platform,
    Dropper,
}

#[derive(Debug, Copy, Clone)]
struct SquareActor {
    actor_type: Actor,
    location: Point2,
    size: Point2,
    velocity: Vector2,
    color: graphics::Color,
    use_image: bool,
    image_id: usize,
    landed: bool,
    dying: bool,
    dead: bool,
}

impl SquareActor {
    fn new(actor_type: Actor) -> SquareActor {
        SquareActor {
            actor_type,
            location: Point2::new(0.0, 0.0),
            size: Point2::new(BASE_UNIT_SIZE, BASE_UNIT_SIZE),
            velocity: Vector2::new(0.0, 0.0),
            color: graphics::Color::from_rgb(255, 255, 255),
            use_image: false,
            image_id: 0,
            landed: false,
            dying: false,
            dead: false,
        }
    }

    fn as_rect(&self) -> graphics::Rect {
        graphics::Rect {
            x: self.location.x,
            y: self.location.y,
            w: self.size.x,
            h: self.size.y,
        }
    }

    fn do_motion(&mut self) {
        if self.dying {
            self.velocity *= 1.05;
        }

        self.location += self.velocity;

        match self.actor_type {
            Actor::Platform => {}
            Actor::Dropper => {}
            Actor::Box => {}
        }
    }

    fn draw(&self, ctx: &mut ggez::Context, img: &Vec<graphics::Image>) -> GameResult {
        if self.use_image {
            // todo: handle images for platform and dropper. we currently assume box only
            let image = &img[self.image_id];

            // todo: our images are 64x64 but we should scale accordingly, but we are square-ish anyway
            let scalar = (1.0 / f32::from(image.width())) * BASE_UNIT_SIZE;

            let draw_params = graphics::DrawParam::default()
                .dest(self.location)
                .scale(Vector2::new(scalar, scalar));
            graphics::draw(ctx, image, draw_params)?;
        } else {
            // mesh should probably be stored
            let mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                self.as_rect(),
                self.color,
            )?;

            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }

        Ok(())
    }

    fn bounds_check(&mut self, bounds: Point2) -> bool {
        let mut bounded = false;

        if self.location.x < 0.0 {
            self.location.x = 0.0;
            bounded = true;
        } else if self.location.x + self.size.x > bounds.x {
            self.location.x = bounds.x - self.size.x;
            bounded = true;
        };

        if self.location.y < 0.0 {
            self.location.y = 0.0;
            bounded = true;
        } else if self.actor_type != Actor::Box && self.location.y + self.size.y > bounds.y {
            self.location.x = bounds.y - self.size.y;
            bounded = true;
        } else if self.actor_type == Actor::Box && self.location.y > bounds.y {
            self.velocity *= 0.0;
            self.dead = true;
            return true;
        };

        if bounded && !self.dying {
            match self.actor_type {
                Actor::Dropper | Actor::Platform => {
                    self.velocity *= -1.0;
                }
                Actor::Box => {
                    if self.landed {
                        self.set_dying()
                    } else {
                        self.velocity.x *= -1.0;
                    }
                }
            }
        }

        bounded
    }

    fn set_dying(&mut self) {
        if !self.dying {
            self.velocity *= 0.0;
            self.velocity.y = 1.0;
            self.dying = true;
            self.landed = false;
        } else if self.dying && self.velocity.y <= 0.0 {
            self.dying = false;
            self.set_dying()
        }
    }

    fn collison_check(&mut self, target: &SquareActor) -> bool {
        if target.as_rect().overlaps(&self.as_rect()) && !target.dying {
            if target.actor_type == Actor::Dropper && self.landed {
                self.set_dying();
                return true;
            }

            self.landed = true;
            self.velocity = target.velocity;
            true
        } else {
            false
        }
    }
}

// Game loop impl, only new/setup, update, and render - rest is in impl below it
impl StackIt<'_> {
    fn new(ctx: &mut ggez::Context) -> StackIt {
        // set default game state
        // load configs, images, etc
        // construct game objects/actors and set their start positions
        let mut random = rand::thread_rng();

        let (win_w, win_h) = graphics::drawable_size(ctx);

        let mut platform = SquareActor::new(Actor::Platform);
        platform.size.x *= PLATFORM_WIDTH_MUL;
        platform.location.x = win_w / 2.0 - platform.size.x / 2.0;
        platform.location.y = win_h - platform.size.y;
        platform.velocity.x = coin_flip(&mut random, PLATFORM_SPEED);
        platform.color = random_rgb(&mut random);

        let mut dropper = SquareActor::new(Actor::Dropper);
        dropper.size *= DROPPER_SIZE_MUL;
        dropper.location.x = win_w / 2.0 - dropper.size.x / 2.0;
        dropper.velocity.x = coin_flip(&mut random, DROPPER_SPEED);
        dropper.color = random_rgb(&mut random);

        let box_actors: Slab<SquareActor> = Slab::with_capacity(100);
        let box_landed: Slab<SquareActor> = Slab::with_capacity(100);

        let mut box_images = Vec::with_capacity(BOX_IMAGES.len());
        for img in BOX_IMAGES.iter() {
            box_images.push(graphics::Image::new(ctx, img).expect(
                "Images are not in the right place or some are missing! Check the READ.ME!",
            ));
        }

        StackIt {
            ctx,
            platform,
            dropper,
            box_actors,
            box_landed,
            random,
            box_images,
        }
    }

    fn update(&mut self) -> GameResult {
        // fixme we get this in new and store it
        let bounds = graphics::drawable_size(self.ctx);
        let bounds = Point2::new(bounds.0, bounds.1);

        // todo: remove this
        graphics::set_window_title(
            self.ctx,
            format!(
                "FPS: {:.1} Boxes: {}",
                ggez::timer::fps(self.ctx),
                self.box_actors.len() + self.box_landed.len()
            )
            .as_str(),
        );

        self.platform.do_motion();
        let platform_bounded = self.platform.bounds_check(bounds);

        self.dropper.do_motion();
        self.dropper.bounds_check(bounds);

        for (_idx, box_actor) in self.box_actors.iter_mut() {
            box_actor.do_motion();
            box_actor.bounds_check(bounds);

            if box_actor.collison_check(&self.platform) {
                self.box_landed.insert(box_actor.clone());
                continue;
            }

            for (_idx, landed_box) in self.box_landed.iter_mut() {
                if box_actor.collison_check(landed_box) {
                    self.box_landed.insert(box_actor.clone());
                    break;
                }
            }
        }

        for (_idx, landed_box) in self.box_landed.iter_mut() {
            if !landed_box.dying && platform_bounded {
                landed_box.velocity = self.platform.velocity
            };

            landed_box.do_motion();
            landed_box.bounds_check(bounds);
            landed_box.collison_check(&self.dropper);
        }

        // todo: should we serparate dying out of landed in to their own collection?

        self.box_actors
            .retain(|_, v| v.landed == !true && v.dead == !true);
        self.box_landed.retain(|_, v| v.dead == !true);

        Ok(())
    }

    fn render(&mut self) -> GameResult {
        graphics::clear(self.ctx, graphics::BLACK);

        for (_idx, box_actor) in self.box_actors.iter() {
            box_actor.draw(self.ctx, &self.box_images)?;
        }

        for (_idx, landed_box) in self.box_landed.iter() {
            landed_box.draw(self.ctx, &self.box_images)?;
        }

        self.platform.draw(self.ctx, &self.box_images)?;
        self.dropper.draw(self.ctx, &self.box_images)?;

        graphics::present(self.ctx)?;

        Ok(())
    }
}

// impl for other stuff that isn't game loop specific
impl StackIt<'_> {
    fn spawn_box(&mut self) {
        let mut box_new = SquareActor::new(Actor::Box);
        box_new.location = self.dropper.location + ((self.dropper.size - box_new.size) / 2.0);
        box_new.velocity = self.dropper.velocity;
        box_new.velocity.y = 2.0;
        box_new.color = random_rgb(&mut self.random);
        box_new.use_image = true;
        box_new.image_id = self.random.gen_range(0, self.box_images.len());

        self.box_actors.insert(box_new);
    }

    fn clear_boxes(&mut self) {
        self.box_actors.iter_mut().for_each(|(_, b)| b.set_dying());
        self.box_landed.iter_mut().for_each(|(_, b)| b.set_dying());
        // self.box_actors.clear();
        // self.box_landed.clear();
    }
}

fn center_window(ctx: &mut ggez::Context) {
    let window = graphics::window(ctx);
    let mut pos = window
        .get_position()
        .expect("Failed to get window position for centering!");
    pos.x = f64::from(SCREEN_WIDTH / 2.0 - WIN_WIDTH / 2.0);
    pos.y = f64::from(SCREEN_HEIGHT / 2.0 - WIN_HEIGHT / 2.0);
    window.set_position(pos);
}

fn random_rgb(rng: &mut rand::rngs::ThreadRng) -> graphics::Color {
    graphics::Color::from_rgb(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>())
}

fn coin_flip(rng: &mut rand::rngs::ThreadRng, val: f32) -> f32 {
    val * if rng.gen::<bool>() { 1.0 } else { -1.0 }
}

fn main() -> GameResult {
    // Context and mostly Window setup
    let window_mode = ggez::conf::WindowMode::default().dimensions(WIN_WIDTH, WIN_HEIGHT);
    let window_setup = ggez::conf::WindowSetup::default()
        .vsync(true)
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
        let delta = time::Instant::now();

        // Window Events
        events_loop.poll_events(|p_event| {
            game.ctx.process_event(&p_event);

            if let Event::WindowEvent { event, .. } = p_event {
                match event {
                    WindowEvent::CloseRequested => event::quit(game.ctx),

                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(keycode), state,
                                .. // scancode: u32, modifiers: ModifiersState
                            },
                        .. // device_id: DeviceId
                    } if state == ElementState::Pressed => match keycode {
                        event::KeyCode::Escape => event::quit(game.ctx),
                        event::KeyCode::D => { game.spawn_box() },
                        event::KeyCode::C => { game.clear_boxes() },
                        _ => { /* https://docs.rs/ggez/0.5.1/ggez/input/keyboard/enum.KeyCode.html */ }
                    },

                    WindowEvent::MouseInput {
                        button, state,
                        .. // device_id: DeviceID, modifiers: ModifiersState
                    } if state == ElementState::Pressed => match button  {
                        ggez::input::mouse::MouseButton::Left => { game.spawn_box() },
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
        // while ggez::timer::check_update_time(game.ctx, 30) {
        game.update()?;
        // }

        // Render
        game.render()?;

        // never accurate sleep, but we just want a cool cpu/gpu
        // thread::sleep(time::Duration::from_nanos(1));
        if time::Duration::from_millis(1000 / 60) > delta.elapsed() {
            let sleep_time = time::Duration::from_millis(1000 / 60) - delta.elapsed();
            sleepy_timey(sleep_time)
        }
    }

    Ok(())
}

#[cfg(windows)]
fn sleepy_timey(sleep_time: time::Duration) {
    let delta = time::Instant::now();

    use std::mem;
    use winapi::um::mmsystem::{TIMECAPS, TIMERR_NOERROR};
    use winapi::um::timeapi::{timeBeginPeriod, timeEndPeriod, timeGetDevCaps};

    let min_slice: winapi::shared::minwindef::UINT = {
        let timecaps_size = mem::size_of::<TIMECAPS>() as u32;
        let mut timecaps = TIMECAPS {
            wPeriodMin: 0,
            wPeriodMax: 0,
        };

        if unsafe { timeGetDevCaps(&mut timecaps, timecaps_size) == TIMERR_NOERROR } {
            timecaps.wPeriodMin
        } else {
            1
        }
    };

    let resolution = time::Duration::new(0, min_slice * 1000000);
    // dbg!(resolution);

    if sleep_time > resolution {
        unsafe {
            timeBeginPeriod(min_slice);
        }
        thread::sleep(sleep_time - resolution);
        unsafe {
            timeEndPeriod(min_slice);
        }
    }

    while delta.elapsed() < sleep_time {
        thread::yield_now();
    }
}

#[cfg(not(windows))]
fn sleepy_timey(sleep_time: std::time::Duration) {
    let delta = time::Instant::now();

    let resolution = time::Duration::new(0, 125_000);

    if sleep_time > resolution {
        thread::sleep(sleep_time - resolution);
    }

    while delta.elapsed() < sleep_time {
        thread::yield_now();
    }
}
