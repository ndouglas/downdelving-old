use crate::map_builders::{
    AreaStartingPosition, BuilderChain, CellularAutomataBuilder, CullUnreachable, VoronoiSpawning,
    XStart, YStart, YellowBrickRoad,
};

pub fn forest_builder(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut chain = BuilderChain::new(new_depth, width, height, "Into the Woods");
    chain.start_with(CellularAutomataBuilder::new());
    chain.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    chain.with(CullUnreachable::new());
    chain.with(AreaStartingPosition::new(XStart::LEFT, YStart::CENTER));
    chain.with(VoronoiSpawning::new());
    chain.with(YellowBrickRoad::new());
    chain
}
