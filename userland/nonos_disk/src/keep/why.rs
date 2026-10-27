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

//! Why an installed NØNOS cannot be updated in place, in the words the
//! confirm screen shows where it would say "NONOS installed": each starts
//! with what the disk holds, then says why its data cannot be kept.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotKept {
    /// A sector the checks need did not read.
    Unread,
    /// The partition table is not the one this writer makes: its header, a
    /// checksum, or its four entries and nothing else.
    Table,
    /// The store, the plan or the data partition is not at its fixed place,
    /// or the boot partition is not one GiB or more at the end.
    Moved,
    /// The disk plan names no data volume, or a range outside NONOS-DATA.
    Plan,
    /// No key header: a volume never keyed, or keyed before there were any.
    NoKeyHeader,
    /// Way 1, a TPM key bound to the kernel that made the volume.
    MadeBefore,
    /// A way this release does not know.
    UnknownWay(u8),
    /// The new release's boot files do not fit the boot partition there.
    EspTooSmall,
}

impl NotKept {
    pub fn text(self) -> &'static str {
        match self {
            NotKept::Unread => "NØNOS, but its table and plan did not read",
            NotKept::Table => "NØNOS, with a partition table it did not write",
            NotKept::Moved => "NØNOS, laid out another way: its data cannot be kept",
            NotKept::Plan => "NØNOS, but its plan names no volume in NONOS-DATA",
            NotKept::NoKeyHeader => "NØNOS with no key header: its data cannot be kept",
            NotKept::MadeBefore => "NØNOS made before 0.9.3: its data cannot be kept",
            NotKept::UnknownWay(_) => "NØNOS, with a key this release does not know",
            NotKept::EspTooSmall => "NØNOS, but the new boot files do not fit there",
        }
    }
}
