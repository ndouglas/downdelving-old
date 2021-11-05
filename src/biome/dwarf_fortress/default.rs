use crate::map_builders::{
    AreaEndingPosition, AreaStartingPosition, BspCorridors, BspDungeonBuilder, BuilderChain,
    CorridorSpawner, CullUnreachable, DistantExit, DragonSpawner, DragonsLair, RoomDrawer,
    RoomSort, RoomSorter, VoronoiSpawning, XEnd, XStart, YEnd, YStart,
};

pub fn dwarf_fortress_builder(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut chain = BuilderChain::new(new_depth, width, height, "Dwarven Fortress");
    chain.start_with(BspDungeonBuilder::new());
    chain.with(RoomSorter::new(RoomSort::CENTRAL));
    chain.with(RoomDrawer::new());
    chain.with(BspCorridors::new());
    chain.with(CorridorSpawner::new());
    chain.with(DragonsLair::new());
    chain.with(AreaStartingPosition::new(XStart::LEFT, YStart::TOP));
    chain.with(CullUnreachable::new());
    chain.with(AreaEndingPosition::new(XEnd::RIGHT, YEnd::BOTTOM));
    chain.with(VoronoiSpawning::new());
    chain.with(DistantExit::new());
    chain.with(DragonSpawner::new());
    chain
}
