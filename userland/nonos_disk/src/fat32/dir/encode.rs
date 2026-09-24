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

//! A whole directory: `.` and `..` for a subdirectory, then one slot per
//! child. The caller pads to the run's length, and that zero tail is the
//! terminator a reader stops at.

use alloc::vec::Vec;

use super::short_name::{encode as short_name, NameError};
use super::slot::{slot, ATTR_ARCHIVE, ATTR_DIR};
use crate::fat32::tree::{Content, Run};

pub fn encode(index: usize, runs: &[Run<'_>]) -> Result<Vec<u8>, NameError> {
    let run = &runs[index];
    let entries = match &run.content {
        Content::Dir(entries) => entries,
        Content::File(_) => return Ok(Vec::new()),
    };
    let mut out = Vec::with_capacity((entries.len() + 3) * 32);
    if let Some(parent) = run.parent {
        /*
         * The root directory is written as cluster 0 in a `..` slot, not
         * 2: that is what the specification says and what every driver
         * expects, and a `..` that says 2 sends some of them looking for
         * a parent of the root.
         */
        let up = if parent == 0 { 0 } else { runs[parent].first_cluster };
        out.extend_from_slice(&slot(*b".          ", ATTR_DIR, 0, run.first_cluster, 0));
        out.extend_from_slice(&slot(*b"..         ", ATTR_DIR, 0, up, 0));
    }
    for e in entries {
        let name = short_name(e.name)?;
        let attr = if e.is_dir { ATTR_DIR } else { ATTR_ARCHIVE };
        let cluster = runs[e.run].first_cluster;
        out.extend_from_slice(&slot(name.bytes, attr, name.nt_flags, cluster, e.size));
    }
    Ok(out)
}
