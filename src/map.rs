mod glyphs;

use std::{
    cmp::{max, min},
    usize,
};

use bracket_lib::prelude::{to_cp437, BTerm, RandomNumberGenerator, Rect, RGBA};
// use super::Rect;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub rect: Rect,
    pub map: Vec<TileType>,
    pub rooms: Vec<Rect>,
}

pub fn new_map() -> Map {
    Map {
        rect: Rect::with_size(0, 0, 0, 0),
        map: Vec::new(),
        rooms: Vec::new(),
    }
}

impl Map {
    pub fn new_map_rooms_and_corridors(&mut self, (w, h): (i32, i32)) {
        self.rect = Rect::with_size(0, 0, w, h);
        self.map = vec![TileType::Wall; w as usize * h as usize];
        self.rooms = Vec::new();

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _i in 0..MAX_ROOMS {
            let rw = rng.range(MIN_SIZE, MAX_SIZE);
            let rh = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.range(1, w - rw - 1);
            let y = rng.range(1, h - rh - 1);
            let new_room = Rect::with_size(x, y, rw, rh);
            let mut ok = true;
            for other_room in self.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                self.apply_room_to_map(&new_room);

                if !self.rooms.is_empty() {
                    let p_new = new_room.center();
                    let p_prev = self.rooms[self.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        self.apply_horizontal_tunnel(p_prev.x, p_new.x, p_prev.y);
                        self.apply_vertical_tunnel(p_prev.y, p_new.y, p_new.x);
                    } else {
                        self.apply_vertical_tunnel(p_prev.y, p_new.y, p_prev.x);
                        self.apply_horizontal_tunnel(p_prev.x, p_new.x, p_new.y);
                    }
                }

                self.rooms.push(new_room);
            }
        }
    }

    pub fn apply_room_to_map(&mut self, room: &Rect) {
        for x in room.x1..=room.x2 {
            for y in room.y1..=room.y2 {
                self.map[xy_idx(self.rect, x, y)] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = xy_idx(self.rect, x, y);
            if idx > 0 && idx < 80 * 50 {
                self.map[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = xy_idx(self.rect, x, y);
            if idx > 0 && idx < self.area() {
                self.map[idx as usize] = TileType::Floor;
            }
        }
    }

    fn area(&self) -> usize {
        (self.rect.width() * self.rect.height()) as usize
    }

    pub fn draw(&self, ctx: &mut BTerm) {
        let mut y = 0;
        let mut x = 0;
        for tile in self.map.iter() {
            // Render a tile depending upon the tile type
            match tile {
                TileType::Floor => {
                    ctx.set(
                        x,
                        y,
                        RGBA::from_u8(36, 18, 4, 255),
                        // RGBA::from_f32(0.5, 0.5, 0.5, 1.),
                        RGBA::from_f32(0., 0., 0., 1.),
                        // to_cp437('·'),
                        glyphs::BLOCK1,
                    );
                }
                TileType::Wall => {
                    ctx.set(
                        x,
                        y,
                        RGBA::from_u8(0, 31, 36, 255),
                        RGBA::from_f32(0., 0., 0., 1.),
                        // to_cp437('#'),
                        glyphs::SOLID1,
                    );
                }
            }

            // Move the coordinates
            x += 1;
            if x >= self.rect.x2 {
                x = 0;
                y += 1;
            }
        }
    }

    pub fn at(&self, x: i32, y: i32) -> TileType {
        self.map[xy_idx(self.rect, x, y)]
    }
}

pub fn xy_idx(rect: Rect, x: i32, y: i32) -> usize {
    (x + (y * rect.width())) as usize
}
