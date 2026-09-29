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

/*
 * sysinfo, getrusage and times: what the family has used, as the kernel
 * measured it for the family's own threads (file/system/cpu/), and the system
 * as NONOS declares it (file/system/declared/). Two figures are not what Linux
 * means by them: the kernel keeps no peak resident size, so ru_maxrss is
 * the resident size at the call, and it does not tell a voluntary switch
 * from another, so ru_nvcsw counts every switch and ru_nivcsw none. The
 * store's reads and writes are not counted per process, so ru_inblock and
 * ru_oublock are zero. The fields Linux itself leaves at zero are zero.
 */

mod rusage;
mod sysinfo;
mod times;

pub use rusage::getrusage;
pub use sysinfo::sysinfo;
pub use times::{mine, times};
