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

//! The modern bring-up over struct virtio_pci_common_cfg.
//!
//! Every step takes the common configuration as a `CommonCfg`, which the
//! mapped region implements with volatile accesses and the proofs implement
//! with a model device, so the order of the handshake, the features taken
//! and the queue programming are proved without hardware.

mod access;
mod generation;
mod negotiate;
mod queue;
mod regs;
mod status;
mod vector;

pub use access::CommonCfg;
pub use generation::{stable_read, GENERATION_TRIES};
pub use negotiate::{accept, device_features, write_driver_features};
pub use queue::{choose_size, queue_max, setup_queue, NotifyArea, QueueReady, QueueSpec};
pub use regs::*;
pub use status::{
    driver_ok, fail, reset, start, RESET_POLLS, STATUS_ACKNOWLEDGE, STATUS_DRIVER,
    STATUS_DRIVER_OK, STATUS_FAILED, STATUS_FEATURES_OK,
};
pub use vector::{set_config_vector, NO_VECTOR};
