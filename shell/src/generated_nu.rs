// The contents of this file are generated; do not modify them.

use crate::cmd::to_value;
use crate::state::State;
use nu_engine::command_prelude::*;
use serverness::*;
use std::sync::{Arc, Mutex};
pub async fn execute_api_key_list<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.api_key_list();
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_api_key_create<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.api_key_create();
    if let Some(value) = call.get_flag::<::std::string::String>(engine_state, stack, "label")? {
        request = request.body_map(|body| body.label(value.clone()))
    }
    if let Some(value) = call.get_flag::<std::path::PathBuf>(engine_state, stack, "json-body")? {
        let body_txt = std::fs::read_to_string(value).unwrap();
        let body_value = serde_json::from_str::<types::ApiKeyCreateParams>(&body_txt).unwrap();
        request = request.body(body_value);
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_api_key_describe<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.api_key_describe();
    if let Some(value) = call.get_flag::<::uuid::Uuid>(engine_state, stack, "id")? {
        request = request.id(value.clone());
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_api_key_destroy<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.api_key_destroy();
    if let Some(value) = call.get_flag::<::uuid::Uuid>(engine_state, stack, "id")? {
        request = request.id(value.clone());
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => return Ok(Value::nothing(span)),
    }
}
pub async fn execute_install_list<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.install_list();
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_install_create<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.install_create();
    if let Some(value) = call.get_flag::<std::path::PathBuf>(engine_state, stack, "json-body")? {
        let body_txt = std::fs::read_to_string(value).unwrap();
        let body_value = serde_json::from_str::<types::InstallCreateParams>(&body_txt).unwrap();
        request = request.body(body_value);
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_install_describe<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.install_describe();
    if let Some(value) = call.get_flag::<::uuid::Uuid>(engine_state, stack, "id")? {
        request = request.id(value.clone());
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_install_destroy<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.install_destroy();
    if let Some(value) = call.get_flag::<::uuid::Uuid>(engine_state, stack, "id")? {
        request = request.id(value.clone());
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => return Ok(Value::nothing(span)),
    }
}
pub async fn execute_instance_list<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.instance_list();
    if let Some(value) = call.get_flag::<::std::num::NonZeroU32>(engine_state, stack, "limit")? {
        request = request.limit(value.clone());
    }
    if let Some(value) = call.get_flag::<types::SortMode>(engine_state, stack, "sort")? {
        request = request.sort(value.clone());
    }
    let limit = call
        .get_flag::<std::num::NonZeroU32>(engine_state, stack, "limit")
        .map_or(usize::MAX, |x| match x {
            Some(x) => x.get() as usize,
            None => usize::MAX,
        } as usize);
    let mut stream = futures::StreamExt::take(request.stream(), limit);
    let mut results = vec![];
    loop {
        match futures::TryStreamExt::try_next(&mut stream).await {
            Err(r) => return Err(anyhow::Error::new(r)),
            Ok(None) => {
                return Ok(to_value(results, span).unwrap());
            }
            Ok(Some(value)) => {
                results.push(value);
            }
        }
    }
}
pub async fn execute_instance_create<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.instance_create();
    if let Some(value) = call.get_flag::<::std::string::String>(engine_state, stack, "hostname")? {
        request = request.body_map(|body| body.hostname(value.clone()))
    }
    if let Some(value) = call.get_flag::<::std::string::String>(engine_state, stack, "pool")? {
        request = request.body_map(|body| body.pool(value.clone()))
    }
    if let Some(value) = call.get_flag::<std::path::PathBuf>(engine_state, stack, "json-body")? {
        let body_txt = std::fs::read_to_string(value).unwrap();
        let body_value = serde_json::from_str::<types::InstanceCreateParams>(&body_txt).unwrap();
        request = request.body(body_value);
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_instance_describe<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.instance_describe();
    if let Some(value) = call.get_flag::<::uuid::Uuid>(engine_state, stack, "id")? {
        request = request.id(value.clone());
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_instance_destroy<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.instance_destroy();
    if let Some(value) = call.get_flag::<::uuid::Uuid>(engine_state, stack, "id")? {
        request = request.id(value.clone());
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => return Ok(Value::nothing(span)),
    }
}
pub async fn execute_pool_list<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.pool_list();
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_session_list<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.session_list();
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_session_destroy<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.session_destroy();
    if let Some(value) = call.get_flag::<::uuid::Uuid>(engine_state, stack, "id")? {
        request = request.id(value.clone());
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => return Ok(Value::nothing(span)),
    }
}
pub async fn execute_ssh_key_list<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.ssh_key_list();
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_ssh_key_create<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.ssh_key_create();
    if let Some(value) = call.get_flag::<::std::string::String>(engine_state, stack, "label")? {
        request = request.body_map(|body| body.label(value.clone()))
    }
    if let Some(value) =
        call.get_flag::<::std::string::String>(engine_state, stack, "public-key")?
    {
        request = request.body_map(|body| body.public_key(value.clone()))
    }
    if let Some(value) = call.get_flag::<std::path::PathBuf>(engine_state, stack, "json-body")? {
        let body_txt = std::fs::read_to_string(value).unwrap();
        let body_value = serde_json::from_str::<types::SshKeyCreateParams>(&body_txt).unwrap();
        request = request.body(body_value);
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_ssh_key_describe<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.ssh_key_describe();
    if let Some(value) = call.get_flag::<::uuid::Uuid>(engine_state, stack, "id")? {
        request = request.id(value.clone());
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_ssh_key_update<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.ssh_key_update();
    if let Some(value) = call.get_flag::<::uuid::Uuid>(engine_state, stack, "id")? {
        request = request.id(value.clone());
    }
    if let Some(value) = call.get_flag::<std::path::PathBuf>(engine_state, stack, "json-body")? {
        let body_txt = std::fs::read_to_string(value).unwrap();
        let body_value = serde_json::from_str::<types::SshKeyUpdateParams>(&body_txt).unwrap();
        request = request.body(body_value);
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_ssh_key_destroy<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.ssh_key_destroy();
    if let Some(value) = call.get_flag::<::uuid::Uuid>(engine_state, stack, "id")? {
        request = request.id(value.clone());
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => return Ok(Value::nothing(span)),
    }
}
pub async fn execute_user_list<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.user_list();
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_user_create<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.user_create();
    if let Some(value) = call.get_flag::<::std::string::String>(engine_state, stack, "label")? {
        request = request.body_map(|body| body.label(value.clone()))
    }
    if let Some(value) =
        call.get_flag::<::std::string::String>(engine_state, stack, "public-key")?
    {
        request = request.body_map(|body| body.public_key(value.clone()))
    }
    if let Some(value) = call.get_flag::<std::path::PathBuf>(engine_state, stack, "json-body")? {
        let body_txt = std::fs::read_to_string(value).unwrap();
        let body_value = serde_json::from_str::<types::UserCreateParams>(&body_txt).unwrap();
        request = request.body(body_value);
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_user_describe<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.user_describe();
    if let Some(value) = call.get_flag::<::uuid::Uuid>(engine_state, stack, "id")? {
        request = request.id(value.clone());
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_user_update<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.user_update();
    if let Some(value) = call.get_flag::<::uuid::Uuid>(engine_state, stack, "id")? {
        request = request.id(value.clone());
    }
    if let Some(value) = call.get_flag::<std::path::PathBuf>(engine_state, stack, "json-body")? {
        let body_txt = std::fs::read_to_string(value).unwrap();
        let body_value = serde_json::from_str::<types::UserUpdateParams>(&body_txt).unwrap();
        request = request.body(body_value);
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => {
            return Ok(to_value(r.into_inner(), span).unwrap());
        }
    }
}
pub async fn execute_user_destroy<'c>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'c nu_protocol::engine::Call<'c>,
    client: &serverness::Client,
) -> anyhow::Result<Value> {
    let span = call.head;
    let mut request = client.user_destroy();
    if let Some(value) = call.get_flag::<::uuid::Uuid>(engine_state, stack, "id")? {
        request = request.id(value.clone());
    }
    let result = request.send().await;
    match result {
        Err(r) => Err(anyhow::Error::new(r)),
        Ok(r) => return Ok(Value::nothing(span)),
    }
}
#[derive(Copy, Clone, Debug)]
pub enum NuCommand {
    ApiKeyList,
    ApiKeyCreate,
    ApiKeyDescribe,
    ApiKeyDestroy,
    InstallList,
    InstallCreate,
    InstallDescribe,
    InstallDestroy,
    InstanceList,
    InstanceCreate,
    InstanceDescribe,
    InstanceDestroy,
    PoolList,
    SessionList,
    SessionDestroy,
    SshKeyList,
    SshKeyCreate,
    SshKeyDescribe,
    SshKeyUpdate,
    SshKeyDestroy,
    UserList,
    UserCreate,
    UserDescribe,
    UserUpdate,
    UserDestroy,
}
impl NuCommand {
    pub fn iter() -> impl Iterator<Item = NuCommand> {
        vec![
            NuCommand::ApiKeyList,
            NuCommand::ApiKeyCreate,
            NuCommand::ApiKeyDescribe,
            NuCommand::ApiKeyDestroy,
            NuCommand::InstallList,
            NuCommand::InstallCreate,
            NuCommand::InstallDescribe,
            NuCommand::InstallDestroy,
            NuCommand::InstanceList,
            NuCommand::InstanceCreate,
            NuCommand::InstanceDescribe,
            NuCommand::InstanceDestroy,
            NuCommand::PoolList,
            NuCommand::SessionList,
            NuCommand::SessionDestroy,
            NuCommand::SshKeyList,
            NuCommand::SshKeyCreate,
            NuCommand::SshKeyDescribe,
            NuCommand::SshKeyUpdate,
            NuCommand::SshKeyDestroy,
            NuCommand::UserList,
            NuCommand::UserCreate,
            NuCommand::UserDescribe,
            NuCommand::UserUpdate,
            NuCommand::UserDestroy,
        ]
        .into_iter()
    }
}
