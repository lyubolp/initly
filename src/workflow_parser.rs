pub mod workflow_parser {
    use std::collections::HashMap;
    use crate::Step;
    use crate::workflow::workflow::Workflow;
    use std::fs::read_to_string;
    use toml::Value;

    pub struct WorkflowParser {
        built_in_steps: HashMap<String, Step>,
    }

    impl WorkflowParser {
        pub fn new() -> Self {
            WorkflowParser {
                built_in_steps: HashMap::from([
                    (String::from("Create a directory $1"), Step::new("Create a directory", "mkdir $1", 1)),
                    (String::from("Change directory $1"), Step::new("Change directory", "cd $1", 1)),
                    (String::from("Create python venv"), Step::new("Create python venv", "python3 -m venv .venv", 0)),
                    (String::from("Activate venv"), Step::new("Activate venv", "source .venv/bin/activate", 0)),
                    (String::from("Create file $2"), Step::new("Create file", "touch $2", 1))
                ])
            }
        }
        pub fn read_workflow(&self, path: &str) -> Result<Workflow, String> {
            let content = read_to_string(path);

            match content {
                Ok(file_content) => self.read_workflow_from_content(file_content),
                Err(_) => Err(String::from("Cannot open file"))
            }
        }

        fn read_workflow_from_content(&self, file_content: String) -> Result<Workflow, String> {
            match file_content.as_str().parse::<Value>() {
                Ok(values) => {
                    let name: Option<&str> = values["name"].as_str();
                    let description: Option<&str> = values["description"].as_str();
                    let steps: Option<&Vec<Value>> = values["steps"].as_array();
                    let arguments_count = values["arguments"].as_integer();

                    if name.is_none() || description.is_none() || steps.is_none() || arguments_count.is_none() {
                        Err(String::from("Cannot parse file"))
                    } else {
                        Ok(Workflow::new(String::from(name.unwrap()),
                                         String::from(description.unwrap()),
                                         steps.unwrap().iter()
                                             .map(|item: &Value| item.as_str().unwrap()) // This is not ideal
                                             .map(|command_string: &str| {
                                                 if self.built_in_steps.contains_key(command_string) {
                                                     self.built_in_steps.get(command_string).unwrap().clone()
                                                 } else {
                                                     Step::from_command_string(command_string)
                                                 }
                                             })
                                             .collect(),
                                         arguments_count.unwrap() as u32,
                        ))
                    }
                }
                Err(_) => Err(String::from("Cannot parse file"))
            }
        }
    }
}