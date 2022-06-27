use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    // rusty: obtain, then modify data in separate loop
    // borrow checker problems
    let targets = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.target))
        .collect::<Vec<(Entity, Entity)>>();

    for (message, target) in &targets {
        let is_player = ecs
            .entry_ref(*target)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(mut health) = ecs
            .entry_mut(*target)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 && !is_player {
                commands.remove(*target);
            }
            println!("Health after attack: {}", health.current);
        };
        commands.remove(*message);
    }
}
