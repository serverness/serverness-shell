use indoc::indoc;
use nu_command::{HelpAliases, HelpCommands, HelpModules};
use nu_engine::command_prelude::Call;
use nu_engine::documentation::HelpStyle;
use nu_engine::CallExt;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    Category, Example, IntoPipelineData, PipelineData, ShellError, Signature, Span, Spanned,
    SyntaxShape, Type, Value,
};

#[derive(Clone)]
pub struct Help;

impl Command for Help {
    fn name(&self) -> &str {
        "help"
    }

    fn signature(&self) -> Signature {
        Signature::build("help")
            .input_output_types(vec![(Type::Nothing, Type::String)])
            .rest(
                "rest",
                SyntaxShape::String,
                "the name of command, alias or module to get help on",
            )
            .named(
                "find",
                SyntaxShape::String,
                "string to find in command names, usage, and search terms",
                Some('f'),
            )
            .category(Category::Core)
    }

    fn description(&self) -> &str {
        "Display help information about different parts of Nushell."
    }

    fn extra_description(&self) -> &str {
        r#"`help word` searches for "word" in commands, aliases and modules, in that order."#
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let head = call.head;
        let find: Option<Spanned<String>> = call.get_flag(engine_state, stack, "find")?;
        let rest: Vec<Spanned<String>> = call.rest(engine_state, stack, 0)?;

        if rest.is_empty() && find.is_none() {
            let msg = indoc! {"
                \x1b[32mWelcome to Serverness Shell!\x1b[0m

                Here are some tips to help you get started:
                * help commands - list all available commands
                * help <command name> - display help about a particular command
                * help commands | where category == serverness - list all available Serverness commands

                [Examples]

                Get all instances:
                    instance list
                
                Get all columns:
                    instance list | columns
                
                Select specific columns:
                    instance list | select id hostname status created_at

                Filter by status:
                    instance list | select id hostname status created_at | where status == active

                You can also learn more at https://serverness.io/docs/ and https://www.nushell.sh/book/
            "};

            Ok(Value::string(msg, head).into_pipeline_data())
        } else if find.is_some() {
            HelpCommands {}.run(engine_state, stack, call, PipelineData::Empty)
        } else {
            let result = HelpAliases {}.run(engine_state, stack, call, PipelineData::Empty);

            let result = if let Err(ShellError::AliasNotFound { .. }) = result {
                HelpCommands {}.run(engine_state, stack, call, PipelineData::Empty)
            } else {
                result
            };

            let result = if let Err(ShellError::CommandNotFound { .. }) = result {
                HelpModules.run(engine_state, stack, call, PipelineData::Empty)
            } else {
                result
            };

            if let Err(ShellError::ModuleNotFoundAtRuntime { .. }) = result {
                let rest_spans: Vec<Span> = rest.iter().map(|arg| arg.span).collect();
                Err(ShellError::NotFound {
                    span: Span::concat(&rest_spans),
                })
            } else {
                result
            }
        }
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "show help for single command, alias, or module",
                example: "help match",
                result: None,
            },
            Example {
                description: "show help for single sub-command, alias, or module",
                example: "help str lpad",
                result: None,
            },
            Example {
                description: "search for string in command names, usage and search terms",
                example: "help --find char",
                result: None,
            },
        ]
    }
}
