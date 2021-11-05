use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Stalactite,
    Stalagmite,
    Floor,
    DownStairs,
    Road,
    Grass,
    ShallowWater,
    DeepWater,
    WoodFloor,
    Bridge,
    Gravel,
    UpStairs,
}

impl TileType {
    pub fn is_walkable(self: TileType) -> bool {
        match self {
            TileType::Floor
            | TileType::DownStairs
            | TileType::Road
            | TileType::Grass
            | TileType::ShallowWater
            | TileType::WoodFloor
            | TileType::Bridge
            | TileType::Gravel
            | TileType::UpStairs => true,
            _ => false,
        }
    }

    pub fn is_opaque(self: TileType) -> bool {
        match self {
            TileType::Wall | TileType::Stalactite | TileType::Stalagmite => true,
            _ => false,
        }
    }

    pub fn cost(self: TileType) -> f32 {
        match self {
            TileType::Road => 0.8,
            TileType::Grass => 1.1,
            TileType::ShallowWater => 1.2,
            _ => 1.0,
        }
    }
}
