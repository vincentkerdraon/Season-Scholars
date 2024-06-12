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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    None,
    Up,
    UpRight,
    Right,
    RightDown,
    Down,
    DownLeft,
    Left,
    LeftUp
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Teacher {
    A,B
}
