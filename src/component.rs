use bracket_lib::prelude::{FontCharType, RGBA};
use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Position {
    // pub coords: Point,
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGBA,
    pub bg: RGBA,
}

#[derive(Component, Debug)]
pub struct Player {}
