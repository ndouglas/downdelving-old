use rltk::Rltk;
use specs::prelude::*;

use crate::camera;
use crate::components::*;
use crate::gamelog;
use crate::gui;
use crate::map;
use crate::map::{freeze_level_entities, Map, MasterDungeonMap};
use crate::player;
use crate::player::*;
use crate::saveload_system;
use crate::spawner;
use crate::vendor;
use crate::vendor::VendorMode;
use crate::RunState;
use crate::State;
use crate::SHOW_MAPGEN_VISUALIZER;

#[derive(PartialEq, Copy, Clone)]
pub enum MainGameRunState {
    AwaitingInput,
    PreRun,
    Ticking,
    ShowInventory,
    ShowDropItem,
    ShowTargeting { range: i32, item: Entity },
    SaveGame,
    NextLevel,
    PreviousLevel,
    TownPortal,
    ShowRemoveItem,
    GameOver,
    MagicMapReveal { row: i32 },
    MapGeneration,
    ShowCheatMenu,
    ShowVendor { vendor: Entity, mode: VendorMode },
    TeleportingToOtherLevel { x: i32, y: i32, depth: i32 },
    ShowRemoveCurse,
    ShowIdentify,
}

pub struct MainGameState {
    pub mapgen_next_state: Option<RunState>,
    pub mapgen_history: Vec<Map>,
    pub mapgen_index: usize,
    pub mapgen_timer: f32,
}

impl MainGameState {
    pub fn goto_level(&mut self, ecs: &mut World, offset: i32) {
        freeze_level_entities(ecs);

        // Build a new map and place the player
        let current_depth = ecs.fetch::<Map>().depth;
        self.generate_world_map(ecs, current_depth + offset, offset);

        // Notify the player
        gamelog::Logger::new().append("You change level.").log();
    }

    pub fn game_over_cleanup(&mut self, ecs: &mut World) {
        // Delete everything
        let mut to_delete = Vec::new();
        for e in ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            ecs.delete_entity(*del).expect("Deletion failed");
        }

        // Spawn a new player
        {
            let player_entity = spawner::player(ecs, 0, 0);
            let mut player_entity_writer = ecs.write_resource::<Entity>();
            *player_entity_writer = player_entity;
        }

        // Replace the world maps
        ecs.insert(map::MasterDungeonMap::new());

        // Build a new map and place the player
        self.generate_world_map(ecs, 1, 0);
    }

    pub fn generate_world_map(&mut self, ecs: &mut World, new_depth: i32, offset: i32) {
        self.mapgen_index = 0;
        self.mapgen_timer = 0.0;
        self.mapgen_history.clear();
        let map_building_info = map::level_transition(ecs, new_depth, offset);
        if let Some(history) = map_building_info {
            self.mapgen_history = history;
        } else {
            map::thaw_level_entities(ecs);
        }

        gamelog::clear_log();
        gamelog::Logger::new()
            .append("Welcome to")
            .color(rltk::YELLOW)
            .append("Downdelving")
            .log();

        gamelog::clear_events();
    }
}

#[allow(clippy::cognitive_complexity)]
pub fn tick(state: &mut State, ctx: &mut Rltk, runstate: &RunState) -> RunState {
    let mut newrunstate = *runstate;

    // Draw the game map unless we're in one of a few states.
    match newrunstate {
        RunState::MainGame {
            runstate: inner_runstate,
        } => match inner_runstate {
            MainGameRunState::GameOver { .. } => {}
            _ => {
                camera::render_camera(&state.ecs, ctx);
                gui::draw_ui(&state.ecs, ctx);
            }
        },
        _ => {}
    }

    match newrunstate {
        RunState::MainGame {
            runstate: inner_runstate,
        } => match inner_runstate {
            MainGameRunState::MapGeneration => {
                if !SHOW_MAPGEN_VISUALIZER {
                    newrunstate = state.main_game_state.mapgen_next_state.unwrap();
                } else {
                    ctx.cls();
                    if state.main_game_state.mapgen_index
                        < state.main_game_state.mapgen_history.len()
                        && state.main_game_state.mapgen_index
                            < state.main_game_state.mapgen_history.len()
                    {
                        camera::render_debug_map(
                            &state.main_game_state.mapgen_history
                                [state.main_game_state.mapgen_index],
                            ctx,
                        );
                    }

                    state.main_game_state.mapgen_timer += ctx.frame_time_ms;
                    if state.main_game_state.mapgen_timer > 250.0 {
                        state.main_game_state.mapgen_timer = 0.0;
                        state.main_game_state.mapgen_index += 1;
                        if state.main_game_state.mapgen_index
                            >= state.main_game_state.mapgen_history.len()
                        {
                            //state.main_game_state.mapgen_index -= 1;
                            newrunstate = state.main_game_state.mapgen_next_state.unwrap();
                        }
                    }
                }
            }

            MainGameRunState::PreRun => {
                state.run_systems();
                state.ecs.maintain();
                newrunstate = RunState::MainGame {
                    runstate: MainGameRunState::AwaitingInput,
                };
            }

            MainGameRunState::AwaitingInput => {
                newrunstate = player_input(state, ctx);
                if newrunstate
                    != (RunState::MainGame {
                        runstate: MainGameRunState::AwaitingInput,
                    })
                {
                    crate::gamelog::record_event("Turn", 1);
                }
            }

            MainGameRunState::Ticking => {
                let mut should_change_target = false;
                while newrunstate
                    == (RunState::MainGame {
                        runstate: MainGameRunState::Ticking,
                    })
                {
                    state.run_systems();
                    state.ecs.maintain();
                    match *state.ecs.fetch::<RunState>() {
                        RunState::MainGame {
                            runstate: MainGameRunState::AwaitingInput,
                        } => {
                            newrunstate = RunState::MainGame {
                                runstate: MainGameRunState::AwaitingInput,
                            };
                            should_change_target = true;
                        }
                        RunState::MainGame {
                            runstate: MainGameRunState::MagicMapReveal { .. },
                        } => {
                            newrunstate = RunState::MainGame {
                                runstate: MainGameRunState::MagicMapReveal { row: 0 },
                            }
                        }
                        RunState::MainGame {
                            runstate: MainGameRunState::TownPortal,
                        } => {
                            newrunstate = RunState::MainGame {
                                runstate: MainGameRunState::TownPortal,
                            }
                        }
                        RunState::MainGame {
                            runstate: MainGameRunState::TeleportingToOtherLevel { x, y, depth },
                        } => {
                            newrunstate = RunState::MainGame {
                                runstate: MainGameRunState::TeleportingToOtherLevel { x, y, depth },
                            }
                        }
                        RunState::MainGame {
                            runstate: MainGameRunState::ShowRemoveCurse,
                        } => {
                            newrunstate = RunState::MainGame {
                                runstate: MainGameRunState::ShowRemoveCurse,
                            }
                        }
                        RunState::MainGame {
                            runstate: MainGameRunState::ShowIdentify,
                        } => {
                            newrunstate = RunState::MainGame {
                                runstate: MainGameRunState::ShowIdentify,
                            }
                        }
                        _ => {
                            newrunstate = RunState::MainGame {
                                runstate: MainGameRunState::Ticking,
                            }
                        }
                    }
                }
                if should_change_target {
                    player::end_turn_targeting(&mut state.ecs);
                }
            }
            MainGameRunState::ShowInventory => {
                let result = gui::show_inventory(state, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => {
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::AwaitingInput,
                        }
                    }
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let is_ranged = state.ecs.read_storage::<Ranged>();
                        let is_item_ranged = is_ranged.get(item_entity);
                        if let Some(is_item_ranged) = is_item_ranged {
                            newrunstate = RunState::MainGame {
                                runstate: MainGameRunState::ShowTargeting {
                                    range: is_item_ranged.range,
                                    item: item_entity,
                                },
                            };
                        } else {
                            let mut intent = state.ecs.write_storage::<WantsToUseItem>();
                            intent
                                .insert(
                                    *state.ecs.fetch::<Entity>(),
                                    WantsToUseItem {
                                        item: item_entity,
                                        target: None,
                                    },
                                )
                                .expect("Unable to insert intent");
                            newrunstate = RunState::MainGame {
                                runstate: MainGameRunState::Ticking,
                            };
                        }
                    }
                }
            }
            MainGameRunState::ShowCheatMenu => {
                let result = gui::show_cheat_mode(state, ctx);
                match result {
                    gui::CheatMenuResult::Cancel => {
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::AwaitingInput,
                        }
                    }
                    gui::CheatMenuResult::NoResponse => {}
                    gui::CheatMenuResult::TeleportToExit => {
                        state.main_game_state.goto_level(&mut state.ecs, 1);
                        state.main_game_state.mapgen_next_state = Some(RunState::MainGame {
                            runstate: MainGameRunState::PreRun,
                        });
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::MapGeneration,
                        };
                    }
                    gui::CheatMenuResult::Heal => {
                        let player = state.ecs.fetch::<Entity>();
                        let mut pools = state.ecs.write_storage::<Pools>();
                        let mut player_pools = pools.get_mut(*player).unwrap();
                        player_pools.hit_points.current = player_pools.hit_points.max;
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::AwaitingInput,
                        };
                    }
                    gui::CheatMenuResult::Reveal => {
                        let mut map = state.ecs.fetch_mut::<Map>();
                        for v in map.revealed_tiles.iter_mut() {
                            *v = true;
                        }
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::AwaitingInput,
                        };
                    }
                    gui::CheatMenuResult::GodMode => {
                        let player = state.ecs.fetch::<Entity>();
                        let mut pools = state.ecs.write_storage::<Pools>();
                        let mut player_pools = pools.get_mut(*player).unwrap();
                        player_pools.god_mode = true;
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::AwaitingInput,
                        };
                    }
                    gui::CheatMenuResult::LevelUp => {
                        let player = state.ecs.fetch::<Entity>();
                        crate::effects::add_effect(
                            None,
                            crate::effects::EffectType::AddExperienceLevel,
                            crate::effects::Targets::Single { target: *player },
                        );
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::Ticking,
                        };
                    }
                    gui::CheatMenuResult::Eat => {
                        let player = state.ecs.fetch::<Entity>();
                        crate::effects::add_effect(
                            None,
                            crate::effects::EffectType::WellFed,
                            crate::effects::Targets::Single { target: *player },
                        );
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::Ticking,
                        };
                    }
                }
            }
            MainGameRunState::ShowDropItem => {
                let result = gui::drop_item_menu(state, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => {
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::AwaitingInput,
                        }
                    }
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let mut intent = state.ecs.write_storage::<WantsToDropItem>();
                        intent
                            .insert(
                                *state.ecs.fetch::<Entity>(),
                                WantsToDropItem { item: item_entity },
                            )
                            .expect("Unable to insert intent");
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::Ticking,
                        };
                    }
                }
            }
            MainGameRunState::ShowRemoveItem => {
                let result = gui::remove_item_menu(state, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => {
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::AwaitingInput,
                        }
                    }
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let mut intent = state.ecs.write_storage::<WantsToRemoveItem>();
                        intent
                            .insert(
                                *state.ecs.fetch::<Entity>(),
                                WantsToRemoveItem { item: item_entity },
                            )
                            .expect("Unable to insert intent");
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::Ticking,
                        };
                    }
                }
            }
            MainGameRunState::ShowRemoveCurse => {
                let result = gui::remove_curse_menu(state, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => {
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::AwaitingInput,
                        }
                    }
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        state.ecs.write_storage::<CursedItem>().remove(item_entity);
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::Ticking,
                        };
                    }
                }
            }
            MainGameRunState::ShowIdentify => {
                let result = gui::identify_menu(state, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => {
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::AwaitingInput,
                        }
                    }
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        if let Some(name) = state.ecs.read_storage::<Name>().get(item_entity) {
                            let mut dm = state.ecs.fetch_mut::<MasterDungeonMap>();
                            dm.identified_items.insert(name.name.clone());
                        }
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::Ticking,
                        };
                    }
                }
            }
            MainGameRunState::ShowTargeting { range, item } => {
                let result = gui::ranged_target(state, ctx, range);
                match result.0 {
                    gui::ItemMenuResult::Cancel => {
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::AwaitingInput,
                        }
                    }
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        if state
                            .ecs
                            .read_storage::<SpellTemplate>()
                            .get(item)
                            .is_some()
                        {
                            let mut intent = state.ecs.write_storage::<WantsToCastSpell>();
                            intent
                                .insert(
                                    *state.ecs.fetch::<Entity>(),
                                    WantsToCastSpell {
                                        spell: item,
                                        target: result.1,
                                    },
                                )
                                .expect("Unable to insert intent");
                            newrunstate = RunState::MainGame {
                                runstate: MainGameRunState::Ticking,
                            };
                        } else {
                            let mut intent = state.ecs.write_storage::<WantsToUseItem>();
                            intent
                                .insert(
                                    *state.ecs.fetch::<Entity>(),
                                    WantsToUseItem {
                                        item,
                                        target: result.1,
                                    },
                                )
                                .expect("Unable to insert intent");
                            newrunstate = RunState::MainGame {
                                runstate: MainGameRunState::Ticking,
                            };
                        }
                    }
                }
            }
            MainGameRunState::ShowVendor { vendor, mode } => {
                let result = gui::show_vendor_menu(state, ctx, vendor, mode);
                newrunstate = vendor::handle_vendor_result(
                    &mut state.ecs,
                    vendor,
                    newrunstate,
                    result.0,
                    result.1,
                    result.2,
                    result.3,
                );
            }

            MainGameRunState::GameOver => {
                let result = gui::game_over(ctx);
                match result {
                    gui::GameOverResult::NoSelection => {}
                    gui::GameOverResult::QuitToMenu => {
                        state.main_game_state.game_over_cleanup(&mut state.ecs);
                        newrunstate = RunState::MainGame {
                            runstate: MainGameRunState::MapGeneration,
                        };
                        state.main_game_state.mapgen_next_state = Some(RunState::MainMenu {
                            menu_selection: gui::MainMenuSelection::NewGame,
                        });
                    }
                }
            }
            MainGameRunState::SaveGame => {
                saveload_system::save_game(&mut state.ecs);
                newrunstate = RunState::MainMenu {
                    menu_selection: gui::MainMenuSelection::LoadGame,
                };
            }
            MainGameRunState::NextLevel => {
                state.main_game_state.goto_level(&mut state.ecs, 1);
                state.main_game_state.mapgen_next_state = Some(RunState::MainGame {
                    runstate: MainGameRunState::PreRun,
                });
                newrunstate = RunState::MainGame {
                    runstate: MainGameRunState::MapGeneration,
                };
            }
            MainGameRunState::PreviousLevel => {
                state.main_game_state.goto_level(&mut state.ecs, -1);
                state.main_game_state.mapgen_next_state = Some(RunState::MainGame {
                    runstate: MainGameRunState::PreRun,
                });
                newrunstate = RunState::MainGame {
                    runstate: MainGameRunState::MapGeneration,
                };
            }
            MainGameRunState::TownPortal => {
                // Spawn the portal
                spawner::spawn_town_portal(&mut state.ecs);

                // Transition
                let map_depth = state.ecs.fetch::<Map>().depth;
                let destination_offset = 0 - (map_depth - 1);
                state
                    .main_game_state
                    .goto_level(&mut state.ecs, destination_offset);
                state.main_game_state.mapgen_next_state = Some(RunState::MainGame {
                    runstate: MainGameRunState::PreRun,
                });
                newrunstate = RunState::MainGame {
                    runstate: MainGameRunState::MapGeneration,
                };
            }
            MainGameRunState::TeleportingToOtherLevel { x, y, depth } => {
                state.main_game_state.goto_level(&mut state.ecs, depth - 1);
                let player_entity = state.ecs.fetch::<Entity>();
                if let Some(pos) = state
                    .ecs
                    .write_storage::<Position>()
                    .get_mut(*player_entity)
                {
                    pos.x = x;
                    pos.y = y;
                }
                let mut ppos = state.ecs.fetch_mut::<rltk::Point>();
                ppos.x = x;
                ppos.y = y;
                state.main_game_state.mapgen_next_state = Some(RunState::MainGame {
                    runstate: MainGameRunState::PreRun,
                });
                newrunstate = RunState::MainGame {
                    runstate: MainGameRunState::MapGeneration,
                };
            }
            MainGameRunState::MagicMapReveal { row } => {
                let mut map = state.ecs.fetch_mut::<Map>();
                for x in 0..map.width {
                    let idx = map.xy_idx(x as i32, row);
                    map.revealed_tiles[idx] = true;
                }
                if row == map.height - 1 {
                    newrunstate = RunState::MainGame {
                        runstate: MainGameRunState::Ticking,
                    };
                } else {
                    newrunstate = RunState::MainGame {
                        runstate: MainGameRunState::MagicMapReveal { row: row + 1 },
                    };
                }
            }
        },
        _ => {}
    }

    newrunstate
}
