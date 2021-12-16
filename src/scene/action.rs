pub struct Action {
    name : String
}

impl Action {

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn generate_output(&self) -> String {
        self.name().clone()
    }
}
