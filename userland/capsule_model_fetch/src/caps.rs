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
 * The capabilities this program asks for, written where the build reads
 * them (scripts/check_declared_caps.py) and refuses to sign a manifest that
 * grants others. No FileSystem: nothing on the data volume is readable here.
 */

use nonos_cap::{CAP_CORE_EXEC, CAP_CRYPTO, CAP_IPC, CAP_MEMORY, CAP_NETWORK, CAP_STREAM_IMPORT};

#[no_mangle]
#[used]
#[link_section = ".nonos.caps"]
pub static NONOS_DECLARED_CAPS: u64 =
    CAP_CORE_EXEC | CAP_NETWORK | CAP_IPC | CAP_MEMORY | CAP_CRYPTO | CAP_STREAM_IMPORT;
