// src/model/enums.rs

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StudentCols {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Station {
    None,
    StudentLeft,
    StudentCenter,
    StudentRight,
    Welcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Season {
    Spring,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Teacher {
    A = 2,
    B,
}
