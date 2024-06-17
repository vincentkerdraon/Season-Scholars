
use bevy::prelude::*;

use super::overlord::Teacher;

#[derive(Event, Debug)]
pub struct CookedEvent {
    pub teacher: Teacher,
    pub food_remaining: FoodRemaining,
}

#[derive(Event, Debug)]
pub struct StudentsEatEvent {
    pub food_remaining: FoodRemaining,
}

#[derive(Event, Debug)]
pub struct TeacherAteEvent {
    pub teacher: Teacher,
    pub food_remaining: FoodRemaining,
}

pub type FoodRemaining = i8;
