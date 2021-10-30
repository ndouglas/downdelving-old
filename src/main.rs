extern crate serde;
use rltk::{GameState, Point, Rltk};
use specs::prelude::*;
use specs::saveload::SimpleMarkerAllocator;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
mod rect;
pub use rect::Rect;
mod damage_system;
mod game_system;
mod gamelog;
mod gui;
pub mod main_game;
pub use main_game::MainGameRunState;
pub mod map_builders;
pub mod random_table;
pub mod raws;
pub mod rex_assets;
pub mod saveload_system;
mod spawner;
pub use game_system::*;
pub mod effects;
#[macro_use]
extern crate lazy_static;
pub mod rng;
pub mod spatial;
mod systems;

const SHOW_MAPGEN_VISUALIZER: bool = false;
const SHOW_FPS: bool = true;

#[derive(PartialEq, Copy, Clone)]
pub enum Demo {
    AStarPathfinding,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    MainMenu {
        menu_selection: gui::MainMenuSelection,
    },
    DemoMenu {
        menu_selection: gui::DemoMenuSelection,
    },
    Demo {
        demo: Demo,
    },
    MainGame {
        runstate: MainGameRunState,
    },
}

pub struct State {
    pub ecs: World,
    main_game_state: main_game::MainGameState,
    dispatcher: Box<dyn systems::UnifiedDispatcher + 'static>,
}

impl State {
    fn run_systems(&mut self) {
        self.dispatcher.run_now(&mut self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    #[allow(clippy::cognitive_complexity)]
    fn tick(&mut self, ctx: &mut Rltk) {
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(0);
        ctx.cls();
        systems::particle_system::update_particles(&mut self.ecs, ctx);

        // Draw the game map unless we're in one of a few states.
        match newrunstate {
            RunState::MainMenu { .. } => {}
            RunState::DemoMenu { .. } => {}
            RunState::Demo { .. } => {}
            RunState::MainGame { runstate } => match runstate {
                MainGameRunState::GameOver { .. } => {}
                _ => {
                    camera::render_camera(&self.ecs, ctx);
                    gui::draw_ui(&self.ecs, ctx);
                }
            },
        }

        match newrunstate {
            RunState::MainGame { .. } => {
                newrunstate = main_game::tick(self, ctx, &newrunstate);
            }

            RunState::MainMenu { .. } => {
                let result = gui::main_menu(self, ctx);
                match result {
                    gui::MainMenuResult::NoSelection { selected } => {
                        newrunstate = RunState::MainMenu {
                            menu_selection: selected,
                        }
                    }
                    gui::MainMenuResult::Selected { selected } => match selected {
                        gui::MainMenuSelection::NewGame => {
                            saveload_system::new_game(&mut self.ecs);
                            self.main_game_state.generate_world_map(&mut self.ecs, 1, 0);
                            newrunstate = RunState::MainGame {
                                runstate: MainGameRunState::MapGeneration,
                            };
                            self.main_game_state.mapgen_next_state = Some(RunState::MainGame {
                                runstate: MainGameRunState::PreRun,
                            });
                        }
                        gui::MainMenuSelection::LoadGame => {
                            saveload_system::load_game(&mut self.ecs);
                            newrunstate = RunState::MainGame {
                                runstate: MainGameRunState::AwaitingInput,
                            };
                            saveload_system::delete_save();
                        }
                        gui::MainMenuSelection::Demos => {
                            newrunstate = RunState::DemoMenu {
                                menu_selection: gui::DemoMenuSelection::Exit,
                            }
                        }
                        gui::MainMenuSelection::Quit => {
                            ::std::process::exit(0);
                        }
                    },
                }
            }
            RunState::DemoMenu { .. } => {
                let result = gui::demo_menu(self, ctx);
                match result {
                    gui::DemoMenuResult::NoSelection { selected } => {
                        newrunstate = RunState::DemoMenu {
                            menu_selection: selected,
                        }
                    }
                    gui::DemoMenuResult::Selected { selected } => match selected {
                        gui::DemoMenuSelection::AStarPathfindingDemo => {
                            newrunstate = RunState::Demo {
                                demo: Demo::AStarPathfinding,
                            };
                        }
                        gui::DemoMenuSelection::Exit => {
                            newrunstate = RunState::MainMenu {
                                menu_selection: gui::MainMenuSelection::Demos,
                            }
                        }
                    },
                }
            }
            RunState::Demo { demo } => match demo {
                Demo::AStarPathfinding => {
                    rltk::console::log("Yep, did the thing.");
                    newrunstate = RunState::DemoMenu {
                        menu_selection: gui::DemoMenuSelection::AStarPathfindingDemo,
                    }
                }
            },
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
        damage_system::delete_the_dead(&mut self.ecs);

        rltk::render_draw_buffer(ctx)
            .map_err(|err| println!("{:?}", err))
            .ok();
        if SHOW_FPS {
            ctx.print(1, 59, &format!("FPS: {}", ctx.fps));
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::simple(80, 60)
        .unwrap()
        .with_title("Downdelving")
        .with_font("vga8x16.png", 8, 16)
        .with_sparse_console(80, 30, "vga8x16.png")
        .with_vsync(false)
        .build()?;
    context.with_post_scanlines(false);
    let mut gs = State {
        ecs: World::new(),
        main_game_state: main_game::MainGameState {
            mapgen_next_state: Some(RunState::MainMenu {
                menu_selection: gui::MainMenuSelection::NewGame,
            }),
            mapgen_index: 0,
            mapgen_history: Vec::new(),
            mapgen_timer: 0.0,
        },
        dispatcher: systems::build(),
    };
    register_all(&mut gs.ecs);
    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());

    raws::load_raws();

    gs.ecs.insert(map::MasterDungeonMap::new());
    gs.ecs.insert(Map::new(1, 64, 64, "New Map"));
    gs.ecs.insert(Point::new(0, 0));
    let player_entity = spawner::player(&mut gs.ecs, 0, 0);
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::MainGame {
        runstate: MainGameRunState::MapGeneration {},
    });
    gs.ecs
        .insert(systems::particle_system::ParticleBuilder::new());
    gs.ecs.insert(rex_assets::RexAssets::new());

    rltk::main_loop(context, gs)
}
