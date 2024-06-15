#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Season {
    #[default]
    Spring = 1,
    Summer,
    Autumn,
    Winter,
}

// pub enum Direction {
//     None = 0,
//     Up = 1,
//     Right = 2,
//     Down = 4,
//     Left = 8,
//     UpRight = 3,
//     RightDown = 6,
//     DownLeft = 12,
//     LeftUp = 9,
// }

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
