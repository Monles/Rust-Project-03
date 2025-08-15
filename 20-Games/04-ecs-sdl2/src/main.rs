use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};

use specs::prelude::*;
use specs_derive::Component;

use std::time::Duration;
use std::collections::VecDeque;

const PLAYER_MOVEMENT_SPEED: i32 = 20;
const FRAME_RATE: u32 = 20;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns the movement delta for this direction
    const fn to_offset(self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }

    /// Check if this direction is horizontal
    const fn is_horizontal(self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
    }

    /// Check if this direction is vertical  
    const fn is_vertical(self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }
}

// ECS Components
#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position(Point);

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    speed: i32,
    direction: Direction,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
struct Sprite {
    spritesheet: usize,
    region: Rect,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct MovementAnimation {
    current_frame: usize,
    up_frames: Vec<Sprite>,
    down_frames: Vec<Sprite>,
    left_frames: Vec<Sprite>,
    right_frames: Vec<Sprite>,
}

// Player marker component to identify the player entity
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
struct Player;

// ECS Systems
struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, (mut positions, velocities): Self::SystemData) {
        for (position, velocity) in (&mut positions, &velocities).join() {
            if velocity.speed > 0 {
                let (dx, dy) = velocity.direction.to_offset();
                position.0 = position.0.offset(dx * velocity.speed, dy * velocity.speed);
            }
        }
    }
}

struct AnimationSystem;

impl<'a> System<'a> for AnimationSystem {
    type SystemData = (
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, MovementAnimation>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, (mut sprites, mut animations, velocities): Self::SystemData) {
        for (sprite, animation, velocity) in (&mut sprites, &mut animations, &velocities).join() {
            // Only animate if moving
            if velocity.speed > 0 {
                animation.current_frame = (animation.current_frame + 1) % 3;
            }

            // Update sprite based on current direction and frame
            let frames = match velocity.direction {
                Direction::Up => &animation.up_frames,
                Direction::Down => &animation.down_frames,
                Direction::Left => &animation.left_frames,
                Direction::Right => &animation.right_frames,
            };

            if let Some(frame) = frames.get(animation.current_frame) {
                *sprite = frame.clone();
            }
        }
    }
}

// Input handling
#[derive(Debug)]
struct InputHandler {
    direction_stack: VecDeque<Direction>,
    horizontal_balance: i32,
    vertical_balance: i32,
}

impl InputHandler {
    fn new() -> Self {
        Self {
            direction_stack: VecDeque::new(),
            horizontal_balance: 0,
            vertical_balance: 0,
        }
    }

    fn press_key(&mut self, direction: Direction) {
        self.direction_stack.retain(|&d| d != direction);
        self.direction_stack.push_front(direction);
        self.update_balance(direction, 1);
    }

    fn release_key(&mut self, direction: Direction) {
        self.direction_stack.retain(|&d| d != direction);
        self.update_balance(direction, -1);
    }

    fn update_balance(&mut self, direction: Direction, delta: i32) {
        match direction {
            Direction::Left => self.horizontal_balance -= delta,
            Direction::Right => self.horizontal_balance += delta,
            Direction::Up => self.vertical_balance -= delta,
            Direction::Down => self.vertical_balance += delta,
        }
    }

    fn get_effective_direction(&self) -> Option<Direction> {
        self.direction_stack
            .iter()
            .find(|&&direction| self.is_direction_active(direction))
            .copied()
    }

    fn is_direction_active(&self, direction: Direction) -> bool {
        if direction.is_horizontal() {
            self.horizontal_balance != 0
        } else if direction.is_vertical() {
            self.vertical_balance != 0
        } else {
            false
        }
    }
}

// Game state
struct Game {
    world: World,
    input_handler: InputHandler,
    movement_system: MovementSystem,
    animation_system: AnimationSystem,
}

impl Game {
    fn new() -> Self {
        let mut world = World::new();
        
        // Register components
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Sprite>();
        world.register::<MovementAnimation>();
        world.register::<Player>();

        Self {
            world,
            input_handler: InputHandler::new(),
            movement_system: MovementSystem,
            animation_system: AnimationSystem,
        }
    }

    fn create_player(&mut self, _textures: &[Texture]) -> Entity {
        let player_spritesheet = 0;
        let player_top_left_frame = Rect::new(0, 0, 26, 36);

        let player_animation = MovementAnimation {
            current_frame: 0,
            up_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Up),
            down_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Down),
            left_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Left),
            right_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Right),
        };

        self.world.create_entity()  // Start creating a new entity in the game world
            .with(Position(Point::new(0, 0)))  // Add a Position component at coordinates (0,0)
            .with(Velocity {  // Add a Velocity component
                speed: 0,  // Initial speed is 0 (not moving)
                direction: Direction::Right  // Facing right initially
            })
            .with(player_animation.right_frames[0].clone())  // Add the first frame of right-facing animation as current sprite; .clone() is needed because we want to own this sprite data
            .with(player_animation)  // Add the complete animation component with all frames
            .with(Player)  // Add the Player marker component
            .build()  // Finally, build the entity and add it to the world
    }

    fn handle_key_event(&mut self, keycode: Keycode, is_pressed: bool) {
        let direction = match keycode {
            Keycode::Left => Direction::Left,
            Keycode::Right => Direction::Right,
            Keycode::Up => Direction::Up,
            Keycode::Down => Direction::Down,
            _ => return,
        };

        if is_pressed {
            self.input_handler.press_key(direction);
        } else {
            self.input_handler.release_key(direction);
        }
    }

    fn update(&mut self) {
        // Create a new scope for storage operations
        {
            // Update player velocity based on input
            let effective_direction = self.input_handler.get_effective_direction();
            
            let mut velocities = self.world.write_storage::<Velocity>();
            let players = self.world.read_storage::<Player>();
            
            for (velocity, _) in (&mut velocities, &players).join() {
                match effective_direction {
                    Some(dir) => {
                        velocity.direction = dir;
                        velocity.speed = PLAYER_MOVEMENT_SPEED;
                    }
                    None => {
                        velocity.speed = 0;
                    }
                }
            }
        } // Storage references are dropped here

        // Run ECS systems
        self.movement_system.run_now(&self.world.res);
        self.animation_system.run_now(&self.world.res);
        self.world.maintain();
    }
}

/// Returns the row of the spritesheet corresponding to the given direction
fn direction_spritesheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}

/// Create animation frames for the standard character spritesheet
fn character_animation_frames(spritesheet: usize, top_left_frame: Rect, direction: Direction) -> Vec<Sprite> {
    let (frame_width, frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);

    let mut frames = Vec::new();
    for i in 0..3 {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                y_offset,
                frame_width,
                frame_height,
            ),
        })
    }

    frames
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    textures: &[Texture],
    world: &World,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let positions = world.read_storage::<Position>();
    let sprites = world.read_storage::<Sprite>();

    for (position, sprite) in (&positions, &sprites).join() {
        let texture = &textures[sprite.spritesheet];
        
        // Treat the center of the screen as the (0, 0) coordinate
        let screen_position = position.0 + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, sprite.region.width(), sprite.region.height());
        
        canvas.copy(texture, sprite.region, screen_rect)?;
    }

    canvas.present();
    Ok(())
}

fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    // Create window and canvas
    let window = video_subsystem
        .window("Game Tutorial ECS", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| format!("Could not initialize video subsystem: {}", e))?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| format!("Could not create canvas: {}", e))?;

    // Load textures
    let texture_creator = canvas.texture_creator();
    let textures = [
        texture_creator.load_texture("assets/bardo.png")?,
    ];

    // Initialize game
    let mut game = Game::new();
    let _player_entity = game.create_player(&textures);
    
    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_counter = 0u8;

    // Game loop
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } 
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(keycode), repeat: false, .. } => {
                    game.handle_key_event(keycode, true);
                }
                Event::KeyUp { keycode: Some(keycode), repeat: false, .. } => {
                    game.handle_key_event(keycode, false);
                }
                _ => {}
            }
        }

        // Update game state
        game.update();

        // Render
        frame_counter = frame_counter.wrapping_add(1);
        let background_color = Color::RGB(frame_counter, 64, 255 - frame_counter);
        render(&mut canvas, background_color, &textures, &game.world)?;

        // Frame rate control
        std::thread::sleep(Duration::from_nanos(1_000_000_000 / u64::from(FRAME_RATE)));
    }

    Ok(())
}