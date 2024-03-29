mod template;

use crate::prelude::*;
use template::Templates;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player{map_level: 0},
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
        FieldOfView::new(8),
        Damage(1),
    ));
}

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point]
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}

// pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
//     let (hp, name, glyph) = match rng.roll_dice(1, 10) {
//         1..=4 => goblin(),
//         5..=6 => ogre(),
//         7..=7 => ettin(),
//         _ => orc(),
//     };
//     ecs.push((
//         Enemy,
//         pos,
//         Render {
//             color: ColorPair::new(WHITE, BLACK),
//             glyph,
//         },
//         ChasingPlayer {},
//         Health {
//             current: hp,
//             max: hp,
//         },
//         Name(name),
//         FieldOfView::new(6),
//     ));
// }

// pub fn spawn_entity(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
//     let roll = rng.roll_dice(1, 6);
//     match roll {
//         1 => spawn_healing_potion(ecs, pos),
//         2 => spawn_magic_mapper(ecs, pos),
//         _ => spawn_monster(ecs, rng, pos)
//     }
// }

// fn goblin() -> (i32, String, FontCharType) {
//     (1, "Goblin".to_string(), to_cp437('g'))
// }

// fn orc() -> (i32, String, FontCharType) {
//     (2, "Orc".to_string(), to_cp437('o'))
// }

// fn ogre() -> (i32, String, FontCharType) {
//     (5, "Ogre".to_string(), to_cp437('O'))
// }

// fn ettin() -> (i32, String, FontCharType) {
//     (7, "Ettin".to_string(), to_cp437('E'))
// }

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}

// pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
//     ecs.push(
//         (Item,
//          pos,
//          Render{
//              color: ColorPair::new(WHITE, BLACK),
//              glyph: to_cp437('!'),
//          },
//          Name("Healing Potion".to_string()),
//          ProvidesHealing{amount: 6},
//          )
//     );
// }

// pub fn spawn_magic_mapper(ecs: &mut World, pos: Point) {
//     ecs.push(
//         (Item, 
//          pos,
//          Render{
//              color: ColorPair::new(WHITE, BLACK),
//              glyph: to_cp437('{'),
//          },
//          Name("Dungeon Map".to_string()),
//          ProvidesDungeonMap{},
//          )
//     );
// }

