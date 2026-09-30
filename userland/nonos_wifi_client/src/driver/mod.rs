/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The Wi-Fi driver, whichever one this machine runs, and the control
//! protocol it answers.

mod call;
mod connect;
mod find;
mod join_text;
mod link;
mod scan;
mod stage;

pub use call::{OP_STATUS, WIFI_HDR};
pub use connect::ConnectResult;
pub use find::{find, Driver};
pub use join_text::join_text;
pub use link::Link;
pub use scan::{ScanOutcome, ScanStats};
pub use stage::DriverStage;
