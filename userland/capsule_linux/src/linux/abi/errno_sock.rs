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

//! Linux errno values the socket calls answer with, from
//! include/uapi/asm-generic/errno-base.h and errno.h.

pub const EDOM: i64 = 33;
pub const EDESTADDRREQ: i64 = 89;
pub const EMSGSIZE: i64 = 90;
pub const EPROTOTYPE: i64 = 91;
pub const ENOPROTOOPT: i64 = 92;
pub const EPROTONOSUPPORT: i64 = 93;
pub const ESOCKTNOSUPPORT: i64 = 94;
pub const EOPNOTSUPP: i64 = 95;
pub const EADDRINUSE: i64 = 98;
pub const EADDRNOTAVAIL: i64 = 99;
pub const ENETUNREACH: i64 = 101;
pub const EISCONN: i64 = 106;
pub const ECONNABORTED: i64 = 103;
pub const EALREADY: i64 = 114;
