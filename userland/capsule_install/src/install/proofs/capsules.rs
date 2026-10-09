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

//! The capsules running now whose STARK proof the kernel checked at spawn,
//! from the attestation registry, vendor and locally built counted apart.
//! A capsule a publisher only signed ran without a proof and is named so.

use alloc::format;
use alloc::string::String;

use super::mark::Mark;
use super::row::Row;
use crate::install::source::Boot;

const NAME: &str = "CAPSULES";

pub(super) fn capsules(b: &Boot) -> Row {
    let Some(c) = b.capsules else {
        return Row::new(NAME, Mark::Unknown, "the kernel did not list them", String::new());
    };
    let proven = c.vendor + c.local;
    let mut detail = format!("{} vendor, {} locally built", c.vendor, c.local);
    if c.publisher > 0 {
        detail.push_str(&format!(", {} signed only", c.publisher));
    }
    if proven == 0 {
        return Row::new(NAME, Mark::NotChecked, "no running capsule carries a proof", detail);
    }
    let says = if b.path_only {
        format!("{proven} running, each path-checked at spawn; development, no STARK")
    } else {
        format!("{proven} running, each STARK-checked at spawn")
    };
    Row::new(NAME, Mark::Verified, &says, detail)
}
