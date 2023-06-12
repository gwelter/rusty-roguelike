use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        // Busca entidade que quer se mover
        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            // Busca o campo de visão da entidade
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(want_move.entity, fov.clone_dirty());
                // Se for o player, atualiza a camera
                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination);
                    // marca os tiles como revelados se estiverem no campo de visão de forma cumulativa, uma vez revelado, sempre revelado
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[map_index(pos.x, pos.y)] = true;
                    });
                }
            }
        }
    }
    commands.remove(*entity);
}
