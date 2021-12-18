use crate::action::Action;
use std::collections::{HashMap, VecDeque};

pub struct Actor {
    name: String,
    actions: VecDeque<Box<dyn Action>>,
    resources: HashMap<String, Resource>,
    statuses: HashMap<String, Status>,
    target: Option<usize>,
}

impl Actor {
    pub fn new(name: &str) -> Self {
        Actor {
            name: name.to_string(),
            actions: VecDeque::new(),
            resources: HashMap::new(),
            statuses: HashMap::new(),
            target: None,
        }
    }

    pub fn resources_mut(&mut self) -> &mut HashMap<String, Resource> {
        &mut self.resources
    }

    pub fn insert_resource(&mut self, resource: Resource) {
        self.resources
            .insert(resource.name().to_lowercase(), resource);
    }

    pub fn actions_mut(&mut self) -> &mut VecDeque<Box<dyn Action>> {
        &mut self.actions
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn get_target(&self) -> Option<usize> {
        self.target
    }

    pub fn change_resource(&mut self, identifier: &str, amount: f32) {
        if let Some(r) = self.resources.get_mut(identifier) {
            r.change(amount);
        }
    }

    pub fn get_resource(&self, identifier: &str) -> Option<&Resource> {
        self.resources.get(identifier)
    }

    pub fn get_status(&self, identifier: &str) -> Option<&Status> {
        self.statuses.get(identifier)
    }
}

#[derive(Debug)]
pub struct Resource {
    name: String,
    pub(crate) value: f32,
    min: f32,
    max: f32,
}

impl Default for Resource {
    fn default() -> Self {
        Self {
            name: String::new(),
            value: 0.,
            min: 0.,
            max: 100.,
        }
    }
}

impl Resource {
    pub fn new(name: &str, value: f32, min: f32, max: f32) -> Self {
        Self {
            name: name.to_string(),
            value,
            min,
            max,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn change(&mut self, amount: f32) {
        self.change_to(self.value + amount);
    }

    pub fn change_to(&mut self, new_value: f32) {
        self.change_to_minmax(new_value, (self.min, self.max));
    }

    pub fn change_minmax(&mut self, amount: f32, minmax: (f32, f32)) {
        self.change_to_minmax(self.value + amount, minmax);
    }

    pub fn change_to_minmax(&mut self, new_value: f32, minmax: (f32, f32)) {
        if new_value > minmax.1 {
            self.value = minmax.1;
        } else if new_value < minmax.0 {
            self.value = minmax.0
        } else {
            self.value = new_value;
        }
    }
}

pub enum Status {
    Tag,
    Value(f32),
}
