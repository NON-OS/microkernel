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

//! The line for a namespace that gets no I/O queue.

use crate::admin::NamespaceIdentity;
use crate::log::{emit, Line};
use crate::nvm::Refusal;

/// Why the namespace gets no I/O queue, with the identify fields that
/// decided it.
pub fn refused(ns: &NamespaceIdentity, why: Refusal) {
    let mut line = Line::new();
    line.text(b"nsid ").dec(ns.nsid as u64).text(b" gets no I/O queue: ");
    match why {
        Refusal::NoNamespace => {
            line.text(b"no namespace");
        }
        Refusal::Empty => {
            line.text(b"NSZE is 0");
        }
        Refusal::BlockSize(size) => {
            line.text(b"LBA size ").dec(size as u64).text(b" bytes, only 512 and 4096 served");
        }
        Refusal::Format { index, upper, nlbaf } => {
            line.text(b"FLBAS index ")
                .dec(index as u64)
                .text(b" upper bits ")
                .dec(upper as u64)
                .text(b" past NLBAF ")
                .dec(nlbaf as u64);
        }
        Refusal::Metadata(ms) => {
            line.text(b"format carries ").dec(ms as u64).text(b" metadata bytes per LBA (MS)");
        }
        Refusal::TransferBelowBlock { max_bytes, lba_size } => {
            line.text(b"MDTS allows ")
                .dec(max_bytes)
                .text(b" bytes, below one ")
                .dec(lba_size as u64)
                .text(b"-byte LBA");
        }
    }
    line.text(b" (LBA ")
        .dec(ns.lba_size as u64)
        .text(b" MS ")
        .dec(ns.metadata_size as u64)
        .text(b" FLBAS ")
        .dec(ns.format_index as u64)
        .text(b" NLBAF ")
        .dec(ns.formatted_lba_count.wrapping_sub(1) as u64)
        .text(b")");
    emit(&mut line);
}
