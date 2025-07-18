pub mod token_admin;
pub mod token_requester;

use std::sync::Arc;

use anyhow::Result;
use cainome::cairo_serde::ContractAddress;
use serde::{Deserialize, Serialize};
use starkbiter_core::middleware::StarkbiterMiddleware;
use starkbiter_engine::{
    machine::{Behavior, ControlFlow, CreateStateMachine, Engine, EventStream, StateMachine},
    messager::{Message, Messager, To},
};
use starkbiter_macros::Behaviors;
use tracing::{debug, error, trace, warn};

#[derive(Behaviors, Debug, Clone, Serialize, Deserialize)]
pub enum Behaviors {
    TokenAdmin(token_admin::TokenAdmin),
    TokenRequester(token_requester::TokenRequester),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub address: Option<ContractAddress>,
}
