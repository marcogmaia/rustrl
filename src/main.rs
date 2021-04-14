use bracket_lib::prelude::{BError, BTerm, BTermBuilder, RGB, RGBA, *};
use specs::prelude::*;

mod component;
mod map;
mod player;
// mod camera;
// pub use camera::*;
pub use component::*;
pub use map::*;
pub use player::*;

bracket_terminal::embedded_resource!(TILE_FONT, "../assets/terminal16x16.png");
fn main() -> BError {
    bracket_terminal::link_resource!(TILE_FONT, "resources/terminal16x16.png");

    let map_size = (64, 48);

    let mut context: BTerm = BTermBuilder::new()
        .with_title("MaiaRL")
        .with_font("terminal16x16.png", 16, 16)
        .with_simple_console(map_size.0, map_size.1, "terminal16x16.png")
        .with_tile_dimensions(16, 16)
        .with_dimensions(map_size.0, map_size.1)
        .with_fps_cap(240.0)
        .with_automatic_console_resize(true)
        .with_fullscreen(false)
        .build()?;
    context.set_active_console(0);


    let mut gs = State { ecs: World::new() };

    let mut map = map::new_map();

    map.new_map_rooms_and_corridors(map_size);
    let initial_pos = map.rooms[0].center();
    gs.ecs.insert(map);
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    let yellow = RGBA::from_f32(1., 1., 0., 1.);
    let black = RGBA::from_f32(0., 0., 0., 0.);

    gs.ecs
        .create_entity()
        .with(Player {})
        .with(Position {
            x: initial_pos.x,
            y: initial_pos.y,
        })
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

        // camera::render_camera(ecs: &World, ctx: &mut Rltk)

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();
        // draw_map(&map, ctx);
        map.draw(ctx);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
