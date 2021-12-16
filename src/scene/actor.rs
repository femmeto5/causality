use std::collections::VecDeque;
use crate::scene::action::Action;

pub struct Actor {
    name : String,
    actions : VecDeque<Action>
}

impl Actor {
    pub fn new(name: &str) -> Self {
        Actor {
            name: name.to_string(),
            actions: VecDeque::new()
        }
    }

    pub fn actions_mut(&mut self) -> &mut VecDeque<Action> {
        &mut self.actions
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
