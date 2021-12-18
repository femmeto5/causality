use crate::scene::actor::{Actor, Status};
use crate::scene::Scene;
use std::collections::HashMap;

pub mod action_collection;
mod basic_actions;

pub use basic_actions::*;

pub struct PossibleActionSet {
    pub(crate) possible_actions: Vec<PossibleAction>,
}

impl PossibleActionSet {
    pub fn build_action_set(
        &self,
        actor_list: &HashMap<usize, &mut Actor>,
        actor: usize,
        scene: &Scene,
    ) -> Vec<Box<dyn Action>> {
        let mut action_set = Vec::new();
        for possible_action in &self.possible_actions {
            if possible_action.check(actor_list, actor, scene) {
                action_set.push(possible_action.action.clone());
            }
        }
        action_set
    }
}

pub fn build_action_option_mapping(actions: &Vec<Box<dyn Action>>) -> HashMap<String, usize> {
    let mut mapping = HashMap::new();
    for (i, _) in actions.iter().enumerate() {
        mapping.insert(i.to_string(), i);
    }
    mapping
}

pub struct PossibleAction {
    pub action: Box<dyn Action>,
    pub(crate) requirements: Vec<ActionRequirement>,
}

impl PossibleAction {
    pub fn check(
        &self,
        actor_list: &HashMap<usize, &mut Actor>,
        actor: usize,
        scene: &Scene,
    ) -> bool {
        for r in &self.requirements {
            if !r.check(actor_list, actor, scene) {
                return false;
            }
        }
        true
    }
}

pub enum ActionRequirement {
    ActorStatus(String, Status),
    ActorResourceMin(String, f32),
    ActorResourceMax(String, f32),
    Target(Box<ActionRequirement>),
}

impl ActionRequirement {
    pub fn check(
        &self,
        actor_list: &HashMap<usize, &mut Actor>,
        actor_id: usize,
        _scene: &Scene,
    ) -> bool {
        match self {
            Self::ActorResourceMin(identifier, min) => {
                if let Some(actor) = actor_list.get(&actor_id) {
                    if let Some(v) = actor.get_resource(identifier) {
                        return v.value >= *min;
                    }
                }
            }
            Self::ActorResourceMax(identifier, max) => {
                if let Some(actor) = actor_list.get(&actor_id) {
                    if let Some(v) = actor.get_resource(identifier) {
                        return v.value <= *max;
                    }
                }
            }
            Self::ActorStatus(identifier, _status) => {
                if let Some(actor) = actor_list.get(&actor_id) {
                    if let Some(_) = actor.get_status(identifier) {
                        return true;
                        // TODO: Statuses with values.
                    }
                }
            }
            _ => {}
        }
        true
    }
}

#[derive(Copy, Clone)]
pub enum ActionTarget {
    Actor,
    Target,
    Others,
    All,
}

#[derive(Copy, Clone)]
pub enum NextAction {
    Abort,
}

pub trait ActionClone {
    fn clone_box(&self) -> Box<dyn Action>;
}

impl<T> ActionClone for T
where
    T: 'static + Action + Clone,
{
    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Action> {
    fn clone(&self) -> Box<dyn Action> {
        self.clone_box()
    }
}

pub trait Action: ActionClone {
    fn apply(
        &self,
        actor_list: &mut HashMap<usize, &mut Actor>,
        source: usize,
        scene: &mut Scene,
    ) -> (Option<NextAction>, Vec<(usize, Box<dyn Action>)>);

    fn target(&self) -> Option<usize> {
        return None;
    }

    fn name(&self) -> String {
        String::new()
    }

    fn set_target(&mut self, _target: Option<usize>) {}
}
