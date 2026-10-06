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

//! The capsule policy root: the root the spawn gate folds every capsule's
//! path against. It is compiled into the kernel image, so it is proven by
//! exactly what proved the kernel, and no more.

use alloc::format;
use alloc::string::String;

use super::mark::Mark;
use super::row::Row;
use crate::install::format::hex_prefix;
use crate::install::source::Boot;

const NAME: &str = "CAPSULE POLICY ROOT";

pub(super) fn capsule_root(b: &Boot, kernel: Mark) -> Row {
    let Some(p) = b.policy else {
        return Row::new(
            NAME,
            Mark::Unknown,
            "the kernel did not report its policy",
            String::new(),
        );
    };
    let Some(t) = p.capsule else {
        return Row::new(NAME, Mark::NotChecked, "the spawn gate folds no paths", String::new());
    };
    let root = format!("root {}", hex_prefix(&t.root));
    let (mark, how) = match kernel {
        Mark::Verified => (Mark::Verified, "in the kernel proven above"),
        Mark::Unknown => (Mark::Unknown, "in a kernel whose proof is unknown"),
        _ => (Mark::NotVerified, "in a kernel with no STARK pass"),
    };
    Row::new(NAME, mark, &format!("{how}, epoch {}", t.epoch), root)
}
