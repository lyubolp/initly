pub mod workflow {
    use crate::Step;

    pub struct Workflow {
        name: String,
        description: String,
        steps: Vec<Step>
    }

    impl Workflow {
        pub fn new(name: String, description: String, steps: Vec<Step>) -> Self {
            Workflow {
                name,
                description,
                steps
            }
        }

        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        pub fn get_description(&self) -> String {
            self.description.clone()
        }

        pub fn get_steps(&self) -> Vec<Step> {
            self.steps.clone()
        }
    }
}