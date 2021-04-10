use bracket_lib::prelude::{BError, BTerm, BTermBuilder, RGB, RGBA, *};
use specs::prelude::*;

mod map;
mod player;
mod component;
pub use map::*;
pub use player::*;
pub use component::*;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State { ecs: World::new() };

    gs.ecs.insert(new_map());

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    let yellow = RGBA::from(RGB::named(YELLOW));
    let black = RGBA::from(RGB::named(BLACK));
    gs.ecs
        .create_entity()
        .with(Player {})
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: yellow,
            bg: black,
        })
        .build();

    main_loop(context, gs)
}

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
