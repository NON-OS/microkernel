// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The TCG event log, crypto-agile format, as far as the PCR 4 check reads it:
//! the Spec ID header naming the banks, then one `TCG_PCR_EVENT2` after another.

mod banks;
mod consts;
mod error;
mod event;
mod grow;
mod read;
mod replay;
mod spec_id;

pub use banks::Banks;
pub use consts::{
    EV_EFI_BOOT_SERVICES_APPLICATION, EV_NO_ACTION, MAX_ALGS, MAX_DIGEST, MAX_EVENTS,
    MAX_EVENT_BYTES, MAX_LOG_BYTES, PCR_BOOT_MANAGER, TPM_ALG_SHA256,
};
pub use error::LogError;
pub use event::{event, event_walk, Event};
pub use grow::{grow, Walk};
pub use replay::{replay, Replay};
pub use spec_id::{spec_id, spec_id_walk};
