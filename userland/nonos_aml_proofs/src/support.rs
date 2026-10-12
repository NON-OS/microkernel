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

//! The proofs' own scaffolding: AML encoders, the probe host, the register
//! and EC models, and the runners. Kept out of `lib.rs`, which mirrors the
//! crate's modules, and brought to the crate root there so tests name them
//! as `crate::enc` and the like.

#[path = "consts.rs"]
pub mod consts;
#[path = "ec_model.rs"]
pub mod ec_model;
#[path = "ec_model_io.rs"]
pub mod ec_model_io;
#[path = "enc/mod.rs"]
pub mod enc;
#[path = "fadt_enc.rs"]
pub mod fadt_enc;
#[path = "flat.rs"]
pub mod flat;
#[path = "probe.rs"]
pub mod probe;
#[path = "probe_host.rs"]
pub mod probe_host;
#[path = "probe_lock.rs"]
pub mod probe_lock;
#[path = "regs.rs"]
pub mod regs;
#[path = "regs_io.rs"]
pub mod regs_io;
#[path = "regs_shared.rs"]
pub mod regs_shared;
#[path = "rig.rs"]
pub mod rig;
#[path = "run.rs"]
pub mod run;
