// src/model/enums.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StudentCols {
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
