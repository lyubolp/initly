pub mod workflow {
    use crate::program_output::program_output::ProgramOutput;
    use crate::step::step::Step;
    use std::collections::VecDeque;

    pub struct Workflow {
        name: String,
        description: String,
        steps: Vec<Step>,
        arguments_count: u32,
    }

    impl Workflow {
        pub fn new(
            name: String,
            description: String,
            steps: Vec<Step>,
            arguments_count: u32,
        ) -> Self {
            Workflow {
                name,
                description,
                steps,
                arguments_count,
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

        pub fn run(&self, arguments: Option<Vec<String>>) -> Result<String, String> {
            // Check if arguments are enough in general

            let mut last_output = Err(String::from("No steps executed"));
            for step in self.steps.clone() {
                if step.get_argument_ids().len() == 0 {
                    last_output = step.execute(None);
                } else {
                    let filtered_arguments =
                        Workflow::filter_arguments_for_step(&step, &arguments.as_ref().unwrap());
                    last_output = step.execute(Some(&filtered_arguments))
                }

                match last_output {
                    Ok(program_output) => {
                        println!(
                            "Executed {} , rc = {}, stdout = {}, stderr = {}",
                            step.get_name(),
                            program_output.get_exit_code(),
                            program_output.get_stdout(),
                            program_output.get_stderr()
                        );

                        if program_output.get_exit_code() != 0 {
                            return Err(String::from("Error"));
                        }
                    }

                    Err(error) => return Err(error),
                }
            }
            Ok(String::from("All good !"))
        }

        pub fn filter_arguments_for_step(step: &Step, arguments: &Vec<String>) -> VecDeque<String> {
            arguments
                .iter()
                .enumerate()
                .filter(|(index, _)| step.get_argument_ids().contains(&(index + 1)))
                .map(|(_, item)| item.clone())
                .collect()
        }
    }
}
