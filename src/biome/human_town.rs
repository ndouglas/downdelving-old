use crate::map_builders::{BuilderChain, TownBuilder};

pub fn town_builder(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut chain = BuilderChain::new(new_depth, width, height, "The Town of Downdelving");
    chain.start_with(TownBuilder::new());
    chain
}
