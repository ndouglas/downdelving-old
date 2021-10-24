use super::*;
use specs::prelude::*;
use crate::gamesystem::{ mana_at_level, player_hp_at_level };
use crate::components::{ Attributes, Pools, Skills, EquipmentChanged };

pub fn add_experience(ecs: &mut World, experience_effect: &EffectSpawner, target: Entity) {
    let mut pools = ecs.write_storage::<Pools>();
    let mut attributes = ecs.write_storage::<Attributes>();
    let mut player_stats = pools.get_mut(target).unwrap();
    if let EffectType::AddExperience { amount } = experience_effect.effect_type {
        player_stats.xp += amount;
        if player_stats.xp >= player_stats.level * 1000 {
            add_effect(
                None,
                EffectType::AddExperienceLevel {},
                Targets::Single { target: target },
            );
        }
    }
}

pub fn add_experience_level(ecs: &mut World, level_effect: &EffectSpawner, target: Entity) {
    let mut pools = ecs.write_storage::<Pools>();
    let mut attributes = ecs.write_storage::<Attributes>();
    let mut player_stats = pools.get_mut(target).unwrap();
    let mut player_attributes = attributes.get_mut(target).unwrap();
    player_stats.level += 1;
    crate::gamelog::Logger::new()
        .color(rltk::MAGENTA)
        .append("Congratulations, you are now level")
        .append(format!("{}", player_stats.level))
        .log();

    let attr_to_boost = crate::rng::roll_dice(1, 4);
    match attr_to_boost {
        1 => {
            player_attributes.might.base += 1;
            crate::gamelog::Logger::new()
                .color(rltk::GREEN)
                .append("You feel stronger!")
                .log();
        }
        2 => {
            player_attributes.fitness.base += 1;
            crate::gamelog::Logger::new()
                .color(rltk::GREEN)
                .append("You feel healthier!")
                .log();
        }
        3 => {
            player_attributes.quickness.base += 1;
            crate::gamelog::Logger::new()
                .color(rltk::GREEN)
                .append("You feel quicker!")
                .log();
        }
        _ => {
            player_attributes.intelligence.base += 1;
            crate::gamelog::Logger::new()
                .color(rltk::GREEN)
                .append("You feel smarter!")
                .log();
        }
    }

    let mut skills = ecs.write_storage::<Skills>();
    let player_skills = skills.get_mut(*ecs.fetch::<Entity>()).unwrap();
    for sk in player_skills.skills.iter_mut() {
        *sk.1 += 1;
    }

    ecs.write_storage::<EquipmentChanged>()
        .insert(*ecs.fetch::<Entity>(), EquipmentChanged {})
        .expect("Insert Failed");

    player_stats.hit_points.max = player_hp_at_level(
        player_attributes.fitness.base + player_attributes.fitness.modifiers,
        player_stats.level,
    );
    player_stats.hit_points.current = player_stats.hit_points.max;
    player_stats.mana.max = mana_at_level(
        player_attributes.intelligence.base + player_attributes.intelligence.modifiers,
        player_stats.level,
    );
    player_stats.mana.current = player_stats.mana.max;

    let player_pos = ecs.fetch::<rltk::Point>();
    let map = ecs.fetch::<Map>();
    for i in 0..10 {
        if player_pos.y - i > 1 {
            add_effect(
                None,
                EffectType::Particle {
                    glyph: rltk::to_cp437('░'),
                    fg: rltk::RGB::named(rltk::GOLD),
                    bg: rltk::RGB::named(rltk::BLACK),
                    lifespan: 400.0,
                },
                Targets::Tile {
                    tile_idx: map.xy_idx(player_pos.x, player_pos.y - i) as i32,
                },
            );
        }
    }
}
