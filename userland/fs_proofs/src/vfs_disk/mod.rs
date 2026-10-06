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

// The whole vfs block layer (crate::vfs_blk, from capsule source) over the
// disk in memory the libc shim keeps. The loader, the appender, the remover
// and the status word run as they do in vfs_pool; only the two kernel calls
// under them are the shim's. So a proof can put any bytes at the store's
// sectors, take the disk away, or cut the power after any sector, and read
// what the next boot's load would find.

pub use crate::vfs_blk as blk;

#[cfg(test)]
mod fixture;
#[cfg(test)]
mod run;
#[cfg(test)]
mod tests_append;
#[cfg(test)]
mod tests_corrupt;
#[cfg(test)]
mod tests_damage;
#[cfg(test)]
mod tests_errno;
#[cfg(test)]
mod tests_load;
#[cfg(test)]
mod tests_names;
#[cfg(test)]
mod tests_patience;
#[cfg(test)]
mod tests_random;
#[cfg(test)]
mod tests_remove;
#[cfg(test)]
mod tests_streamed;
