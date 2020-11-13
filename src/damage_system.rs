use specs::prelude::*;
use super::{CombatStats, SufferDamage, Player, Name, GameLog};

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = ( WriteStorage<'a, CombatStats>,
                        WriteStorage<'a, SufferDamage>,
                      );


    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp -= damage.amount.iter().sum::<i32>(); // iterate through all damage, not just one per turn
        }
        damage.clear();
    }

}

pub fn remove_dead(world: &mut World) {
    let mut dead: Vec<Entity> = Vec::new(); // initialize empty array to store all dead entities
    // new scope to avoid borrow
    {
        let combat_stats = world.read_storage::<CombatStats>();
        let players = world.read_storage::<Player>();
        let names = world.read_storage::<Name>();
        let entities = world.entities();
        let mut log = world.write_resource::<GameLog>();
        // loop through entities with hp
        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1 {
                let player = players.get(entity);
                match player {
                    None => {
                        let victim_name = names.get(entity);
                        if let Some(victim_name) = victim_name {
                            log.entries.push(format!("{} died", &victim_name.name));
                        }
                        dead.push(entity);
                    },

                    Some(_) => {
                        let death_text = String::from("You have perished.");
                        println!("{}", death_text);
                        log.entries.push(death_text);
                    }
                }
            }
        }
    }

    // loop through dead entites and clear them
    for entity in dead {
        world.delete_entity(entity).expect("Unable to remove dead entity");
    }
}
