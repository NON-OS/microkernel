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

/* The names a guest is told: the kernel, the host and the machine. */

/* The kernel a guest is told it runs on: uname's release. */
pub const RELEASE: &[u8] = b"6.1.0";

/* uname's nodename and the hostname files: the family's name for itself. */
pub const HOSTNAME: &[u8] = b"nonos";

/* uname's version field. */
pub const VERSION: &[u8] = b"NONOS Linux personality";

/* uname's sysname and /proc/sys/kernel/ostype. */
pub const OSTYPE: &[u8] = b"Linux";

/* uname's machine. */
pub const MACHINE: &[u8] = b"x86_64";

/* uname's domainname. */
pub const DOMAIN: &[u8] = b"nonos";
