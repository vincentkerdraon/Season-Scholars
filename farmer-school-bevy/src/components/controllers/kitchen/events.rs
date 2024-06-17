use crate::model::definitions::*;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct CookEvent {
    pub teacher: Teacher,
    pub food_remaining: FoodRemaining,
}

#[derive(Event, Debug)]
pub struct StudentsEatEvent {
    pub food_remaining: FoodRemaining,
}

#[derive(Event, Debug)]
pub struct TeacherEatEvent {
    pub teacher: Teacher,
    pub food_remaining: FoodRemaining,
}
