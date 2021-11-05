use crate::map_builders::{
    AreaStartingPosition, BuilderChain, CaveDecorator, DLABuilder, DistantExit, PrefabBuilder,
    VoronoiSpawning, XStart, YStart,
};

pub fn limestone_deep_cavern_builder(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut chain = BuilderChain::new(new_depth, width, height, "Deep Limestone Caverns");
    chain.start_with(DLABuilder::central_attractor());
    chain.with(AreaStartingPosition::new(XStart::LEFT, YStart::TOP));
    chain.with(VoronoiSpawning::new());
    chain.with(DistantExit::new());
    chain.with(CaveDecorator::new());
    chain.with(PrefabBuilder::sectional(
        crate::map_builders::prefab_builder::prefab_sections::ORC_CAMP,
    ));
    chain
}
