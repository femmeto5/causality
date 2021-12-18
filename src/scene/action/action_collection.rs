use crate::scene::action::PrintAction;
use crate::scene::SceneOutput;
use crate::{ActionList, ActionTarget, ChangeResourceAction, PossibleAction, Scene};

pub struct ChangeResourceDummy {}

impl ChangeResourceDummy {
    pub fn build() -> PossibleAction {
        let mut action_list = ActionList::default();
        action_list.add_action(Box::from(ChangeResourceAction::new(
            None,
            ActionTarget::Target,
            "ChangeResourceDummy".to_string(),
            "health".to_string(),
            -10.,
            SceneOutput {
                content: "Reducing Resource by 10".to_string(),
            },
        )));
        PossibleAction {
            action: Box::from(action_list),
            requirements: vec![],
        }
    }
}
