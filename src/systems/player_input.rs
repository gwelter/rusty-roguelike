use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let mut turn_skipped = false;
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::Space => {
                turn_skipped = true;
                Point::zero()
            }
            _ => Point::zero(),
        };
        if delta.x != 0 || delta.y != 0 || turn_skipped {
            let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
            let (player_entity, destination) = players
                .iter(ecs)
                .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
                .unwrap();
            let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
            if delta.x != 0 || delta.y != 0 {
                let mut hit_something = false;
                enemies
                    .iter(ecs)
                    .filter(|(_, position)| **position == destination)
                    .for_each(|(entity, _)| {
                        hit_something = true;
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: player_entity,
                                victim: *entity,
                            },
                        ));
                    });
                if !hit_something {
                    commands.push((
                        (),
                        WantsToMove {
                            entity: player_entity,
                            destination,
                        },
                    ));
                }
            }
            if turn_skipped {
                if let Ok(mut health) = ecs
                    .entry_mut(player_entity)
                    .unwrap()
                    .get_component_mut::<Health>()
                {
                    health.current = i32::min(health.max, health.current + 1);
                }
            }
        }
        *turn_state = TurnState::PlayerTurn;
    }
}
