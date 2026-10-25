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

#[path = "../../../../../src/process/foreign/seal/family.rs"]
pub mod family;
#[path = "../../../../../src/process/foreign/seal/family_calls.rs"]
mod family_calls;
#[path = "../../../../../src/process/foreign/seal/family_new.rs"]
mod family_new;
#[path = "../../../../../src/process/foreign/seal/family_rows.rs"]
mod family_rows;
#[path = "../../../../../src/process/foreign/seal/finish.rs"]
pub mod finish;
#[path = "../../../../../src/process/foreign/seal/hist.rs"]
pub mod hist;
#[path = "../../../../../src/process/foreign/seal/meas.rs"]
pub mod meas;
#[path = "../../../../../src/process/foreign/seal/live.rs"]
pub mod live;
mod nonce;
#[path = "../../../../../src/process/foreign/seal/record.rs"]
pub mod record;
#[path = "../../../../../src/process/foreign/seal/sealed.rs"]
pub mod sealed;
#[path = "../../../../../src/process/foreign/seal/table.rs"]
pub mod table;
#[path = "../../../../../src/process/foreign/seal/snapshot.rs"]
mod snapshot;
#[cfg(test)]
#[path = "../../../../../src/process/foreign/seal/sys_keep.rs"]
pub mod sys_keep;
#[cfg(test)]
#[path = "../../../../../src/process/foreign/seal/sys_read.rs"]
pub mod sys_read;
#[cfg(test)]
mod tests;
#[path = "../../../../../src/process/foreign/seal/trace.rs"]
pub mod trace;
#[path = "../../../../../src/process/foreign/seal/wipe.rs"]
mod wipe;
