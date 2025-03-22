use std::sync::{Arc, Mutex};

use nu_engine::command_prelude::*;
use nu_protocol::SyntaxShape;

use crate::state::State;

#[derive(Clone)]
pub struct InstanceCreate {
    state: Arc<Mutex<State>>,
}

impl InstanceCreate {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Self { state }
    }
}

impl Command for InstanceCreate {
    fn name(&self) -> &str {
        "instance create"
    }

    fn signature(&self) -> Signature {
        Signature::build("instance create")
            .required_named("hostname", SyntaxShape::String, "the machine name", None)
            .required_named("pool", SyntaxShape::String, "pool", None)
            // .input_output_types(vec![(Type::Nothing, Type::List)])
            .category(Category::Custom("serverness".into()))
    }

    fn description(&self) -> &str {
        "create a plan instance"
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
            .block_on(crate::generated_nu::execute_instance_create(
                engine_state,
                stack,
                call,
                &guard.client,
            ))
            .unwrap();

        Ok(results.into_pipeline_data())
    }
}
