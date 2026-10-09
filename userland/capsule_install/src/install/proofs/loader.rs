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

//! The bootloader's row: the kernel's own check of the loader that started
//! it, from `MkBootAttest`, in the words the kernel's boot log uses.

use alloc::format;
use alloc::string::String;

use nonos_libc::boot_attest::{AdmittedLoader, BootAttest};

use super::mark::Mark;
use super::row::Row;
use crate::install::format::hex_prefix;
use crate::install::source::Boot;

const NAME: &str = "BOOTLOADER";

pub(super) fn bootloader(b: &Boot) -> Row {
    let Some(v) = b.loader else {
        return Row::new(NAME, Mark::Unknown, "the kernel did not answer", String::new());
    };
    match v {
        BootAttest::Measured(a) => {
            let says = format!("measured and enrolled, epoch {}", a.epoch);
            Row::new(NAME, Mark::Verified, &says, proven(&a))
        }
        BootAttest::SelfReported(a) => {
            let says = format!("self-reported, not measured: enrolled, epoch {}", a.epoch);
            Row::new(NAME, Mark::SelfReported, &says, proven(&a))
        }
        BootAttest::Refused(code) => {
            let says = format!("refused, code {code}: {}", stage(code));
            Row::new(NAME, Mark::Failed, &says, String::new())
        }
        BootAttest::NoEvidence => {
            Row::new(NAME, Mark::NotChecked, "no boot evidence", String::new())
        }
        BootAttest::NotYet => Row::new(NAME, Mark::NotChecked, "not yet checked", String::new()),
    }
}

fn proven(a: &AdmittedLoader) -> String {
    format!("loader {}  root {}", hex_prefix(&a.measurement), hex_prefix(&a.root))
}

/* The kernel's codes are grouped by what refused: see BootError::code. */
fn stage(code: u32) -> &'static str {
    match code / 100 {
        0 => "the chain",
        1 => "the TCG log",
        2 => "the boot-root record",
        3 => "the loader image",
        _ => "the STARK proof",
    }
}
