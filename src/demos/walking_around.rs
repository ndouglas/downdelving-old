use crate::demos::Demo;
use crate::gui;
use crate::RunState;
use derivative::Derivative;
use rltk::prelude::*;
use rltk::NavigationPath;
use std::sync::Mutex;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

#[derive(PartialEq, Copy, Clone)]
enum Mode {
    Waiting,
    Moving,
    Exiting,
}

#[derive(Derivative)]
#[derivative(PartialEq, Clone)]
pub struct DemoState {
    #[derivative(PartialEq = "ignore")]
    map: Vec<TileType>,
    player_position: usize,
    #[derivative(PartialEq = "ignore")]
    visible: Vec<bool>,
    mode: Mode,
    #[derivative(PartialEq = "ignore")]
    path: NavigationPath,
    width: i32,
    height: i32,
}

pub fn xy_idx(width: i32, x: i32, y: i32) -> usize {
    (y as usize * width as usize) + x as usize
}

pub fn idx_xy(width: usize, idx: usize) -> (i32, i32) {
    (idx as i32 % width as i32, idx as i32 / width as i32)
}

impl DemoState {
    pub fn new(width: i32, height: i32) -> DemoState {
        let length = width * height;
        let mut state = DemoState {
            map: vec![TileType::Floor; length.try_into().unwrap()],
            player_position: xy_idx(width as i32, width as i32 / 2, height as i32 / 2),
            visible: vec![false; length.try_into().unwrap()],
            mode: Mode::Waiting,
            path: NavigationPath::new(),
            width: width,
            height: height,
        };
        state.recreate_map();

        state
    }

    pub fn recreate_map(&mut self) {
        let length = self.width * self.height;
        self.map = vec![TileType::Floor; length.try_into().unwrap()];

        for x in 0..self.width {
            self.map[xy_idx(self.width, x, 0)] = TileType::Wall;
            self.map[xy_idx(self.width, x, self.height - 1)] = TileType::Wall;
        }
        for y in 0..self.height {
            self.map[xy_idx(self.width, 0, y)] = TileType::Wall;
            self.map[xy_idx(self.width, self.width - 1, y)] = TileType::Wall;
        }

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..length / 2 {
            let x = rng.range(1, self.width - 1);
            let y = rng.range(1, self.height - 1);
            let idx = xy_idx(self.width, x, y);
            if self.player_position != idx {
                self.map[idx] = TileType::Wall;
            }
        }
    }

    pub fn move_player(&mut self, delta_x: i32, delta_y: i32) {
        self.mode = Mode::Moving;
        let current_position = idx_xy(self.width.try_into().unwrap(), self.player_position);
        let new_position = (current_position.0 + delta_x, current_position.1 + delta_y);
        let new_idx = xy_idx(self.width, new_position.0, new_position.1);
        if self.map[new_idx] == TileType::Floor {
            self.player_position = new_idx;
        }
        self.mode = Mode::Waiting;
    }

    fn tick(&mut self, ctx: &mut BTerm, runstate: &RunState) -> RunState {
        let mut newrunstate = *runstate;

        // We'll use batched drawing
        let mut draw_batch = DrawBatch::new();

        // Set all tiles to not visible
        for v in &mut self.visible {
            *v = false;
        }

        // Obtain the player's visible tile set, and apply it
        let player_position = self.index_to_point2d(self.player_position);
        let fov = field_of_view_set(player_position, 8, self);

        // Note that the steps above would generally not be run every frame!
        for idx in &fov {
            self.visible[xy_idx(self.width, idx.x, idx.y)] = true;
        }

        // Clear the screen
        draw_batch.cls();

        // Iterate the map array, incrementing coordinates as we go.
        let mut y = 0;
        let mut x = 0;
        for (i, tile) in self.map.iter().enumerate() {
            // Render a tile depending upon the tile type; now we check visibility as well!
            let mut fg;
            let mut glyph = ".";

            match tile {
                TileType::Floor => {
                    fg = RGB::from_f32(0.5, 0.5, 0.0);
                }
                TileType::Wall => {
                    fg = RGB::from_f32(0.0, 1.0, 0.0);
                    glyph = "#";
                }
            }
            if !self.visible[i] {
                fg = fg.to_greyscale();
            }
            draw_batch.print_color(
                Point::new(x, y),
                glyph,
                ColorPair::new(fg, RGB::from_f32(0., 0., 0.)),
            );

            // Move the coordinates
            x += 1;
            if x > self.width - 1 {
                x = 0;
                y += 1;
            }
        }

        match ctx.key {
            None => {}
            Some(key) => {
                match key {
                    VirtualKeyCode::R => self.recreate_map(),
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

        if self.mode == Mode::Exiting {
            newrunstate = RunState::DemoMenu {
                menu_selection: gui::DemoMenuSelection::WalkingAroundDemo,
            };
        }

        newrunstate
    }
}

impl BaseMap for DemoState {
    fn is_opaque(&self, idx: usize) -> bool {
        self.map[idx] == TileType::Wall
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
            Demo::WalkingAround => {
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
