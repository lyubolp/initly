pub mod executor {
    use crate::program_output::program_output::ProgramOutput;
    use std::process::Command;

    pub fn execute_command(command: &str, arguments: Vec<&str>) -> Result<ProgramOutput, String> {
        let output = Command::new(command)
            .args(arguments)
            .output();

        match output {
            Ok(process) => {
                match process.status.code() {
                    Some(exit_code) => Ok(ProgramOutput::new(process.stdout, process.stderr, exit_code)),
                    None => Err(String::from("Process has been terminated by a signal"))
                }
            },
            Err(err) => {
                let mut result = String::from("Can't execute the command. Error: ").to_owned();
                result.push_str(err.to_string().as_str());
                Err(result)
            }
        }
    }
}