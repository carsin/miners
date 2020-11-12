use specs::prelude::*;
use super::{CombatStats, MeleeAttacking, Name, SufferDamage};

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = ( Entities<'a>,
                        WriteStorage<'a, MeleeAttacking>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, CombatStats>,
                        WriteStorage<'a, SufferDamage>,
                      );


    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut meleeing, names, combat_stats, mut inflict_damage) = data;

        for (_entity, meleeing, name, attacker_stats) in (&entities, &meleeing, &names, &combat_stats).join() {
            if attacker_stats.hp > 0 { // check if attacker is alive
                let target_stats = combat_stats.get(meleeing.target).unwrap();
                if target_stats.hp > 0 { // check if target is alive
                    let target_name = names.get(meleeing.target).unwrap();

                    let damage = i32::max(0, attacker_stats.damage - target_stats.armor);

                    if damage == 0 {
                        println!("{}", format!("{} attacks can't get through {}'s armor.", &name.name, &target_name.name));
                    } else {
                        println!("{}", format!("{} attacks {} for {} dmg", &name.name, &target_name.name, damage));
                        SufferDamage::new_damage(&mut inflict_damage, meleeing.target, damage);
                    }
                }
            }
        }
        meleeing.clear();
    }
}
