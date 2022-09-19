mod executor;
mod program_output;
mod step;
mod workflow_parser;
mod workflow;

use crate::step::step::Step;
use crate::workflow::workflow::Workflow;
use crate::workflow_parser::workflow_parser::WorkflowParser;

fn main() {
    let path = "/home/lyubo/initly/workflows/jupyter.toml";

    let workflow_parser = WorkflowParser::new();
    let workflow = workflow_parser.read_workflow(path).unwrap();

    let args = Some(vec![String::from("/tmp/initly"), String::from("temp_project")]);

    println!("{:?}", Workflow::filter_arguments_for_step(&workflow.get_steps()[4], &args.unwrap()));
    let result = workflow.run(args);
    
    match result {
        Ok(program_output) => println!("Success !"),
        Err(error) => println!("{}", error)
    };
}
