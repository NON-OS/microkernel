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

//! The machine key's files, all of them but the kernel label: enrollment
//! reaches into create, derive's lock, flush, run, session and wire.

#[path = "../../../../../../src/security/tpm/machine_key/consts.rs"]
pub mod consts;
#[path = "../../../../../../src/security/tpm/machine_key/create.rs"]
pub mod create;
#[path = "../../../../../../src/security/tpm/machine_key/derive.rs"]
pub mod derive;
#[path = "../../../../../../src/security/tpm/machine_key/error.rs"]
pub mod error;
#[path = "../../../../../../src/security/tpm/machine_key/flush.rs"]
pub mod flush;
#[path = "../../../../../../src/security/tpm/machine_key/hmac.rs"]
pub mod hmac;
#[path = "../../../../../../src/security/tpm/machine_key/pcrs.rs"]
pub mod pcrs;
#[path = "../../../../../../src/security/tpm/machine_key/policy.rs"]
pub mod policy;
#[path = "../../../../../../src/security/tpm/machine_key/run.rs"]
pub mod run;
#[path = "../../../../../../src/security/tpm/machine_key/session.rs"]
pub mod session;
#[path = "../../../../../../src/security/tpm/machine_key/wire.rs"]
pub mod wire;

pub use error::KeyError;
