use std::collections::{HashMap, HashSet, VecDeque};
use bevy::ecs::component::SparseStorage;
use bevy::prelude::*;

pub mod actor;
pub mod action;

use crate::scene::actor::Actor;
use crate::scene::action::Action;

#[derive(Default)]
pub struct Scene {
    actors : std::collections::HashSet<usize>,
}

impl Scene {
    pub fn actors(&self) -> &HashSet<usize> {
        &self.actors
    }

    pub fn add_actor(&mut self, actor_id : usize) {
        self.actors.insert(actor_id);
    }
}

impl Component for Scene { type Storage = SparseStorage; }

pub fn process_scene(scene : &mut Scene, actor_list: &mut HashMap<usize, &mut Actor>) {
    for idx in scene.actors() {
        if ! actor_list.contains_key(idx ) {
            continue;
        }
        let actor = actor_list.get_mut(idx).unwrap(); // TODO: Replace Unwrap.
        let action = actor.actions_mut().pop_front().unwrap();
        let action_results = apply_action(action, actor);
        for (i, a) in action_results {
            if let Some(actor) = actor_list.get_mut(&i) {
                let _ = apply_action(a, actor); // TODO: Use result.
            }
        }
    }
}

fn apply_action(action : Action, actor : &mut Actor) -> Vec<(usize, Action)> {
    println!("{}", action.generate_output());
    println!("{}", actor.name());
    Vec::new()
}