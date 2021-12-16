use bevy::prelude::*;
use scene::{Scene, SimpleCharacter, Actor, SimpleAction, process_scene};
use std::collections::HashMap;

mod scene;

fn main() {
    let mut scene = Scene::default();
    let mut actors: HashMap<usize,&mut  Box<dyn Actor>> = HashMap::new();
    let mut char1 : Box<dyn Actor> = Box::new(SimpleCharacter::new("One"));
    let mut char2: Box<dyn Actor> = Box::new(SimpleCharacter::new("Two"));
    actors.insert(1,&mut char1);
    actors.insert(2,&mut char2);
    scene.add_actor(1);
    scene.add_actor(2);
    actors.get_mut(&1).unwrap().actions_mut().push_back(Box::from( SimpleAction { name : "Action!".to_string()  }));
    actors.get_mut(&2).unwrap().actions_mut().push_back(Box::from( SimpleAction { name : "Action!".to_string()  }));
    process_scene(&mut scene, &mut actors);
}
