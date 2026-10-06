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

//! The check of a described namespace against what the driver serves.

use super::max_transfer_bytes;
use super::params::NamespaceGeometry;
use super::refusal::Refusal;
use crate::admin::{ControllerIdentity, NamespaceIdentity};

impl NamespaceGeometry {
    /// The geometry to build the I/O queue around, or why the namespace the
    /// controller described cannot be served. Every field comes from the
    /// device's identify pages. An absent namespace (the controller reported
    /// none, so nsid is 0) and an empty one are refused, and so is any block
    /// size but the two real namespaces are formatted with: many enterprise
    /// and consumer SSDs ship 4096-byte LBAs, the rest 512.
    pub fn check(identity: &ControllerIdentity, ns: &NamespaceIdentity) -> Result<Self, Refusal> {
        if ns.nsid == 0 {
            return Err(Refusal::NoNamespace);
        }
        if ns.size_lba == 0 {
            return Err(Refusal::Empty);
        }
        // FLBAS must name a format the namespace has (NLBAF is zero-based)
        // and one of the 16 slots the parser read; an index extended by
        // bits 6:5 names a slot the block size was never taken from.
        if ns.format_index_upper != 0 || ns.format_index >= ns.formatted_lba_count {
            return Err(Refusal::Format {
                index: ns.format_index,
                upper: ns.format_index_upper,
                nlbaf: ns.formatted_lba_count.wrapping_sub(1),
            });
        }
        // Metadata would ride inside each block (extended LBAs), overrunning
        // the data buffer and the PRPs built for it, or go to a separate
        // buffer the driver never provides (MPTR is 0). Only formats without
        // it are served.
        if ns.metadata_size != 0 {
            return Err(Refusal::Metadata(ns.metadata_size));
        }
        if !matches!(ns.lba_size, 512 | 4096) {
            return Err(Refusal::BlockSize(ns.lba_size));
        }
        let max_bytes = max_transfer_bytes(identity.mdts);
        let too_small = Refusal::TransferBelowBlock { max_bytes, lba_size: ns.lba_size };
        let max_sectors = u32::try_from(max_bytes / ns.lba_size as u64).map_err(|_| too_small)?;
        if max_sectors == 0 {
            return Err(too_small);
        }
        Ok(Self {
            nsid: ns.nsid,
            capacity_sectors: ns.size_lba,
            lba_size: ns.lba_size,
            max_sectors,
        })
    }
}
