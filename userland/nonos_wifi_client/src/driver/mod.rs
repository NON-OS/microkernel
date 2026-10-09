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
mod join_text;
mod link;
mod scan;
mod services;
mod stage;

pub use call::{OP_STATUS, WIFI_HDR};
pub use crate::join_wire::{ConnectResult, Link};
pub use find::{find, Driver};
pub use join_text::join_text;
pub use scan::{ScanOutcome, ScanStats};
pub use fw_step::firmware_step_text;
pub use stage::DriverStage;
