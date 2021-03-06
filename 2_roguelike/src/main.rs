#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod blacksheepwall;
mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}
use prelude::*;

pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

struct State {
    ecs: World,
    resources: Resources,

    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        let mut ecs = World::default();
        let mut resources = Resources::default();
        spawn_player(&mut ecs, map_builder.player_start);
        // spawn_amulet(&mut self.ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        spawn_level(
            &mut ecs,
            &mut rng,
            &mut resources,
            0,
            &map_builder.monster_spawns,
        );
        resources.insert(map_builder.map);

        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.theme);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            monster_systems: build_monster_scheduler(),
            player_systems: build_player_scheduler(),
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs, map_builder.player_start);
        // spawn_amulet(&mut self.ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        spawn_level(
            &mut self.ecs,
            &mut rng,
            &mut self.resources,
            0,
            &map_builder.monster_spawns,
        );
        self.resources.insert(map_builder.map);

        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "You Died");
        ctx.print_color_centered(42, GREEN, BLACK, "Press 1 to play again.");

        if ctx.key == Some(VirtualKeyCode::Key1) {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "You won!");
        ctx.print_color_centered(42, GREEN, BLACK, "Press 1 to play again.");

        if ctx.key == Some(VirtualKeyCode::Key1) {
            self.reset_game_state();
        }
    }
    fn pause(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Paused. Press ESCAPE to resume");

        if ctx.key == Some(VirtualKeyCode::Tab) {
            // get the map, start, amulet start, and monster spawns
            let map = self.resources.get::<Map>().unwrap();
            let mut player = <&Point>::query().filter(component::<Player>());
            let player_pos = player.iter(&self.ecs).next().unwrap();
            let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
            let amulet_default = Point::new(-1, -1);
            let amulet_pos = amulet.iter(&self.ecs).next().unwrap_or(&amulet_default);
            let mut monsters = <(&Enemy, &Point)>::query();
            let monster_pos = monsters
                .iter(&self.ecs)
                .map(|(_, point)| *point)
                .collect::<Vec<Point>>();
            let mut items = <(&Item, &Point, &Name)>::query();

            let item_pos = items
                .iter(&self.ecs)
                .map(|(_, point, name)| (*point, name))
                .collect::<Vec<(Point, &Name)>>();
            blacksheepwall::display("Map", &map, player_pos, amulet_pos, &monster_pos, &item_pos);
        } else if ctx.key == Some(VirtualKeyCode::Escape) {
            ctx.set_active_console(0);
            ctx.cls();
            self.resources.insert(TurnState::AwaitingInput);
        }
    }

    /**
    Remove all entities from the ECS World that aren???t either the player or items carried by the player.

    Set the is_dirty flag on the player???s FieldOfView to ensure that the map renders correctly on the next turn.

    Generate a new level as you did before.

    Check the current level number: if it???s 0 or 1, spawn an exit staircase; if it???s 2, spawn the Amulet of Yala.

    Finish setting up spawned monsters and resources as you did before.
             */
    fn advance_level(&mut self) {
        use std::collections::HashSet;
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&self.ecs)
            .next()
            .unwrap();

        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);

        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_e, carry)| carry.0 == player_entity)
            .map(|(e, _carry)| *e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });

        let mut cb = CommandBuffer::new(&self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }
        cb.flush(&mut self.ecs, &mut self.resources);

        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });

        if map_level == 2 {
            spawn_amulet(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        spawn_level(
            &mut self.ecs,
            &mut rng,
            &mut self.resources,
            map_level as usize,
            &map_builder.monster_spawns,
        );
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));

        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        // q: "we know for sure a turn state exists - skip error checking"
        // _we_ know, but how does the program know?
        // almost got `if let` working, to avoid explicit unwrap and possible
        // runtime panic
        // > if let Some(current_state) = self.resources.get::<TurnState>() {
        let current_state = *self.resources.get::<TurnState>().unwrap();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => {
                self.game_over(ctx);
            }
            TurnState::Victory => {
                self.victory(ctx);
            }
            TurnState::NextLevel => self.advance_level(),
            TurnState::Paused => self.pause(ctx),
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Roguelike")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;
    main_loop(context, State::new())
}
