use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(Carried)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    // rusty: obtain, then modify data in separate loop
    // borrow checker problems
    let targets = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.target))
        .collect::<Vec<(Entity, Entity, Entity)>>();

    for (message, attacker, target) in &targets {
        let is_player = ecs
            .entry_ref(*target)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        let base_damage = if let Ok(v) = ecs.entry_ref(*attacker) {
            if let Ok(dmg) = v.get_component::<Damage>() {
                dmg.0
            } else {
                0
            }
        } else {
            0
        };

        let weapon_damage: i32 = <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, dmg)| dmg.0)
            .sum();

        let final_damage = base_damage + weapon_damage;

        if let Ok(mut health) = ecs
            .entry_mut(*target)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);

            health.current -= final_damage;
            if health.current < 1 && !is_player {
                commands.remove(*target);
            }
            println!("Health after attack: {}", health.current);
        };
        commands.remove(*message);
    }
}
