use crate::prelude::*;

const FORSTRESS: (&str, i32, i32) = (
    "
    ------------
    ---######---
    ---#----#---
    ---#-M--#---
    -###----###-
    --M------M--
    -###----###-
    ---#----#---
    ---#----#---
    ---######---
    ------------
    ",
    12,
    11,
);

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut placement = None;
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &vec![mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0,
    );
    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        let dimensions = Rect::with_size(
            rng.range(0, SCREEN_WIDTH - FORSTRESS.1),
            rng.range(0, SCREEN_HEIGHT - FORSTRESS.2),
            FORSTRESS.1,
            FORSTRESS.2,
        );
        let mut can_place = false;
        dimensions.for_each(|point| {
            let idx = mb.map.point2d_to_index(point);
            let distance = dijkstra_map.map[idx];
            if distance < 2000.0 && distance > 20.0 && mb.amulet_start != point {
                can_place = true;
            }
        });
        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.monster_spawn.retain(|point| !points.contains(point));
        }
        attempts += 1;
    }
    if let Some(placement) = placement {
        let string_vec: Vec<char> = FORSTRESS
            .0
            .chars()
            .filter(|a| *a != '\r' && *a != '\n')
            .collect();
        let mut i = 0;
        for ty in placement.y..placement.y + FORSTRESS.2 {
            for tx in placement.x..placement.x + FORSTRESS.1 {
                let idx = map_idx(tx, ty);
                let c = string_vec[i];
                match c {
                    'M' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.monster_spawn.push(Point::new(tx, ty));
                    }
                    '-' => mb.map.tiles[idx] = TileType::Floor,
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    _ => {}
                }
                i += 1;
            }
        }
    }
}
