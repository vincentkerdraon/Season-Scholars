use rand::Rng;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StudentCol {
    #[default]
    Left = 1,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Station {
    #[default]
    None,
    StudentLeft,
    StudentCenter,
    StudentRight,
    Welcome,
    Portal,
    Kitchen,
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Season {
    #[default]
    Spring = 1,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        let s = rng.gen_range(1..=4);
        s.into()
    }
}

impl From<usize> for Season {
    fn from(num: usize) -> Self {
        match num {
            1 => Season::Spring,
            2 => Season::Summer,
            3 => Season::Autumn,
            4 => Season::Winter,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Teacher {
    #[default]
    A = 1,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Screen {
    Menu = 1,
    Game,
    GameOverRecap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reaction {
    Long = 1,
    Short,
    Fail,
}

pub type StudentId = i64;
pub type StudentRow = i8;

#[derive(Debug, Default, Clone, Hash)]
pub struct Student {
    pub id: StudentId,
    pub row: StudentRow,
    pub col: StudentCol,
    pub knowledge: Vec<Season>,
}

pub fn station_to_student_col(station: Station) -> StudentCol {
    match station {
        Station::StudentLeft => StudentCol::Left,
        Station::StudentCenter => StudentCol::Center,
        Station::StudentRight => StudentCol::Right,
        _ => panic!(),
    }
}

pub type FoodRemaining = i8;
