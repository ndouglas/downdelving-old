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

    pub fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = (y * self.width) + x;
        self.map[idx as usize] == TileType::Floor
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
        if INPUT.lock().is_key_pressed(VirtualKeyCode::R) {
            self.recreate_map();
        }
        if INPUT.lock().is_key_pressed(VirtualKeyCode::Escape) {
            self.mode = Mode::Exiting;
        }

        // Either render the proposed path or run along it
        if self.mode == Mode::Waiting {
            // Render a mouse cursor
            let mouse_pos = INPUT.lock().mouse_tile(0);
            let mouse_idx = self.point2d_to_index(mouse_pos);
            draw_batch.print_color(
                mouse_pos,
                "X",
                ColorPair::new(RGB::from_f32(0.0, 1.0, 1.0), RGB::from_f32(0.0, 1.0, 1.0)),
            );
            if self.map[mouse_idx as usize] != TileType::Wall {
                let path = a_star_search(self.player_position, mouse_idx, self);
                if path.success {
                    for loc in path.steps.iter().skip(1) {
                        let x = (loc % self.width as usize) as i32;
                        let y = (loc / self.width as usize) as i32;
                        draw_batch.print_color(
                            Point::new(x, y),
                            "*",
                            ColorPair::new(RGB::from_f32(1., 0., 0.), RGB::from_f32(0., 0., 0.)),
                        );
                    }

                    if INPUT.lock().is_mouse_button_pressed(0) {
                        self.mode = Mode::Moving;
                        self.path = path;
                    }
                }
            }
        } else if self.mode == Mode::Moving {
            let mut step = self.path.steps[0] as usize;
            self.path.steps.remove(0);
            if !self.is_exit_valid(
                (step % self.width as usize) as i32,
                (step / self.width as usize) as i32,
            ) {
                let goal = self.path.destination;
                let path = a_star_search(self.player_position, goal, self);
                if path.steps.len() == 0 {
                    self.mode = Mode::Waiting;
                } else {
                    step = path.steps[0] as usize;
                    self.path = path;
                }
            }
            self.player_position = step;
            if self.path.steps.is_empty() {
                self.mode = Mode::Waiting;
            }
        } else if self.mode == Mode::Exiting {
            newrunstate = RunState::DemoMenu {
                menu_selection: gui::DemoMenuSelection::AStarPathfindingDemo,
            };
        }

        // Render the player @ symbol
        let ppos = idx_xy(self.width.try_into().unwrap(), self.player_position);
        draw_batch.print_color(
            Point::from_tuple(ppos),
            "@",
            ColorPair::new(RGB::from_f32(1.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.)),
        );

        // Submit the rendering
        draw_batch.submit(0).expect("Batch error");
        render_draw_buffer(ctx).expect("Render error");

        newrunstate
    }
}

impl BaseMap for DemoState {
    fn is_opaque(&self, idx: usize) -> bool {
        self.map[idx] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let x = (idx % self.width as usize) as i32;
        let y = (idx / self.width as usize) as i32;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - self.width as usize, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + self.width as usize, 1.0))
        };

        // Diagonals
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((idx - self.width as usize) - 1, 1.4));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((idx - self.width as usize) + 1, 1.4));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((idx + self.width as usize) - 1, 1.4));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((idx + self.width as usize) + 1, 1.4));
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let p1 = Point::new(idx1 % self.width as usize, idx1 / self.width as usize);
        let p2 = Point::new(idx2 % self.width as usize, idx2 / self.width as usize);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
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
            Demo::AStarPathfinding => {
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
