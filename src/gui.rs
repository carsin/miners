use bracket_terminal::prelude::{RGB, BTerm, Console};
use super::{CombatStats, Player, GameLog};
use specs::prelude::*;

pub fn draw_ui(world: &World, ctx: &mut BTerm) {
    let log = world.fetch::<GameLog>();
    ctx.draw_box(0, 43, 79, 6, RGB::from_f32(0.7, 0.7, 0.7), RGB::from_f32(0.0, 0.0, 0.0));

    let mut y = 44;
    // TODO: Improve look of this
    for msg in log.entries.iter().rev() {
        if y < 49 { ctx.print_color(2, y, RGB::from_f32(0.8, 0.8, 0.8), RGB::from_f32(0.0, 0.0, 0.0), msg); }
        y += 1;
    }

    // render HP
    let combat_stats = world.read_storage::<CombatStats>();
    let players = world.read_storage::<Player>();
    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        ctx.print_color(12, 43, RGB::from_f32(0.8, 0.8, 0.0), RGB::from_f32(0.0, 0.0, 0.0), &health);
        ctx.draw_bar_horizontal(28, 43, 51, stats.hp, stats.max_hp, RGB::from_f32(0.9, 0.0, 0.0), RGB::from_f32(0.0, 0.0, 0.0));
    }
}
