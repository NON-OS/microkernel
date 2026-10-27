/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The Wi-Fi driver, whichever one this machine runs, and the control
//! protocol it answers.

mod call;
mod connect;
mod find;
mod fw_step;
mod intel_step;
mod join_text;
mod link;
mod reply;
mod scan;
mod send;
mod services;
mod stage;
mod stage_kind;
mod stop;
mod stop_words;

pub use crate::join_wire::{ConnectResult, Link};
pub use call::OP_STATUS;
pub use find::{find, Driver};
pub use fw_step::firmware_step_text;
pub use join_text::join_text;
pub use reply::WIFI_HDR;
pub use scan::{ScanOutcome, ScanStats};
pub use stage_kind::DriverStage;
pub use stop::StopWhy;
