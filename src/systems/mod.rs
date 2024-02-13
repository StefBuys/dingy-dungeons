use crate::prelude::*;

mod chasing;
mod combat;
mod end_turn;
mod entity_render;
mod fov;
mod hud;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod tooltips;
mod use_item;

// todo: update the schedulers to have an input, logic and rendering scheduler
// todo: look at legions way of handling realtime/ticks to ensure monsters aren't insanely fast
// pub fn build_input_scheduler() -> Schedule {
//     Schedule::builder()
//         .add_system(player_input::player_input_system())
//         .add_system(fov::fov_system())
//         .flush()
//         .add_system(map_render::map_render_system())
//         .add_system(entity_render::entity_render_system())
//         .add_system(hud::hud_system())
//         .add_system(tooltips::tooltips_system())
//         .build()
// }

// pub fn build_player_scheduler() -> Schedule {
//     Schedule::builder()
//         .add_system(use_item::use_items_system())
//         .add_system(combat::combat_system())
//         .flush()
//         .add_system(movement::movement_system())
//         .flush()
//         .add_system(fov::fov_system())
//         .flush()
//         .add_system(map_render::map_render_system())
//         .add_system(entity_render::entity_render_system())
//         .add_system(hud::hud_system())
//         .add_system(end_turn::end_turn_system())
//         .build()
// }

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .add_system(chasing::chasing_system())
        .flush()
        // .add_system(use_item::use_items_system())
        // .add_system(combat::combat_system())
        // .flush()
        // .add_system(movement::movement_system())
        // .flush()
        // .add_system(fov::fov_system())
        // .flush()
        // .add_system(map_render::map_render_system())
        // .add_system(entity_render::entity_render_system())
        // .add_system(hud::hud_system())
        // .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_realtime_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(use_item::use_items_system())
        .flush()
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_render_scheduler() -> Schedule {
    Schedule::builder()
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}
