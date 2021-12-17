use std::collections::{HashMap, HashSet, VecDeque};

pub mod action;
pub mod actor;

use crate::scene::action::Action;
use crate::scene::actor::Actor;

#[derive(Default)]
pub struct Scene {
    actors: std::collections::HashSet<usize>,
    pub status: SceneStatus,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SceneStatus {
    RequireUserInput,
    RequireAIInput,
    Ready,
}

impl Default for SceneStatus {
    fn default() -> Self {
        Self::RequireUserInput
    }
}

impl Scene {
    pub fn actors(&self) -> &HashSet<usize> {
        &self.actors
    }

    pub fn add_actor(&mut self, actor_id: usize) {
        self.actors.insert(actor_id);
    }
}

pub fn process_scene(scene: &mut Scene, actor_list: &mut HashMap<usize, &mut Actor>) {
    let mut actors = scene.actors().iter().map(|x| *x).collect::<Vec<usize>>();
    actors.sort();
    for idx in actors {
        process_actions(scene, actor_list, idx);
    }
    scene.status = SceneStatus::RequireUserInput;
}

fn process_actions(scene: &mut Scene, actor_list: &mut HashMap<usize, &mut Actor>, idx: usize) {
    if !actor_list.contains_key(&idx) {
        return;
    }
    let mut maybe_action = actor_list.get_mut(&idx).unwrap().actions_mut().pop_front();
    while let Some(action) = maybe_action {
        maybe_action = actor_list.get_mut(&idx).unwrap().actions_mut().pop_front();
        let (next, reactions) = action.apply(actor_list, idx, scene);
        for (i, a) in reactions {
            a.apply(actor_list, i, scene);
        }
    }
}
