use scene::action::{ActionList, ActionTarget, ChangeResourceAction};
use scene::actor::{Actor, Resource};
use scene::{process_scene, Scene, SceneStatus};
use std::collections::HashMap;

mod scene;

fn main() {
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
