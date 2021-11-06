use rltk::RGB;

use super::{Map, TileType};
use crate::components::Renderable;

pub fn get_tile_renderable(idx: usize, map: &Map) -> Renderable {
    let mut result = match map.depth {
        9 => get_mushroom_tile_renderable(idx, map),
        8 => get_mushroom_tile_renderable(idx, map),
        7 => {
            let x = idx as i32 % map.width;
            if x > map.width - 16 {
                get_default_tile_renderable(idx, map)
            } else {
                get_mushroom_tile_renderable(idx, map)
            }
        }
        5 => {
            let x = idx as i32 % map.width;
            if x < map.width / 2 {
                get_limestone_cavern_tile_renderable(idx, map)
            } else {
                get_default_tile_renderable(idx, map)
            }
        }
        4 => get_limestone_cavern_tile_renderable(idx, map),
        3 => get_limestone_cavern_tile_renderable(idx, map),
        2 => get_forest_tile_renderable(idx, map),
        _ => get_default_tile_renderable(idx, map),
    };

    if map.bloodstains.contains_key(&idx) {
        result.bg = map.bloodstains[&idx];
    }
    if !map.visible_tiles[idx] {
        result.fg = result.fg.to_greyscale();
        result.bg = RGB::from_f32(0., 0., 0.); // Don't show stains out of visual range
    } else if !map.outdoors {
        result.fg = result.fg * map.light[idx];
        result.bg = result.bg * map.light[idx];
    }

    result
}

fn get_forest_tile_renderable(idx: usize, map: &Map) -> Renderable {
    let glyph;
    let fg;
    let bg = RGB::from_f32(0., 0., 0.);

    match map.tiles[idx] {
        TileType::Wall => {
            glyph = rltk::to_cp437('♣');
            fg = RGB::from_f32(0.0, 0.6, 0.0);
        }
        TileType::Bridge => {
            glyph = rltk::to_cp437('.');
            fg = RGB::named(rltk::CHOCOLATE);
        }
        TileType::Road => {
            glyph = rltk::to_cp437('≡');
            fg = RGB::named(rltk::YELLOW);
        }
        TileType::Grass => {
            glyph = rltk::to_cp437('"');
            fg = RGB::named(rltk::GREEN);
        }
        TileType::ShallowWater => {
            glyph = rltk::to_cp437('~');
            fg = RGB::named(rltk::CYAN);
        }
        TileType::DeepWater => {
            glyph = rltk::to_cp437('~');
            fg = RGB::named(rltk::BLUE);
        }
        TileType::Gravel => {
            glyph = rltk::to_cp437(';');
            fg = RGB::from_f32(0.5, 0.5, 0.5);
        }
        TileType::DownStairs => {
            glyph = rltk::to_cp437('>');
            fg = RGB::from_f32(0., 1.0, 1.0);
        }
        TileType::UpStairs => {
            glyph = rltk::to_cp437('<');
            fg = RGB::from_f32(0., 1.0, 1.0);
        }
        _ => {
            glyph = rltk::to_cp437('"');
            fg = RGB::from_f32(0.0, 0.5, 0.0);
        }
    }

    Renderable {
        glyph,
        fg,
        bg,
        render_order: i32::MAX,
    }
}

fn get_mushroom_tile_renderable(idx: usize, map: &Map) -> Renderable {
    let glyph;
    let fg;
    let bg = RGB::from_f32(0., 0., 0.);

    match map.tiles[idx] {
        TileType::Wall => {
            glyph = rltk::to_cp437('♠');
            fg = RGB::from_f32(1.0, 0.0, 1.0);
        }
        TileType::Bridge => {
            glyph = rltk::to_cp437('.');
            fg = RGB::named(rltk::GREEN);
        }
        TileType::Road => {
            glyph = rltk::to_cp437('≡');
            fg = RGB::named(rltk::CHOCOLATE);
        }
        TileType::Grass => {
            glyph = rltk::to_cp437('"');
            fg = RGB::named(rltk::GREEN);
        }
        TileType::ShallowWater => {
            glyph = rltk::to_cp437('~');
            fg = RGB::named(rltk::CYAN);
        }
        TileType::DeepWater => {
            glyph = rltk::to_cp437('~');
            fg = RGB::named(rltk::BLUE);
        }
        TileType::Gravel => {
            glyph = rltk::to_cp437(';');
            fg = RGB::from_f32(0.5, 0.5, 0.5);
        }
        TileType::DownStairs => {
            glyph = rltk::to_cp437('>');
            fg = RGB::from_f32(0., 1.0, 1.0);
        }
        TileType::UpStairs => {
            glyph = rltk::to_cp437('<');
            fg = RGB::from_f32(0., 1.0, 1.0);
        }
        _ => {
            glyph = rltk::to_cp437('"');
            fg = RGB::from_f32(0.0, 0.6, 0.0);
        }
    }

    Renderable {
        glyph,
        fg,
        bg,
        render_order: i32::MAX,
    }
}

fn get_limestone_cavern_tile_renderable(idx: usize, map: &Map) -> Renderable {
    let glyph;
    let fg;
    let bg = RGB::from_f32(0., 0., 0.);

    match map.tiles[idx] {
        TileType::Wall => {
            glyph = rltk::to_cp437('▒');
            fg = RGB::from_f32(0.7, 0.7, 0.7);
        }
        TileType::Bridge => {
            glyph = rltk::to_cp437('.');
            fg = RGB::named(rltk::CHOCOLATE);
        }
        TileType::Road => {
            glyph = rltk::to_cp437('≡');
            fg = RGB::named(rltk::YELLOW);
        }
        TileType::Grass => {
            glyph = rltk::to_cp437('"');
            fg = RGB::named(rltk::GREEN);
        }
        TileType::ShallowWater => {
            glyph = rltk::to_cp437('░');
            fg = RGB::named(rltk::CYAN);
        }
        TileType::DeepWater => {
            glyph = rltk::to_cp437('▓');
            fg = RGB::from_f32(0.2, 0.2, 1.0);
        }
        TileType::Gravel => {
            glyph = rltk::to_cp437(';');
            fg = RGB::from_f32(0.5, 0.5, 0.5);
        }
        TileType::DownStairs => {
            glyph = rltk::to_cp437('>');
            fg = RGB::from_f32(0., 1.0, 1.0);
        }
        TileType::UpStairs => {
            glyph = rltk::to_cp437('<');
            fg = RGB::from_f32(0., 1.0, 1.0);
        }
        TileType::Stalactite => {
            glyph = rltk::to_cp437('╨');
            fg = RGB::from_f32(0.7, 0.7, 0.7);
        }
        TileType::Stalagmite => {
            glyph = rltk::to_cp437('╥');
            fg = RGB::from_f32(0.7, 0.7, 0.7);
        }
        _ => {
            glyph = rltk::to_cp437('\'');
            fg = RGB::from_f32(0.4, 0.4, 0.4);
        }
    }

    Renderable {
        glyph,
        fg,
        bg,
        render_order: i32::MAX,
    }
}

pub fn get_default_tile_renderable(idx: usize, map: &Map) -> Renderable {
    let glyph;
    let fg;
    let bg = RGB::from_f32(0., 0., 0.);

    match map.tiles[idx] {
        TileType::Floor => {
            glyph = rltk::to_cp437('.');
            fg = RGB::from_f32(0.0, 0.5, 0.5);
        }
        TileType::WoodFloor => {
            glyph = rltk::to_cp437('░');
            fg = RGB::named(rltk::CHOCOLATE);
        }
        TileType::Wall => {
            let x = idx as i32 % map.width;
            let y = idx as i32 / map.width;
            glyph = wall_glyph(&*map, x, y);
            fg = RGB::from_f32(0., 1.0, 0.);
        }
        TileType::DownStairs => {
            glyph = rltk::to_cp437('>');
            fg = RGB::from_f32(0., 1.0, 1.0);
        }
        TileType::UpStairs => {
            glyph = rltk::to_cp437('<');
            fg = RGB::from_f32(0., 1.0, 1.0);
        }
        TileType::Bridge => {
            glyph = rltk::to_cp437('.');
            fg = RGB::named(rltk::CHOCOLATE);
        }
        TileType::Road => {
            glyph = rltk::to_cp437('≡');
            fg = RGB::named(rltk::GRAY);
        }
        TileType::Grass => {
            glyph = rltk::to_cp437('"');
            fg = RGB::named(rltk::GREEN);
        }
        TileType::ShallowWater => {
            glyph = rltk::to_cp437('~');
            fg = RGB::named(rltk::CYAN);
        }
        TileType::DeepWater => {
            glyph = rltk::to_cp437('~');
            fg = RGB::named(rltk::BLUE);
        }
        TileType::Gravel => {
            glyph = rltk::to_cp437(';');
            fg = RGB::from_f32(0.5, 0.5, 0.5);
        }
        TileType::Stalactite => {
            glyph = rltk::to_cp437('╨');
            fg = RGB::from_f32(0.5, 0.5, 0.5);
        }
        TileType::Stalagmite => {
            glyph = rltk::to_cp437('╥');
            fg = RGB::from_f32(0.5, 0.5, 0.5);
        }
    }

    Renderable {
        glyph,
        fg,
        bg,
        render_order: i32::MAX,
    }
}

fn wall_glyph(map: &Map, x: i32, y: i32) -> rltk::FontCharType {
    if x < 1 || x > map.width - 2 || y < 1 || y > map.height - 2 as i32 {
        return 35;
    }
    let mut mask: u8 = 0;

    if is_revealed_and_wall(map, x, y - 1) {
        mask += 1;
    }
    if is_revealed_and_wall(map, x, y + 1) {
        mask += 2;
    }
    if is_revealed_and_wall(map, x - 1, y) {
        mask += 4;
    }
    if is_revealed_and_wall(map, x + 1, y) {
        mask += 8;
    }

    match mask {
        0 => 9,    // Pillar because we can't see neighbors
        1 => 186,  // Wall only to the north
        2 => 186,  // Wall only to the south
        3 => 186,  // Wall to the north and south
        4 => 205,  // Wall only to the west
        5 => 188,  // Wall to the north and west
        6 => 187,  // Wall to the south and west
        7 => 185,  // Wall to the north, south and west
        8 => 205,  // Wall only to the east
        9 => 200,  // Wall to the north and east
        10 => 201, // Wall to the south and east
        11 => 204, // Wall to the north, south and east
        12 => 205, // Wall to the east and west
        13 => 202, // Wall to the east, west, and south
        14 => 203, // Wall to the east, west, and north
        15 => 206, // ╬ Wall on all sides
        _ => 35,   // We missed one?
    }
}

fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    let idx = map.xy_idx(x, y);
    map.tiles[idx] == TileType::Wall && map.revealed_tiles[idx]
}
