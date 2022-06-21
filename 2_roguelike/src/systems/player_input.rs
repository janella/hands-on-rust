use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    // q: example of trivially_copy_pass_by_ref
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            // q: alternative turbofish?
            //   <(&mut Point)>::query().filter(component::<Player>());
            //   <(&mut Point, &Player)>::query();
            // we just happen to be fetching the Player component as well in v2
            // (wasteful - since we toss it away in the iterator)
            let mut players = <(Entity, &mut Point)>::query().filter(component::<Player>());

            players.iter_mut(ecs).for_each(|(player, pos)| {
                let destination = *pos + delta;
                commands.push((
                    (),
                    WantsToMove {
                        entity: *player,
                        destination,
                    },
                ));
            });
            *turn_state = TurnState::PlayerTurn;
        }
    }
}
