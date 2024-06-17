use crate::model::definitions::*;
use bevy::prelude::*;

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
