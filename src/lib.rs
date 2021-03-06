// Copyright (c) 2017 Ivo Wetzel

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


// Crates ---------------------------------------------------------------------
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate downcast_rs;


// Enums ----------------------------------------------------------------------
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Space {
    World,
    Local,
    Animation
}


// Exports --------------------------------------------------------------------
mod util;
pub use self::util::*;

mod animation;
pub use self::animation::{Animator, AnimatorBuilder, AnimationData};

mod particle;
pub use self::particle::{
    Constraint, ConstraintType, AngularConstraint, StickConstraint,
    Particle, ParticleSystem, ParticleTemplate
};

pub mod library;

mod ragdoll;
pub use self::ragdoll::Ragdoll;

mod rigid_body;
pub use self::rigid_body::{RigidBodyData, RigidBody};

mod skeleton;
pub use self::skeleton::{SkeletalData, SkeletalConstraint, Skeleton};

