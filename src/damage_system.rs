use specs::prelude::*;
use super::{CombatStats, SufferDamage};

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
        let entities = world.entities();
        // loop through entities with hp
        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1 {
                dead.push(entity);
            }
        }
    }

    // loop through dead entites and clear them
    for entity in dead {
        world.delete_entity(entity).expect("Unable to remove dead entity");
    }
}
