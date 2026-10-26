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

//! The /data volume as a trip lock (L81b part C): a writer swept asleep in
//! any of its disk requests leaves the mount and the disk in a state the
//! next writer and a reboot go on from. Each operation's order is read from
//! the kernel source, so a change there changes the model.

#[cfg(test)]
mod disk;
#[cfg(test)]
mod exec;
#[cfg(test)]
mod exec_io;
#[cfg(test)]
mod exec_room;
#[cfg(test)]
mod exec_step;
#[cfg(test)]
mod glue_hold_tests;
#[cfg(test)]
mod glue_open_tests;
#[cfg(test)]
mod glue_order_tests;
#[cfg(test)]
mod glue_vfs_tests;
#[cfg(test)]
mod glue_volume_tests;
#[cfg(test)]
mod judge;
#[cfg(test)]
mod kinds;
#[cfg(test)]
mod ops;
#[cfg(test)]
mod ops_more;
#[cfg(test)]
mod order;
#[cfg(test)]
mod run;
#[cfg(test)]
mod search;
#[cfg(test)]
mod search_tests;
