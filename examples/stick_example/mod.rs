// Copyright (c) 2017 Ivo Wetzel

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


// External Dependencies ------------------------------------------------------
use lean::Vec2;
use lean::library::{
    Collider, StickFigure, StickFigureConfig, Scarf, Weapon, Renderer
};


// Internal Dependencies ------------------------------------------------------
use super::Context;


// Modules --------------------------------------------------------------------
mod player;
use self::player::{Player, PlayerState};


// Example Code ---------------------------------------------------------------
pub struct Level {
    pub width: f32,
    pub floor: f32
}

impl Level {

    fn draw(&mut self, context: &mut Context) {
        context.line(0.0, self.floor + 1.0, self.width, self.floor + 1.0, 0x00c0_c0c0);
    }

}

impl Collider for Level {

    fn world(&self, mut p: Vec2) -> Option<(Vec2, i32, i32)> {
        let floor = self.floor;
        let (mut horizontal, mut vertical) = (0, 0);

        if p.y > floor {
            p.y = p.y.min(floor);
            vertical = 1;
        }

        if p.x < 0.0 {
            p.x = p.x.max(0.0);
            horizontal = -1;

        } else if p.x > self.width  {
            p.x = p.x.min(self.width);
            horizontal = 1;
        }

        if horizontal != 0 || vertical != 0 {
            Some((p, horizontal, vertical))

        } else {
            None
        }
    }

}

pub struct Example {
    player: Player,
    figure: StickFigure<PlayerState, Context, Level>,
    level: Level,
    show_bounds: bool,
    input_direction: f32
}

impl Example {

    pub fn new(width: f32, height: f32) -> Self {

        let config = StickFigureConfig {

            acceleration: 0.70,
            acceleration_max: 3.5,

            velocity_damping: 0.7,
            velocity_backwards_factor: 0.5,

            jump_force: 5.5,
            fall_speed: 0.25,
            fall_limit: 4.5,

            offset: Vec2::new(0.0, -25.0),
            shoulder_height: 25.0,
            line_of_sight_length: 80.0,

            leanback_min: -45.0,
            leanback_max: 35.0,
            leanback_head_factor: 1.45,

            // TODO move to weapon
            recoil_leanback_factor: 2.0,
            recoil_force: 7.0,
            recoil_damping: 0.9,

            idle_compression: 1.25,
            idle_speed: 5.0,

            land_compression: 10.0,
            land_compression_factor: 0.99,
            land_speed: 11.5,

            run_compression: 1.5,
            run_speed: 16.0,

            crouching_factor: 0.5,
            crouch_compression: 3.0,
            crouch_speed: 1.0

        };

        let player = Player::new(config.clone());
        let mut figure = StickFigure::default(player.get_state(), config);
        figure.add_accessory("Scarf", "Back", Scarf::new(24.0, 6, 0x00ff_ff00));
        figure.add_accessory("Weapon", "Back", Weapon::default(0x00ff_ff00));

        Self {
            player: player,
            figure: figure,
            level: Level {
                width,
                floor: height * 0.75
            },
            show_bounds: false,
            input_direction: 0.0
        }

    }

    pub fn update(
        &mut self,
        mouse_pos: Option<(f32, f32)>,
        left: bool,
        right: bool,
        crouch: bool,
        jump: bool,
        fire: bool,
        kill: bool,
        reset: bool,
        release: bool,
        pickup: bool,
        bounds: bool
    ) {

        if let Some((x, y)) = mouse_pos {
            self.input_direction = self.player.compute_view_angle(Vec2::new(x, y));
        }

        if kill {
            self.player.set_hp(0);
        }

        if reset {
            self.player.set_hp(255);
        }

        if release {
            self.figure.detach("Weapon");

        } else if pickup {
            self.figure.attach("Weapon");
        }

        if bounds {
            self.show_bounds = !self.show_bounds;
        }

        self.player.update_server(fire);
        self.player.update_shared(left, right, crouch, jump, self.input_direction, &self.level);

    }

    pub fn draw(&mut self, context: &mut Context) {

        self.figure.set_state(self.player.get_state());
        self.figure.draw(context, &self.level);
        self.level.draw(context);

        if self.show_bounds {
            let b = self.figure.world_bounds();
            context.draw_rect(b.0, b.1, if self.figure.skeleton().at_rest() { 0x0000_c0f0 } else { 0x00ff_0000 });
        }

    }

}

