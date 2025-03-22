use std::sync::{Arc, Mutex};

use nu_engine::command_prelude::*;
use nu_engine::get_full_help;

use crate::state::State;

#[derive(Clone)]
pub struct Instance {
    state: Arc<Mutex<State>>,
}

impl Instance {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Self { state }
    }
}

impl Command for Instance {
    fn name(&self) -> &str {
        "instance"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("instance")
            .input_output_types(vec![(Type::Nothing, Type::String)])
            .category(Category::Custom("serverness".into()))
    }

    fn description(&self) -> &str {
        "You must use one of the following subcommands. Using this command as-is will only produce this help message."
    }

    fn run(
        &self,
        engine_state: &nu_protocol::engine::EngineState,
        stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::engine::Call,
        input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        Ok(Value::string(get_full_help(self, engine_state, stack), call.head).into_pipeline_data())
    }
}
