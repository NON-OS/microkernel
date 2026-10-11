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

mod absorb;
mod family;
mod family_calls;
mod family_new;
mod family_rows;
mod finish;
pub(crate) mod hist;
mod identity;
pub(crate) mod meas;
mod nonce;
mod open;
mod park;
pub(crate) mod record;
mod sealed;
mod sys_note;
mod sys_read;
mod sys_seal;
mod table;
pub(crate) mod trace;
mod vendor;

pub(in crate::process::foreign) use absorb::answered;
pub(crate) use absorb::{abandon, measured, refused};
pub(crate) use finish::finish;
pub(crate) use open::{enrolled, reserve};
pub(in crate::process::foreign) use park::{park, refused_early};
pub(crate) use sys_note::sys_seal_note;
pub(crate) use sys_read::sys_seal_read;
pub(crate) use sys_seal::sys_foreign_seal;
