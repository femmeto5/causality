use crate::scene::action::NextAction;
use crate::scene::SceneOutput;
use crate::{Action, ActionTarget, Actor, Scene};
use std::collections::HashMap;

// Action List

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

// Change Resource

#[derive(Clone)]
pub struct ChangeResourceAction {
    target: Option<usize>,
    target_type: ActionTarget,
    action_name: String,
    resource_identifier: String,
    amount: f32,
    output: SceneOutput,
}

impl Action for ChangeResourceAction {
    fn apply(
        &self,
        actor_list: &mut HashMap<usize, &mut Actor>,
        source: usize,
        scene: &mut Scene,
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
        scene.output.push(self.output.clone());
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
        output: SceneOutput,
    ) -> Self {
        Self {
            target,
            target_type,
            action_name,
            resource_identifier,
            amount,
            output
        }
    }
}

// Print

#[derive(Clone)]
pub struct PrintAction {
    pub(crate) value: SceneOutput,
}

impl Action for PrintAction {
    fn apply(
        &self,
        _actor_list: &mut HashMap<usize, &mut Actor>,
        _source: usize,
        scene: &mut Scene,
    ) -> (Option<NextAction>, Vec<(usize, Box<dyn Action>)>) {
        scene.output.push(self.value.clone());
        (None, vec![])
    }
}
