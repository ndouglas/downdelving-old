use crate::components::{IdentifiedItem, Item, Pools};
use crate::main_game::MainGameRunState;
use crate::raws::{SpawnType, RAWS};
use crate::RunState;
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum VendorMode {
    Buy,
    Sell,
}

#[derive(PartialEq, Copy, Clone)]
pub enum VendorResult {
    NoResponse,
    Cancel,
    Sell,
    BuyMode,
    SellMode,
    Buy,
}

pub fn handle_vendor_result(
    ecs: &mut World,
    vendor_entity: Entity,
    current_runstate: RunState,
    vendor_result: VendorResult,
    entity_option: Option<Entity>,
    tag_option: Option<String>,
    price_option: Option<f32>,
) -> RunState {
    let mut newrunstate = current_runstate;
    match vendor_result {
        VendorResult::Cancel => {
            newrunstate = RunState::MainGame {
                runstate: MainGameRunState::AwaitingInput,
            }
        }
        VendorResult::BuyMode => {
            newrunstate = RunState::MainGame {
                runstate: MainGameRunState::ShowVendor {
                    vendor: vendor_entity,
                    mode: VendorMode::Buy,
                },
            }
        }
        VendorResult::SellMode => {
            newrunstate = RunState::MainGame {
                runstate: MainGameRunState::ShowVendor {
                    vendor: vendor_entity,
                    mode: VendorMode::Sell,
                },
            }
        }
        VendorResult::NoResponse => {}
        VendorResult::Sell => {
            let entity = entity_option.unwrap();
            let price = ecs.read_storage::<Item>().get(entity).unwrap().base_value * 0.8;
            ecs.write_storage::<Pools>()
                .get_mut(*ecs.fetch::<Entity>())
                .unwrap()
                .gold += price;
            ecs.delete_entity(entity).expect("Unable to delete");
        }
        VendorResult::Buy => {
            let tag = tag_option.unwrap();
            let price = price_option.unwrap();
            let mut pools = ecs.write_storage::<Pools>();
            let player_entity = ecs.fetch::<Entity>();
            let mut identified = ecs.write_storage::<IdentifiedItem>();
            identified
                .insert(*player_entity, IdentifiedItem { name: tag.clone() })
                .expect("Unable to insert");
            std::mem::drop(identified);
            let player_pools = pools.get_mut(*player_entity).unwrap();
            std::mem::drop(player_entity);
            if player_pools.gold >= price {
                player_pools.gold -= price;
                std::mem::drop(pools);
                let player_entity = *ecs.fetch::<Entity>();
                crate::raws::spawn_named_item(
                    &RAWS.lock().unwrap(),
                    ecs,
                    &tag,
                    SpawnType::Carried { by: player_entity },
                );
            }
        }
    }
    newrunstate
}
