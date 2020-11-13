use bracket_terminal::prelude::{RGB, BTerm};
use super::{CombatStats, Player, GameLog, Map, Name, Position, BASE_LIGHT_LEVEL};
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

    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::from_f32(0.2, 0.2, 0.2));
    draw_tooltips(world, ctx);
}

fn draw_tooltips(world: &World, ctx: &mut BTerm) {
    let map = world.fetch::<Map>();
    let names = world.read_storage::<Name>();
    let positions = world.read_storage::<Position>();

    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= map.width as i32 || mouse_pos.1 >= map.height as i32 { return; }
    let mut tooltip : Vec<String> = Vec::new();
    for (name, position) in (&names, &positions).join() {
        let idx = map.xy_idx(position.x, position.y);
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 && map.light_levels[idx] > Some(BASE_LIGHT_LEVEL) {
            tooltip.push(name.name.to_string());
        }
    }

    if !tooltip.is_empty() {
        let mut width :i32 = 0;
        for s in tooltip.iter() {
            if width < s.len() as i32 { width = s.len() as i32; }
        }
        width += 3;

        if mouse_pos.0 > 40 {
            let arrow_pos = Position::new(mouse_pos.0 - 2, mouse_pos.1);
            let left_x = mouse_pos.0 - width;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(left_x, y, RGB::from_u8(255, 255, 255), RGB::from_u8(100, 100, 100), s);
                let padding = (width - s.len() as i32)-1;
                for i in 0..padding {
                    ctx.print_color(arrow_pos.x - i, y, RGB::from_u8(255, 255, 255), RGB::from_u8(100, 100, 100), &" ".to_string());
                }
                y += 1;
            }
            ctx.print_color(arrow_pos.x, arrow_pos.y, RGB::from_u8(255, 255, 255), RGB::from_u8(100, 100, 100), &"->".to_string());
        } else {
            let arrow_pos = Position::new(mouse_pos.0 + 1, mouse_pos.1);
            let left_x = mouse_pos.0 +3;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(left_x + 1, y, RGB::from_u8(255, 255, 255), RGB::from_u8(100, 100, 100), s);
                let padding = (width - s.len() as i32)-1;
                for i in 0..padding {
                    ctx.print_color(arrow_pos.x + 1 + i, y, RGB::from_u8(255, 255, 255), RGB::from_u8(100, 100, 100), &" ".to_string());
                }
                y += 1;
            }
            ctx.print_color(arrow_pos.x, arrow_pos.y, RGB::from_u8(255, 255, 255), RGB::from_u8(100, 100, 100), &"<-".to_string());
        }
    }
}
