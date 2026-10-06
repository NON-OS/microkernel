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

//! The kernel's check of the bootloader that started it: the loader's
//! measurement, read from the firmware's TCG log and held to the live PCR 4,
//! must sit under the boot-root the release signed, with the loader's own v4
//! trailer as the proof. Without a TPM only the loader file the loader handed
//! over can be hashed, and the verdict says so.

mod answer;
mod evidence;
mod key;
mod log;
pub mod record;
mod run;
mod verdict;

pub use answer::boot_attest_record;
pub use run::check_bootloader;
pub use verdict::{verdict, Verdict};
