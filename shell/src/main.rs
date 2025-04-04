mod cmd;
mod config;
mod context;
mod generated_nu;
mod state;
mod terminal;

use std::io::{BufReader, Write};
use std::{
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::Instant,
};

use cmd::help::Help;
use cmd::{Instance, InstanceCreate, InstanceList};
use context::create_context;
use nu_cli::{gather_parent_env_vars, read_plugin_file, EvaluateCommandsOpts};
use nu_path::AbsolutePathBuf;
use nu_protocol::{ByteStream, ByteStreamSource, ByteStreamType, PipelineData};
use state::{create_state, State};

use nu_protocol::{
    engine::{EngineState, Stack, StateWorkingSet},
    report_shell_error, span, IntoPipelineData, Signals, Span, Spanned, Value,
};

use clap::Parser;

#[derive(Clone, Debug, Parser)]
struct Cli {
    #[arg(short, long)]
    address: String,

    #[arg(short, long)]
    secret: String,

    #[arg(short, long)]
    command: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let mut ctx = create_context();
    gather_parent_env_vars(&mut ctx, get_init_cwd().as_path().as_ref());

    let mut stack = Stack::new();

    let state = create_state(args.address, args.secret);

    merge_serverness_delta(&mut ctx, state);

    let signals = Signals::new(Arc::new(AtomicBool::new(false)));
    let handler_signals = signals.clone();
    let context_signals = signals.clone();

    ctrlc::set_handler(move || {
        handler_signals.trigger();
    })
    .expect("Error setting Ctrl-C handler");

    ctx.set_signals(context_signals);

    if let Some(command) = args.command {
        let input = Value::nothing(Span::new(0, 0)).into_pipeline_data();

        ctx.generate_nu_constant();
        read_plugin_file(&mut ctx, None);

        let opts = EvaluateCommandsOpts {
            table_mode: None,
            error_style: None,
            no_newline: false,
        };

        let command = Spanned {
            item: command,
            span: Span::new(0, 0),
        };

        nu_cli::evaluate_commands(&command, &mut ctx, &mut stack, input, opts)
            .expect("Failed to run command");

        return Ok(());
    }

    ctx.is_interactive = true;

    #[cfg(unix)]
    terminal::acquire(ctx.is_interactive);

    read_plugin_file(&mut ctx, None);
    config::read_nu_config_file(&mut ctx, &mut stack);

    let _ = nu_cli::evaluate_repl(&mut ctx, stack, None, None, Instant::now());

    Ok(())
}

fn get_init_cwd() -> AbsolutePathBuf {
    std::env::current_dir()
        .ok()
        .and_then(|path| AbsolutePathBuf::try_from(path).ok())
        .or_else(|| {
            std::env::var("PWD")
                .ok()
                .and_then(|path| AbsolutePathBuf::try_from(path).ok())
        })
        .or_else(nu_path::home_dir)
        .expect("Failed to get current working directory")
}

fn merge_serverness_delta(context: &mut EngineState, state: Arc<Mutex<State>>) {
    let mut working_set = StateWorkingSet::new(context);

    working_set.add_decl(Box::new(Help));
    working_set.add_decl(Box::new(Instance::new(state.clone())));
    working_set.add_decl(Box::new(InstanceList::new(state.clone())));
    working_set.add_decl(Box::new(InstanceCreate::new(state.clone())));

    if let Err(err) = context.merge_delta(working_set.render()) {
        report_shell_error(&context, &err);
    }
}
