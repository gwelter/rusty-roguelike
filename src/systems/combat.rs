use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!(
                "Victim: {:?} - Health before the attack: {}",
                *victim, health.current
            );
            health.current -= 1;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
            println!(
                "Victim: {:?} - Health after the attack: {}",
                *victim, health.current
            );
        }
        commands.remove(*message);
    });
}
