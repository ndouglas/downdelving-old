use super::{spawner, Map, Position, Rect, TileType, SHOW_MAPGEN_VISUALIZER};
use specs::prelude::*;
pub mod area_ending_point;
pub mod area_starting_points;
pub mod bsp_dungeon;
pub mod bsp_interior;
pub mod cellular_automata;
pub mod common;
pub mod cull_unreachable;
pub mod distant_exit;
pub mod dla;
pub mod door_placement;
pub mod drunkard;
pub mod dwarf_fort_builder;
pub mod forest;
pub mod limestone_cavern;
pub mod maze;
pub mod prefab_builder;
pub mod room_based_spawner;
pub mod room_based_stairs;
pub mod room_based_starting_position;
pub mod room_corner_rounding;
pub mod room_corridor_spawner;
pub mod room_draw;
pub mod room_exploder;
pub mod room_sorter;
pub mod rooms_corridors_bsp;
pub mod rooms_corridors_dogleg;
pub mod rooms_corridors_lines;
pub mod rooms_corridors_nearest;
pub mod simple_map;
pub mod town;
pub mod voronoi;
pub mod voronoi_spawning;
pub mod waveform_collapse;
pub use area_ending_point::*;
pub use area_starting_points::{AreaStartingPosition, XStart, YStart};
pub use bsp_dungeon::BspDungeonBuilder;
pub use bsp_interior::BspInteriorBuilder;
pub use cellular_automata::CellularAutomataBuilder;
pub use common::*;
pub use cull_unreachable::CullUnreachable;
pub use distant_exit::DistantExit;
pub use dla::DLABuilder;
pub use door_placement::DoorPlacement;
pub use drunkard::DrunkardsWalkBuilder;
pub use dwarf_fort_builder::*;
pub use forest::*;
pub use limestone_cavern::*;
pub use maze::MazeBuilder;
pub use prefab_builder::PrefabBuilder;
pub use room_based_spawner::RoomBasedSpawner;
pub use room_based_stairs::RoomBasedStairs;
pub use room_based_starting_position::RoomBasedStartingPosition;
pub use room_corner_rounding::RoomCornerRounder;
pub use room_corridor_spawner::CorridorSpawner;
pub use room_draw::RoomDrawer;
pub use room_exploder::RoomExploder;
pub use room_sorter::{RoomSort, RoomSorter};
pub use rooms_corridors_bsp::BspCorridors;
pub use rooms_corridors_dogleg::DoglegCorridors;
pub use rooms_corridors_lines::StraightLineCorridors;
pub use rooms_corridors_nearest::NearestCorridors;
pub use simple_map::SimpleMapBuilder;
pub use town::TownBuilder;
pub use voronoi::VoronoiCellBuilder;
pub use voronoi_spawning::VoronoiSpawning;
pub use waveform_collapse::WaveformCollapseBuilder;

pub struct BuilderMap {
    pub spawn_list: Vec<(usize, String)>,
    pub map: Map,
    pub starting_position: Option<Position>,
    pub rooms: Option<Vec<Rect>>,
    pub corridors: Option<Vec<Vec<usize>>>,
    pub history: Vec<Map>,
    pub width: i32,
    pub height: i32,
}

impl BuilderMap {
    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            for v in snapshot.revealed_tiles.iter_mut() {
                *v = true;
            }
            self.history.push(snapshot);
        }
    }
}

pub struct BuilderChain {
    starter: Option<Box<dyn InitialMapBuilder>>,
    builders: Vec<Box<dyn MetaMapBuilder>>,
    pub build_data: BuilderMap,
}

impl BuilderChain {
    pub fn new<S: ToString>(new_depth: i32, width: i32, height: i32, name: S) -> BuilderChain {
        BuilderChain {
            starter: None,
            builders: Vec::new(),
            build_data: BuilderMap {
                spawn_list: Vec::new(),
                map: Map::new(new_depth, width, height, name),
                starting_position: None,
                rooms: None,
                corridors: None,
                history: Vec::new(),
                width,
                height,
            },
        }
    }

    pub fn start_with(&mut self, starter: Box<dyn InitialMapBuilder>) {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have one starting builder."),
        };
    }

    pub fn with(&mut self, metabuilder: Box<dyn MetaMapBuilder>) {
        self.builders.push(metabuilder);
    }

    pub fn build_map(&mut self) {
        match &mut self.starter {
            None => panic!("Cannot run a map builder chain without a starting build system"),
            Some(starter) => {
                // Build the starting map
                starter.build_map(&mut self.build_data);
            }
        }

        // Build additional layers in turn
        for metabuilder in self.builders.iter_mut() {
            metabuilder.build_map(&mut self.build_data);
        }
    }

    pub fn spawn_entities(&mut self, ecs: &mut World) {
        for entity in self.build_data.spawn_list.iter() {
            spawner::spawn_entity(ecs, &(&entity.0, &entity.1));
        }
    }
}

pub trait InitialMapBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap);
}

pub trait MetaMapBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap);
}

pub fn level_builder(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    rltk::console::log(format!("Depth: {}", new_depth));
    match new_depth {
        1 => crate::biome::town_builder(new_depth, width, height),
        2 => crate::biome::forest_builder(new_depth, width, height),
        3 => crate::biome::limestone_cavern_builder(new_depth, width, height),
        4 => crate::biome::limestone_deep_cavern_builder(new_depth, width, height),
        5 => crate::biome::dwarf_fortress_upper_reaches_builder(new_depth, width, height),
        6 => crate::biome::dwarf_fortress_builder(new_depth, width, height),
        7 => crate::biome::mushroom_entrance(new_depth, width, height),
        8 => crate::biome::mushroom_builder(new_depth, width, height),
        9 => crate::biome::mushroom_exit(new_depth, width, height),
        10 => crate::biome::dark_elf_city(new_depth, width, height),
        _ => crate::biome::random_builder(new_depth, width, height),
    }
}
