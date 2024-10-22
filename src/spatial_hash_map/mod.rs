// use bevy::prelude::*;
// use bevy::utils::HashMap;
//
// #[derive(Debug, Clone, Copy)]
// pub struct Grid {
//     pub spacing: f32,
// }
// /// https://leetless.de/posts/spatial-hashing-vs-ecs/
// #[derive(Debug)]
// pub struct SpatialHashmap {
//     pub grid: Grid,
//     hashmap: HashMap<Vec2, HashMap<Entity, Vec2>>,
// }
//
// impl SpatialHashmap {
//     pub fn insert(&mut self, position: Vec2, entity: Entity) {
//         let index = self.grid.index2d(position);
//         self.hashmap
//             .entry(index)
//             .or_insert(default())
//             .insert(entity, position);
//     }
//
//     pub fn update(&mut self, entity: Entity, previous_position: Vec2, new_position: Vec2) {
//         let prev_index = self.grid.index2d(previous_position);
//         let new_index = self.grid.index2d(new_position);
//
//         if new_index != prev_index {
//             // If old cell exists, remove entry from it
//             self.hashmap.entry(prev_index).and_modify(|h| { h.remove(&entity); });
//         }
//
//         // Ensure new cell exists and insert entity into it
//         self.hashmap.entry(new_index)
//             .or_default()
//             .insert(entity, new_position);
//
//     }
// }
