use std::sync::{Arc, Mutex};

use nu_engine::command_prelude::*;
use nu_engine::get_full_help;

use crate::state::State;

#[derive(Clone)]
pub struct InstanceList {
    state: Arc<Mutex<State>>,
}

impl InstanceList {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Self { state }
    }
}

impl Command for InstanceList {
    fn name(&self) -> &str {
        "instance list"
    }

    fn signature(&self) -> Signature {
        Signature::build("instance list")
            // .input_output_types(vec![(Type::Nothing, Type::List)])
            .category(Category::Custom("serverness".into()))
    }

    fn description(&self) -> &str {
        "list active instances"
    }

    fn run(
        &self,
        engine_state: &nu_protocol::engine::EngineState,
        stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::engine::Call,
        input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        let guard = self.state.lock().unwrap();

        let results = guard
            .runtime
            .block_on(crate::generated_nu::execute_instance_list(
                engine_state,
                stack,
                call,
                &guard.client,
            ))
            .unwrap();

        Ok(results.into_pipeline_data())
    }
}
