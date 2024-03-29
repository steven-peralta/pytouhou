//! Module providing an Enemy struct, to be changed by EclRunner.

use touhou_formats::th06::anm0::Anm0;
use touhou_formats::th06::ecl::Rank;
use crate::th06::anm0::{Sprite, AnmRunner};
use crate::th06::interpolator::{Interpolator1, Interpolator2};
use touhou_utils::prng::Prng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

/// The 2D position of an object in the game.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Position {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

/// An offset which can be added to a Position.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Offset {
    pub(crate) dx: f32,
    pub(crate) dy: f32,
}

impl Position {
    /// Create said position.
    pub fn new(x: f32, y: f32) -> Position {
        Position { x, y }
    }
}

impl Offset {
    /// Create said offset.
    pub fn new(dx: f32, dy: f32) -> Offset {
        Offset { dx, dy }
    }
}

impl std::ops::Add<Offset> for Position {
    type Output = Position;
    fn add(self, offset: Offset) -> Position {
        Position {
            x: self.x + offset.dx,
            y: self.y + offset.dy,
        }
    }
}

impl std::ops::Sub<Position> for Position {
    type Output = Offset;
    fn sub(self, other: Position) -> Offset {
        Offset {
            dx: other.x - self.x,
            dy: other.y - self.y,
        }
    }
}

type Callback = i32;

#[derive(Debug, Clone)]
/// XXX
pub struct Laser {
    /// XXX
    pub placeholder: u32
}

#[derive(Debug, Clone, Default)]
struct Process;

/// Struct representing the player.
pub struct Player {
    pos: Position,
}

/// Struct representing an enemy bullet.
pub struct Bullet {
    /// Current position of the bullet.
    pub pos: Position,

    /// Current speed of the bullet.
    pub speed: f32,

    /// Current XXX of the bullet.
    pub dpos: [f32; 3],

    /// Current XXX of the bullet.
    pub flags: u32,

    /// Current frame of the bullet.
    pub frame: i32,

    /// Current attributes of the bullet.
    pub attributes: [f32; 2],

    /// TODO: what are the values?
    pub state: i8,
}

/// God struct of our game.
pub struct Game {
    enemies: Vec<Rc<RefCell<Enemy>>>,
    anmrunners: Vec<Rc<RefCell<AnmRunner>>>,
    pub(crate) bullets: Vec<Rc<RefCell<Bullet>>>,
    player: Rc<RefCell<Player>>,
    pub(crate) prng: Rc<RefCell<Prng>>,
    rank: Rank,
    difficulty: i32,
}

impl Game {
    /// Create said god struct.
    pub fn new(prng: Rc<RefCell<Prng>>, rank: Rank) -> Game {
        Game {
            enemies: Vec::new(),
            anmrunners: Vec::new(),
            bullets: Vec::new(),
            player: Rc::new(RefCell::new(Player { pos: Position { x: 192., y: 384. } })),
            prng,
            rank,
            difficulty: 0,
        }
    }

    /// Run the simulation for a single frame.
    pub fn run_frame(&mut self) {
        /*
        for eclrunner in self.eclrunners {
            eclrunner.run_frame();
        }
        */

        for anmrunner in self.anmrunners.iter() {
            let mut anmrunner = anmrunner.borrow_mut();
            anmrunner.run_frame();
        }
    }

    /// Returns a list of all sprites currently being displayed on screen.
    pub fn get_sprites(&self) -> Vec<(f32, f32, f32, Rc<RefCell<Sprite>>)> {
        let mut sprites = vec![];
        for enemy in self.enemies.iter() {
            let enemy = enemy.borrow();
            let anmrunner = enemy.anmrunner.upgrade().unwrap();
            let anmrunner = anmrunner.borrow();
            let sprite = anmrunner.get_sprite();
            sprites.push((enemy.pos.x, enemy.pos.y, enemy.z, sprite));
        }
        sprites
    }

    // TODO: Fix this function so we can stop making Game::bullets pub.
    /*
    /// Apply a function on all bullets.
    pub fn iter_bullets(&mut self, mut f: impl FnMut(Bullet)) {
        self.bullets.iter().map(|bullet| {
            let mut bullet = bullet.borrow_mut();
            f(*bullet)
        });
    }
    */

    pub(crate) fn get_player(&self) -> Rc<RefCell<Player>> {
        self.player.clone()
    }
}

/// Common to all elements in game.
struct Element {
    pos: Position,
    removed: bool,
    anmrunner: AnmRunner,
}

#[derive(PartialEq)]
pub(crate) struct DifficultyCoeffs {
    pub(crate) speed_a: f32,
    pub(crate) speed_b: f32,
    pub(crate) nb_a: i16,
    pub(crate) nb_b: i16,
    pub(crate) shots_a: i16,
    pub(crate) shots_b: i16,
}

impl Default for DifficultyCoeffs {
    fn default() -> DifficultyCoeffs {
        DifficultyCoeffs {
            speed_a: -0.5,
            speed_b: 0.5,
            nb_a: 0,
            nb_b: 0,
            shots_a: 0,
            shots_b: 0,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct BulletAttributes {
    pub(crate) anim: i16,
    pub(crate) sprite_index_offset: i16,
    pub(crate) pos: Position, // Doesn’t have a z field.
    pub(crate) launch_angle: f32,
    pub(crate) angle: f32,
    pub(crate) speed: f32,
    pub(crate) speed2: f32,
    pub(crate) extended_attributes: (i32, i32, i32, i32, f32, f32, f32, f32),
    // unknown: x32,
    pub(crate) bullets_per_shot: i16,
    pub(crate) number_of_shots: i16,
    pub(crate) bullet_type: i16,
    // zero: x32,
    pub(crate) flags: u32,

    /// Which sound to play when the bullet gets fired.
    pub sound: Option<u8>,
}

impl BulletAttributes {
    /// Fire!
    pub fn fire(&mut self) {
        println!("PAN!");
    }
}

#[derive(PartialEq)]
pub(crate) enum Direction {
    Left,
    Center,
    Right,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Center
    }
}

/// The enemy struct, containing everything pertaining to an enemy.
#[derive(Default)]
pub struct Enemy {
    // Common to all elements in game.
    pub(crate) pos: Position,
    pub(crate) removed: bool,
    pub(crate) anmrunner: Weak<RefCell<AnmRunner>>,

    // Specific to enemy.
    // Floats.
    pub(crate) z: f32,
    pub(crate) angle: f32,
    pub(crate) speed: f32,
    pub(crate) rotation_speed: f32,
    pub(crate) acceleration: f32,

    // Ints.
    pub(crate) type_: u32,
    pub(crate) bonus_dropped: u32,
    pub(crate) die_score: u32,
    /// XXX
    pub frame: u32,
    pub(crate) life: u32,
    pub(crate) death_flags: u32,
    pub(crate) current_laser_id: u32,
    pub(crate) low_life_trigger: Option<u32>,
    pub(crate) timeout: Option<u32>,
    pub(crate) remaining_lives: u32,
    bullet_launch_interval: u32,
    bullet_launch_timer: u32,
    pub(crate) death_anim: i32,
    pub(crate) direction: Direction,
    pub(crate) update_mode: u32,

    // Bools.
    pub(crate) visible: bool,
    pub(crate) was_visible: bool,
    pub(crate) touchable: bool,
    pub(crate) collidable: bool,
    pub(crate) damageable: bool,
    pub(crate) boss: bool,
    pub(crate) automatic_orientation: bool,
    pub(crate) delay_attack: bool,
    // Actually part of type_ atm.
    pub(crate) mirror: bool,

    // Tuples.
    pub(crate) difficulty_coeffs: DifficultyCoeffs,
    pub(crate) bullet_attributes: BulletAttributes,
    pub(crate) bullet_offset: Offset,
    pub(crate) movement_dependant_sprites: Option<(u8, u8, u8, u8)>,
    pub(crate) screen_box: Option<(f32, f32, f32, f32)>,

    // Callbacks.
    pub(crate) death_callback: Option<Callback>,
    pub(crate) boss_callback: Option<Callback>,
    pub(crate) low_life_callback: Option<Callback>,
    pub(crate) timeout_callback: Option<Callback>,

    // Laser.
    pub(crate) laser_by_id: HashMap<u32, Laser>,

    // Options.
    // TODO: actually a 8 element array.
    options: Vec<Element>,

    // Interpolators.
    pub(crate) interpolator: Option<Interpolator2<f32>>,
    pub(crate) speed_interpolator: Option<Interpolator1<f32>>,

    // Misc stuff, do we need them?
    pub(crate) anm0: Weak<RefCell<[Anm0; 2]>>,
    process: Rc<RefCell<Process>>,
    pub(crate) game: Weak<RefCell<Game>>,
    pub(crate) prng: Weak<RefCell<Prng>>,
    pub(crate) hitbox_half_size: [f32; 2],
}

impl Enemy {
    /// Create a new enemy.
    pub fn new(pos: Position, life: i16, bonus_dropped: i16, die_score: u32, mirror: bool, anm0: Weak<RefCell<[Anm0; 2]>>, game: Weak<RefCell<Game>>) -> Rc<RefCell<Enemy>> {
        let game_rc = game.upgrade().unwrap();
        let mut enemy = Enemy {
            pos,
            anm0,
            game,
            visible: true,
            // XXX: shouldn’t be u32, since that can be -1.
            bonus_dropped: bonus_dropped as u32,
            die_score,
            life: if life < 0 { 1 } else { life as u32 },
            touchable: true,
            collidable: true,
            damageable: true,
            mirror,
            ..Default::default()
        };
        let mut game = game_rc.borrow_mut();
        enemy.prng = Rc::downgrade(&game.prng);
        let enemy = Rc::new(RefCell::new(enemy));
        game.enemies.push(enemy.clone());
        enemy
    }

    /// Sets the animation to the one indexed by index in the current anm0.
    pub fn set_anim(&mut self, index: u8) {
        let anm0 = self.anm0.upgrade().unwrap();
        let game = self.game.upgrade().unwrap();
        let sprite = Rc::new(RefCell::new(Sprite::new()));
        let anmrunner = AnmRunner::new(anm0, index, sprite, self.prng.clone(), 0);
        let anmrunner = Rc::new(RefCell::new(anmrunner));
        self.anmrunner = Rc::downgrade(&anmrunner);
        (*game.borrow_mut()).anmrunners.push(anmrunner);
    }

    /// Sets the current position of the enemy.
    pub fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        self.pos.x = x;
        self.pos.y = y;
        self.z = z;
    }

    /// Sets the hitbox around the enemy.
    pub fn set_hitbox(&mut self, width: f32, height: f32) {
        self.hitbox_half_size = [width / 2., height / 2.];
    }

    /// Defines the attributes for the next bullet fired, and fire it if delay_attack isn’t set!
    pub fn set_bullet_attributes(&mut self, opcode: u16, anim: i16, sprite_index_offset: i16,
                                 bullets_per_shot: i16, number_of_shots: i16, speed: f32,
                                 speed2: f32, launch_angle: f32, angle: f32, flags: u32) {
        // Get the coeffs for the current difficulty.
        let difficulty = self.get_difficulty() as i16;
        let coeff_nb = self.difficulty_coeffs.nb_a + (self.difficulty_coeffs.nb_b - self.difficulty_coeffs.nb_a) * difficulty / 32;
        let coeff_shots = self.difficulty_coeffs.shots_a + (self.difficulty_coeffs.shots_b - self.difficulty_coeffs.shots_a) * difficulty / 32;
        let coeff_speed = self.difficulty_coeffs.speed_a + (self.difficulty_coeffs.speed_b - self.difficulty_coeffs.speed_a) * difficulty as f32 / 32.;

        let opcode = 67;
        let mut bullet = &mut self.bullet_attributes;

        bullet.anim = anim;
        bullet.bullet_type = opcode - 67;
        bullet.sprite_index_offset = sprite_index_offset;

        bullet.bullets_per_shot = bullets_per_shot + coeff_nb;
        if bullet.bullets_per_shot < 1 {
            bullet.bullets_per_shot = 1;
        }

        bullet.number_of_shots = number_of_shots + coeff_shots;
        if bullet.number_of_shots < 1 {
            bullet.number_of_shots = 1;
        }

        bullet.pos = self.pos + self.bullet_offset;

        bullet.speed = speed + coeff_speed;
        if bullet.speed < 0.3 {
            bullet.speed = 0.3;
        }

        bullet.speed2 = speed2 + coeff_speed / 2.;
        if bullet.speed2 < 0.3 {
            bullet.speed2 = 0.3;
        }

        bullet.launch_angle = launch_angle.atan2(0.);
        bullet.angle = angle;
        bullet.flags = flags;

        if !self.delay_attack {
            bullet.fire();
        }
    }

    /// Sets the bullet launch interval.
    pub(crate) fn set_bullet_launch_interval(&mut self, rand_start: u32, interval: i32) {
        let coeff_interval = interval / 5;
        let difficulty_modifier = coeff_interval + (-coeff_interval * 2) * self.get_difficulty() / 32;
        self.bullet_launch_interval = (interval + difficulty_modifier) as u32;
        if self.bullet_launch_interval > 0 {
            self.bullet_launch_timer = rand_start % self.bullet_launch_interval;
        }
    }

    /// Stubbed for now.
    pub(crate) fn play_sound(&self, sound_index: i32) {
        println!("Playing sound {}!", sound_index);
    }

    /// Stubbed for now.
    pub(crate) fn set_boss(&self, enable: bool) {
        match enable {
            true => println!("Enemy is now boss!"),
            false => println!("Enemy is not boss anymore."),
        }
    }

    /// Run all interpolators and such, and update internal variables once per
    /// frame.
    pub fn update(&mut self) {
        let Position { mut x, mut y } = self.pos;

        let speed = if self.update_mode == 1 {
            let mut speed = 0.;
            if let Some(interpolator) = &self.interpolator {
                let values = interpolator.values(self.frame);
                x = values[0];
                y = values[1];
            }
            if let Some(interpolator) = &self.speed_interpolator {
                let values = interpolator.values(self.frame);
                speed = values[0];
            }
            speed
        } else {
            let speed = self.speed;
            self.speed += self.acceleration;
            self.angle += self.rotation_speed;
            speed
        };

        let dx = self.angle.cos() * speed;
        let dy = self.angle.sin() * speed;
        if self.mirror {
            x -= dx;
        } else {
            x += dx;
        }
        y += dy;

        if let Some((end_left, end_right, left, right)) = self.movement_dependant_sprites {
            if x < self.pos.x && self.direction != Direction::Left {
                self.set_anim(left);
                self.direction = Direction::Left;
            } else if x > self.pos.x && self.direction != Direction::Right {
                self.set_anim(right);
                self.direction = Direction::Right;
            } else if x == self.pos.x && self.direction != Direction::Center {
                let anim = if self.direction == Direction::Left {
                    end_left
                } else {
                    end_right
                };
                self.set_anim(anim);
                self.direction = Direction::Center;
            }
        }

        self.pos = Position { x, y };

        if self.bullet_launch_interval != 0 {
            if self.bullet_launch_timer == 0 {
                self.bullet_attributes.fire();
                self.bullet_launch_timer = self.bullet_launch_interval;
            }
            self.bullet_launch_timer += 1;
            self.bullet_launch_timer %= self.bullet_launch_interval;
        }

        self.frame += 1;
    }

    pub(crate) fn get_rank(&self) -> Rank {
        let game = self.game.upgrade().unwrap();
        let game = game.borrow();
        game.rank
    }

    pub(crate) fn get_difficulty(&self) -> i32 {
        let game = self.game.upgrade().unwrap();
        let game = game.borrow();
        game.difficulty
    }

    // TODO: use a trait for positionable entities.
    pub(crate) fn get_angle_to(&self, player: Rc<RefCell<Player>>) -> f32 {
        let player = player.borrow();
        let offset = self.pos - player.pos;
        offset.dy.atan2(offset.dx)
    }

    pub(crate) fn set_aux_anm(&self, number: i32, script: i32) {
        println!("TODO: Spawn aux anm {} with script {}.", number, script);
    }
}

trait Renderable {
    fn get_sprites(&self) -> Vec<Rc<RefCell<Sprite>>>;
}

impl Renderable for Enemy {
    fn get_sprites(&self) -> Vec<Rc<RefCell<Sprite>>> {
        let anmrunner = self.anmrunner.upgrade().unwrap();
        let anmrunner = anmrunner.borrow();
        vec![anmrunner.get_sprite()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Read};
    use std::fs::File;

    #[test]
    fn enemy() {
        let file = File::open("EoSD/ST/stg1enm.anm").unwrap();
        let mut file = io::BufReader::new(file);
        let mut buf = vec![];
        file.read_to_end(&mut buf).unwrap();
        let (_, mut anms) = Anm0::from_slice(&buf).unwrap();
        let anm0 = anms.pop().unwrap();

        let file = File::open("EoSD/ST/stg1enm2.anm").unwrap();
        let mut file = io::BufReader::new(file);
        let mut buf = vec![];
        file.read_to_end(&mut buf).unwrap();
        let (_, mut anms) = Anm0::from_slice(&buf).unwrap();
        let anm0_bis = anms.pop().unwrap();

        let anm0 = Rc::new(RefCell::new([anm0, anm0_bis]));
        let prng = Rc::new(RefCell::new(Prng::new(0)));
        let game = Game::new(prng, Rank::EASY);
        let game = Rc::new(RefCell::new(game));
        let enemy = Enemy::new(Position::new(0., 0.), 500, 0, 640, Rc::downgrade(&anm0), Rc::downgrade(&game));
        let mut enemy = enemy.borrow_mut();
        assert!(enemy.anmrunner.upgrade().is_none());
        enemy.set_anim(0);
        assert!(enemy.anmrunner.upgrade().is_some());
    }
}
