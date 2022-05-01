pub mod step {
    use std::collections::VecDeque;
    use regex::Regex;
    use crate::executor::executor::execute_command;
    use crate::program_output::program_output::ProgramOutput;

    #[derive(Clone, Debug)]
    pub struct Step {
        name: String,
        command: String,
        amount_expected_arguments: u32,
        real_argument_regex: Regex
    }
    // TODO - Add a way to get the exact needed argument. Check the jupyter.toml and 'Create file $2'
    impl Step {
        pub fn new(name: &str, command: &str, amount_expected_arguments: u32) -> Self {
            Step {
                name: String::from(name),
                command: String::from(command),
                amount_expected_arguments,
                real_argument_regex: Regex::new(r"(\$\d*)").unwrap()
            }
        }

        pub fn from_command_string(command_string: &str) -> Self {
            Step {
                name: String::from("Shell command"),
                command: String::from(command_string.split_at(1).1),
                amount_expected_arguments: command_string.chars().filter(|c: &char| *c == '$').count() as u32,
                real_argument_regex: Regex::new(r"(\$\d*)").unwrap()
            }
        }
        pub fn execute(&self, arguments: Option<&VecDeque<String>>) -> Result<ProgramOutput, String> {
            // arguments being Option is not ideal, however Rust has no default args.
            // Think about improving this if
            if arguments.is_none() || arguments.unwrap().len() == self.amount_expected_arguments as usize {
                let (command, args): (String, Vec<String>) = self.build_final_command(arguments);
                execute_command(command.as_str(), args.iter().map(|item: &String| item.as_str()).collect())
            } else {
                Err(String::from("Not enough arguments passed"))
            }
        }

        fn build_final_command(&self, actual_arguments: Option<&VecDeque<String>>) -> (String, Vec<String>) {
            let mut commands: Vec<String> = shlex::split(self.command.as_str()).unwrap();

            let arguments: Vec<String> = match actual_arguments {
                Some(args) => {
                    let mut args = args.clone();
                    commands
                        .split_off(1)
                        .iter()
                        .map(|item: &String|
                            if self.real_argument_regex.is_match(item) {
                                self.real_argument_regex.replace(item, &(args.pop_front().unwrap())).to_string()
                            } else {
                                item.to_string()
                            }
                        )
                        .collect()
                }
                None => {
                    commands.split_off(1)
                }
            };
            (commands[0].clone(), arguments)
        }
    }
}