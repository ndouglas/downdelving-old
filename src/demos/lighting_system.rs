use crate::demos::Demo;
use crate::gui;
use crate::map;
use crate::map_builders::level_builder;
use crate::perception::field_of_view::field_of_view as shadowcasting_fov;
use crate::RunState;
use derivative::Derivative;
use map::{get_tile_renderable, TileType};
use rltk::prelude::field_of_view as bracket_fov;
use rltk::prelude::*;
use rltk::NavigationPath;
use std::fmt::{self, Debug};
use std::sync::Mutex;

#[derive(PartialEq, Copy, Clone)]
enum Mode {
    Waiting,
    Moving,
    Exiting,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum FovAlgorithm {
    Bracket,
    Shadowcasting,
}

impl fmt::Display for FovAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Derivative)]
#[derivative(PartialEq, Clone)]
pub struct DemoState {
    #[derivative(PartialEq = "ignore")]
    map: map::Map,
    player_position: usize,
    mode: Mode,
    #[derivative(PartialEq = "ignore")]
    path: NavigationPath,
    width: i32,
    height: i32,
    fov_algorithm: FovAlgorithm,
}

pub fn xy_idx(width: i32, x: i32, y: i32) -> usize {
    (y as usize * width as usize) + x as usize
}

pub fn idx_xy(width: usize, idx: usize) -> (i32, i32) {
    (idx as i32 % width as i32, idx as i32 / width as i32)
}

impl DemoState {
    pub fn new(width: i32, height: i32) -> DemoState {
        let mut builder = level_builder(47, width, height);
        builder.build_map();
        let position = builder
            .build_data
            .starting_position
            .as_mut()
            .unwrap()
            .clone();
        let mut state = DemoState {
            map: builder.build_data.map,
            player_position: xy_idx(width, position.x, position.y),
            mode: Mode::Waiting,
            path: NavigationPath::new(),
            width: width,
            height: height,
            fov_algorithm: FovAlgorithm::Bracket,
        };
        state.recreate_map();

        state
    }

    pub fn recreate_map(&mut self) {
        let mut builder = level_builder(47, self.width, self.height);
        builder.build_map();
        self.map = builder.build_data.map;
        let position = builder
            .build_data
            .starting_position
            .as_mut()
            .unwrap()
            .clone();
        self.player_position = xy_idx(self.width, position.x, position.y);
    }

    pub fn switch_fov_algorithm(&mut self) {
        self.fov_algorithm = match self.fov_algorithm {
            FovAlgorithm::Bracket => FovAlgorithm::Shadowcasting,
            FovAlgorithm::Shadowcasting => FovAlgorithm::Bracket,
        }
    }

    pub fn move_player(&mut self, delta_x: i32, delta_y: i32) {
        self.mode = Mode::Moving;
        let current_position = idx_xy(self.width.try_into().unwrap(), self.player_position);
        let new_position = (current_position.0 + delta_x, current_position.1 + delta_y);
        let new_idx = xy_idx(self.width, new_position.0, new_position.1);
        if self.map.tiles[new_idx] == TileType::Floor {
            self.player_position = new_idx;
        }
        self.mode = Mode::Waiting;
    }

    fn tick(&mut self, ctx: &mut BTerm, runstate: &RunState) -> RunState {
        let mut newrunstate = *runstate;

        // We'll use batched drawing
        let mut draw_batch = DrawBatch::new();

        // Set all tiles to not visible
        for v in &mut self.map.visible_tiles {
            *v = false;
        }

        // Obtain the player's visible tile set, and apply it
        let player_position = self.index_to_point2d(self.player_position);
        let fov = match self.fov_algorithm {
            FovAlgorithm::Bracket => bracket_fov(player_position, 8, self),
            FovAlgorithm::Shadowcasting => {
                shadowcasting_fov(player_position.x, player_position.y, 8, &self.map)
            }
        };

        // Note that the steps above would generally not be run every frame!
        for idx in &fov {
            self.map.visible_tiles[xy_idx(self.width, idx.x, idx.y)] = true;
        }

        // Clear the screen
        draw_batch.cls();

        // Iterate the map array, incrementing coordinates as we go.
        for (i, _tile) in self.map.tiles.iter().enumerate() {
            let xy = idx_xy(self.width as usize, i);
            if xy.1 == 0 {
                continue;
            }
            let glyph = get_tile_renderable(i, &self.map);
            let mut fg = glyph.1;
            if !self.map.visible_tiles[i] {
                fg = fg.to_greyscale();
            }
            draw_batch.set(Point::new(xy.0, xy.1), ColorPair::new(fg, glyph.2), glyph.0);
        }

        match ctx.key {
            None => {}
            Some(key) => {
                match key {
                    VirtualKeyCode::R => self.recreate_map(),
                    VirtualKeyCode::F => self.switch_fov_algorithm(),
                    VirtualKeyCode::Escape => self.mode = Mode::Exiting,

                    // Cursors
                    VirtualKeyCode::Up => self.move_player(0, -1),
                    VirtualKeyCode::Down => self.move_player(0, 1),
                    VirtualKeyCode::Left => self.move_player(-1, 0),
                    VirtualKeyCode::Right => self.move_player(1, 0),

                    _ => {} // Ignore all the other possibilities
                }
            }
        }

        let ppos = idx_xy(self.width.try_into().unwrap(), self.player_position);
        draw_batch.print_color(
            Point::from_tuple(ppos),
            "@",
            ColorPair::new(RGB::from_f32(1.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.)),
        );

        draw_batch.submit(0).expect("Batch error");
        render_draw_buffer(ctx).expect("Render error");
        ctx.print(
            1,
            0,
            &format!("[F] FOV Algorithm {}", self.fov_algorithm.to_string()),
        );

        if self.mode == Mode::Exiting {
            newrunstate = RunState::DemoMenu {
                menu_selection: gui::DemoMenuSelection::FieldOfViewDemo,
            };
        }

        newrunstate
    }
}

impl BaseMap for DemoState {
    fn is_opaque(&self, idx: usize) -> bool {
        self.map.tiles[idx] == TileType::Wall
    }
}

impl Algorithm2D for DemoState {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

lazy_static! {
    static ref DEMO_STATE: Mutex<DemoState> = Mutex::new(DemoState::new(80, 60));
}

#[allow(clippy::cognitive_complexity)]
pub fn tick(ctx: &mut Rltk, runstate: &RunState) -> RunState {
    let mut newrunstate = *runstate;
    match runstate {
        RunState::Demo { demo } => match demo {
            Demo::LightingSystem => {
                if DEMO_STATE.lock().unwrap().mode == Mode::Exiting {
                    DEMO_STATE.lock().unwrap().mode = Mode::Waiting;
                    DEMO_STATE.lock().unwrap().recreate_map();
                }
                newrunstate = DEMO_STATE.lock().unwrap().tick(ctx, runstate);
            }
            _ => {}
        },
        _ => {}
    }

    newrunstate
}
