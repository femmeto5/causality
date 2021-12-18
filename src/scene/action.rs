use crate::scene::actor::{Actor, Status};
use crate::scene::Scene;
use std::collections::{HashMap, HashSet};

pub struct PossibleActionSet {
    pub(crate) possible_actions : Vec<PossibleAction>
}

impl PossibleActionSet {
    pub fn build_action_set(&self, actor_list : &HashMap<usize, &mut Actor>, actor : usize, scene : &Scene) -> Vec<Box<dyn Action>>{
        let mut action_set = Vec::new();
        for possible_action in &self.possible_actions {
            if possible_action.check(actor_list, actor, scene) {
                action_set.push(possible_action.action.clone());
            }
        }
        action_set
    }
}

pub struct PossibleAction {
    pub action : Box<dyn Action>,
    pub(crate) requirements : Vec<ActionRequirement>
}

impl PossibleAction {
    pub fn check(&self, actor_list : &HashMap<usize, &mut Actor>, actor : usize, scene : &Scene) -> bool {
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
    Target(Box<ActionRequirement>)
}

impl ActionRequirement {
    pub fn check(&self, actor_list : &HashMap<usize, &mut Actor>, actor_id : usize, scene : &Scene) -> bool {
        match self {
            Self::ActorResourceMin(identifier, min) => {
                if let Some(actor) = actor_list.get(&actor_id) {
                    if let Some(v) = actor.get_resource(identifier) {
                        return v.value >= *min;
                    }
                }
            },
            Self::ActorResourceMax(identifier, max) => {
                if let Some(actor) = actor_list.get(&actor_id) {
                    if let Some(v) = actor.get_resource(identifier) {
                        return v.value <= *max;
                    }
                }
            },
            Self::ActorStatus(identifier, status ) => {
                if let Some(actor) = actor_list.get(&actor_id) {
                    if let Some(_) = actor.get_status(identifier) {
                        return true;
                        // TODO: Statuses with values.
                    }
                }
            },
            _ => {
            }
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

impl<T> ActionClone for T where T: 'static + Action + Clone {
    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Action> {
    fn clone(&self) -> Box<dyn Action> {
        self.clone_box()
    }
}

pub trait Action : ActionClone {
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

#[derive(Default, Clone)]
pub struct ActionList {
    actions: Vec<Box<dyn Action>>,
}

impl Action for ActionList {
    fn apply(
        &self,
        actor_list: &mut HashMap<usize, &mut Actor>,
        source: usize,
        scene: &mut Scene,
    ) -> (Option<NextAction>, Vec<(usize, Box<dyn Action>)>) {
        let mut reactions = Vec::new();
        for action in &self.actions {
            let (next, mut reaction) = action.apply(actor_list, source, scene);
            reactions.append(&mut reaction);
            if let Some(NextAction::Abort) = next {
                break;
            }
        }
        (None, reactions)
    }
}

impl ActionList {
    pub fn add_action(&mut self, action: Box<dyn Action>) {
        self.actions.push(action);
    }
}

#[derive(Clone)]
pub struct ChangeResourceAction {
    target: Option<usize>,
    target_type: ActionTarget,
    action_name: String,
    resource_identifier: String,
    amount: f32,
}

impl Action for ChangeResourceAction {
    fn apply(
        &self,
        actor_list: &mut HashMap<usize, &mut Actor>,
        source: usize,
        _scene: &mut Scene,
    ) -> (Option<NextAction>, Vec<(usize, Box<dyn Action>)>) {
        let targets = match self.target_type {
            ActionTarget::Actor => {
                vec![source]
            }
            ActionTarget::Target => {
                if let Some(t) = self.target {
                    vec![t]
                } else {
                    vec![]
                }
            }
            ActionTarget::All => actor_list.iter().map(|(k, _)| *k).collect::<Vec<usize>>(),
            _ => {
                todo!()
            }
        };
        for target in targets {
            if let Some(actor) = actor_list.get_mut(&target) {
                actor.change_resource(&self.resource_identifier, self.amount);
            }
        }
        (None, vec![])
    }

    fn target(&self) -> Option<usize> {
        self.target
    }

    fn name(&self) -> String {
        self.action_name.clone()
    }

    fn set_target(&mut self, target: Option<usize>) {
        self.target = target;
    }
}

impl ChangeResourceAction {
    pub fn new(
        target: Option<usize>,
        target_type: ActionTarget,
        action_name: String,
        resource_identifier: String,
        amount: f32,
    ) -> Self {
        Self {
            target,
            target_type,
            action_name,
            resource_identifier,
            amount,
        }
    }
}
