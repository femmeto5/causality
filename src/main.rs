use crate::scene::action::{
    build_action_option_mapping, Action, ActionRequirement, PossibleAction, PossibleActionSet,
};
use scene::action::{ActionList, ActionTarget, ChangeResourceAction};
use scene::actor::{Actor, Resource};
use scene::{process_scene, Scene, SceneStatus};
use std::collections::HashMap;
use crate::scene::SceneOutput;

mod scene;

fn main() {
    let mut scene = Scene::default();
    let mut actors = Vec::new();
    actors.push(Actor::new("User Actor"));
    let user_actor = 0;
    let mut possible_actions = PossibleActionSet {
        possible_actions: vec![],
    };
    scene.add_actor(user_actor);
    println!("{:?}", &scene);

    possible_actions.possible_actions.push(PossibleAction {
        action: Box::from(ChangeResourceAction::new(
            None,
            ActionTarget::Actor,
            "Action".to_string(),
            "health".to_string(),
            10.,
            SceneOutput {
                content: "Changing resource by 10".to_string()
            },
        )),
        requirements: vec![],
    });
    let mut actor_list = HashMap::new();
    actor_list.insert(user_actor, &mut actors[0]);
    loop {
        if scene.status == SceneStatus::RequireUserInput {
            let action_list: Vec<Box<dyn Action>> =
                possible_actions.build_action_set(&mut actor_list, user_actor, &scene);
            let action_option_mapping = build_action_option_mapping(&action_list);
            for (identifier, idx) in &action_option_mapping {
                println!("{} : {}", identifier, action_list.get(*idx).unwrap().name());
            }
            let mut line = String::new();
            let _ = std::io::stdin().read_line(&mut line).unwrap();
            let line = line.trim().to_string();
            println!("{:?}", action_option_mapping.keys());
            println!("{:?}", &line);
            if let Some(idx) = action_option_mapping.get(&line) {
                if let Some(actor) = actor_list.get_mut(&user_actor) {
                    let action: &Box<dyn Action> = action_list.get(*idx).unwrap();
                    actor.actions_mut().push_back(action.clone());
                }
            } else {
                println!("Invalid Input");
            }

            scene.status = SceneStatus::RequireAIInput;
        } else if scene.status == SceneStatus::RequireAIInput {
            if scene.actors().len() <= 1 {
                scene.status = SceneStatus::Ready;
                continue;
            }
            println!("AI Action Generation not implemented yet");
            scene.status = SceneStatus::Ready;
        } else if scene.status == SceneStatus::Ready {
            process_scene(&mut scene, &mut actor_list);
            scene.status = SceneStatus::RequireUserInput;
        }
    }
}

#[allow(dead_code)]
fn action_list_test() {
    let scene = Scene::default();
    let possible_action = PossibleAction {
        action: Box::from(ChangeResourceAction::new(
            None,
            ActionTarget::Actor,
            "test".to_string(),
            "health".to_string(),
            10.,
            SceneOutput {
                content: "Changing resource by 10".to_string()
            },
        )),
        requirements: vec![ActionRequirement::ActorResourceMax(
            "health".to_string(),
            50.,
        )],
    };
    let possible_action_set = PossibleActionSet {
        possible_actions: vec![possible_action],
    };
    let mut actor1 = Actor::new("One");
    actor1.insert_resource(Resource::new("Health", 100., 0., 100.));
    let mut actors: HashMap<usize, &mut Actor> = HashMap::new();
    actors.insert(1, &mut actor1);
    let mut actor2 = Actor::new("Two");
    actor2.insert_resource(Resource::new("Health", 0., 0., 100.));
    actors.insert(2, &mut actor2);
    let actions = possible_action_set.build_action_set(&actors, 1, &scene);
    println!("{}", actions.len());
    let actions = possible_action_set.build_action_set(&actors, 2, &scene);
    println!("{}", actions.len());
}

#[allow(dead_code)]
fn scene_test() {
    let mut scene = Scene::default();
    let mut actor1 = Actor::new("One");
    let mut actor2 = Actor::new("Two");
    actor1.insert_resource(Resource::new("Health", 100., 0., 100.));
    actor2.insert_resource(Resource::new("Health", 100., 0., 100.));
    let res_action = Box::from(ChangeResourceAction::new(
        Some(1),
        ActionTarget::Target,
        String::new(),
        "health".to_lowercase(),
        10.,
        SceneOutput {
            content: "Changing resource by 10".to_string()
        },
    ));
    let mut action_list = ActionList::default();
    action_list.add_action(res_action.clone());
    action_list.add_action(res_action.clone());
    actor2
        .actions_mut()
        .push_back(Box::from(ChangeResourceAction::new(
            Some(1),
            ActionTarget::Target,
            String::new(),
            "health".to_lowercase(),
            -10.,
            SceneOutput {
                content: "Changing resource by 10".to_string()
            },
        )));
    actor2.actions_mut().push_back(Box::from(action_list));
    actor1
        .actions_mut()
        .push_back(Box::from(ChangeResourceAction::new(
            None,
            ActionTarget::All,
            String::new(),
            "health".to_lowercase(),
            -10.,
            SceneOutput {
                content: "Changing resource by 10".to_string()
            },
        )));
    let mut actors: HashMap<usize, &mut Actor> = HashMap::new();
    actors.insert(1, &mut actor1);
    actors.insert(2, &mut actor2);
    scene.add_actor(1);
    scene.add_actor(2);


    let mut round = 1;

    loop {
        if scene.status == SceneStatus::RequireUserInput {
            let mut line = String::new();
            let _ = std::io::stdin().read_line(&mut line).unwrap();
            scene.status = SceneStatus::RequireAIInput;
        } else if scene.status == SceneStatus::RequireAIInput {
            scene.status = SceneStatus::Ready;
        } else if scene.status == SceneStatus::Ready {
            process_scene(&mut scene, &mut actors);
            round += 1;
            for (_, actor) in &mut actors {
                for r in actor.resources_mut() {
                    println!("{:?}", r);
                }
            }
            println!();
            println!("{}", round);
        }
    }
}
