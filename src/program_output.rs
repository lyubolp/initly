pub mod program_output {
    #[derive(Debug, Eq, PartialEq)]
    pub struct ProgramOutput{
        stdout: String,
        stderr: String,
        exit_code: i32
    }

    impl ProgramOutput {
        pub fn new(out: Vec<u8>, err: Vec<u8>, exit_code: i32) -> ProgramOutput{
            ProgramOutput {
                stdout: String::from_utf8(out).unwrap(),
                stderr: String::from_utf8(err).unwrap(),
                exit_code
            }
        }

        pub fn get_stdout(&self) -> String {
            self.stdout.clone()
        }

        pub fn get_stderr(&self) -> String {
            self.stderr.clone()
        }

        pub fn get_exit_code(&self) -> i32 {
            self.exit_code
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::program_output::program_output::ProgramOutput;

    #[test]
    fn test_creation() {
        let expected_out: String = String::from("Expected out");
        let expected_err: String = String::from("Expected err");
        let expected_rc: i32 = 0;

        let instance: ProgramOutput = ProgramOutput::new(expected_out.clone().into_bytes(), expected_err.clone().into_bytes(), expected_rc);

        assert_eq!(instance.get_stdout(), expected_out);
        assert_eq!(instance.get_stderr(), expected_err);
        assert_eq!(instance.get_exit_code(), expected_rc);
    }
}