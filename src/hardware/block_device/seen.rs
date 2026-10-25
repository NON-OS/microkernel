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

//! What the block layer saw on a disk while choosing the NONOS disk, said
//! on the log when it changes. A stick the layer passed over left nothing:
//! a boot from it showed only that the store was not there, and no line
//! said which disk was asked or why it was not taken.

use super::gpt::Shown;
use super::BlockDeviceError;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Seen {
    /// The driver gave no size: no driver, no device, or still looking.
    NoSize(BlockDeviceError),
    /// The disk ends before the store would start.
    TooSmall(u64),
    /// Its blocks are not 512 bytes.
    NotFiveTwelve(u64),
    /// LBA 1 or the entry array could not be read.
    NoTable(u64, BlockDeviceError),
    /// Its table names another system's partition over the fixed sectors.
    Crossed(u64),
    /// What its store sector held, and its disk plan sector when asked;
    /// `gpt` when its table placed them, `boot` when it holds the loader's
    /// partition.
    At {
        /// The disk's sectors, and the store's first LBA.
        at: (u64, u64),
        got: Result<[u8; 8], BlockDeviceError>,
        plan: Option<Result<[u8; 8], BlockDeviceError>>,
        gpt: bool,
        boot: bool,
        /// What its GPT showed.
        shown: Shown,
    },
}
