use super::*;
use crate::components::{
    Attributes, Bleeds, Confusion, DamageOverTime, Duration, EquipmentChanged, Name, Player, Pools,
    SerializeMe, Slow, StatusEffect,
};
use crate::map::Map;
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};

pub fn inflict_damage(ecs: &mut World, damage: &EffectSpawner, target: Entity) {
    let mut pools = ecs.write_storage::<Pools>();
    let bleeds = ecs.read_storage::<Bleeds>();
    let player_entity = ecs.fetch::<Entity>();
    if let Some(pool) = pools.get_mut(target) {
        if !pool.god_mode {
            if let Some(creator) = damage.creator {
                if creator == target {
                    return;
                }
            }
            if let EffectType::Damage { amount } = damage.effect_type {
                pool.hit_points.current -= amount;
                if let Some(bleeder) = bleeds.get(target) {
                    if bleeder.bleeds {
                        add_effect(
                            None,
                            EffectType::Bloodstain {
                                color: bleeder.color,
                            },
                            Targets::Single { target },
                        );
                    }
                }
                add_effect(
                    None,
                    EffectType::Particle {
                        glyph: rltk::to_cp437('‼'),
                        fg: rltk::RGB::named(rltk::ORANGE),
                        bg: rltk::RGB::named(rltk::BLACK),
                        lifespan: 200.0,
                    },
                    Targets::Single { target },
                );
                if target == *player_entity {
                    crate::gamelog::record_event("Damage Taken", amount);
                }
                if let Some(creator) = damage.creator {
                    if creator == *player_entity {
                        crate::gamelog::record_event("Damage Inflicted", amount);
                    }
                }

                if pool.hit_points.current < 1 {
                    add_effect(
                        damage.creator,
                        EffectType::EntityDeath,
                        Targets::Single { target },
                    );
                }
            }
        }
    }
}

pub fn bloodstain(ecs: &mut World, tile_idx: i32, effect: &EffectSpawner) {
    if let EffectType::Bloodstain { color } = &effect.effect_type {
        let mut map = ecs.fetch_mut::<Map>();
        map.bloodstains.insert(tile_idx as usize, *color);
    }
}

pub fn death(ecs: &mut World, effect: &EffectSpawner, target: Entity) {
    let mut gold_gain = 0.0f32;

    let mut pools = ecs.write_storage::<Pools>();
    let mut attributes = ecs.write_storage::<Attributes>();

    if let Some(pos) = entity_position(ecs, target) {
        crate::spatial::remove_entity(target, pos as usize);
    }

    if let Some(source) = effect.creator {
        if ecs.read_storage::<Player>().get(source).is_some() {
            if let Some(stats) = pools.get(target) {
                add_effect(
                    None,
                    EffectType::AddExperience {
                        amount: stats.level * 100,
                    },
                    Targets::Single { target: source },
                );
                gold_gain += stats.gold;
            }

            if gold_gain != 0.0 {
                let mut player_stats = pools.get_mut(source).unwrap();
                player_stats.gold += gold_gain;
            }
        }
    }
}

pub fn heal_damage(ecs: &mut World, heal: &EffectSpawner, target: Entity) {
    let mut pools = ecs.write_storage::<Pools>();
    if let Some(pool) = pools.get_mut(target) {
        if let EffectType::Healing { amount } = heal.effect_type {
            pool.hit_points.current =
                i32::min(pool.hit_points.max, pool.hit_points.current + amount);
            add_effect(
                None,
                EffectType::Particle {
                    glyph: rltk::to_cp437('‼'),
                    fg: rltk::RGB::named(rltk::GREEN),
                    bg: rltk::RGB::named(rltk::BLACK),
                    lifespan: 200.0,
                },
                Targets::Single { target },
            );
        }
    }
}

pub fn restore_mana(ecs: &mut World, mana: &EffectSpawner, target: Entity) {
    let mut pools = ecs.write_storage::<Pools>();
    if let Some(pool) = pools.get_mut(target) {
        if let EffectType::Mana { amount } = mana.effect_type {
            pool.mana.current = i32::min(pool.mana.max, pool.mana.current + amount);
            add_effect(
                None,
                EffectType::Particle {
                    glyph: rltk::to_cp437('‼'),
                    fg: rltk::RGB::named(rltk::BLUE),
                    bg: rltk::RGB::named(rltk::BLACK),
                    lifespan: 200.0,
                },
                Targets::Single { target },
            );
        }
    }
}

pub fn add_confusion(ecs: &mut World, effect: &EffectSpawner, target: Entity) {
    if let EffectType::Confusion { turns } = &effect.effect_type {
        ecs.create_entity()
            .with(StatusEffect { target })
            .with(Confusion {})
            .with(Duration { turns: *turns })
            .with(Name {
                name: "Confusion".to_string(),
            })
            .marked::<SimpleMarker<SerializeMe>>()
            .build();
    }
}

pub fn attribute_effect(ecs: &mut World, effect: &EffectSpawner, target: Entity) {
    if let EffectType::AttributeEffect {
        bonus,
        name,
        duration,
    } = &effect.effect_type
    {
        ecs.create_entity()
            .with(StatusEffect { target })
            .with(bonus.clone())
            .with(Duration { turns: *duration })
            .with(Name { name: name.clone() })
            .marked::<SimpleMarker<SerializeMe>>()
            .build();
        ecs.write_storage::<EquipmentChanged>()
            .insert(target, EquipmentChanged {})
            .expect("Insert failed");
    }
}

pub fn slow(ecs: &mut World, effect: &EffectSpawner, target: Entity) {
    if let EffectType::Slow { initiative_penalty } = &effect.effect_type {
        ecs.create_entity()
            .with(StatusEffect { target })
            .with(Slow {
                initiative_penalty: *initiative_penalty,
            })
            .with(Duration { turns: 5 })
            .with(if *initiative_penalty > 0.0 {
                Name {
                    name: "Slowed".to_string(),
                }
            } else {
                Name {
                    name: "Hasted".to_string(),
                }
            })
            .marked::<SimpleMarker<SerializeMe>>()
            .build();
    }
}

pub fn damage_over_time(ecs: &mut World, effect: &EffectSpawner, target: Entity) {
    if let EffectType::DamageOverTime { damage } = &effect.effect_type {
        ecs.create_entity()
            .with(StatusEffect { target })
            .with(DamageOverTime { damage: *damage })
            .with(Duration { turns: 5 })
            .with(Name {
                name: "Damage Over Time".to_string(),
            })
            .marked::<SimpleMarker<SerializeMe>>()
            .build();
    }
}
