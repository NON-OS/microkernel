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

//! Option levels and names, from include/uapi/asm-generic/socket.h,
//! include/uapi/linux/in.h and include/uapi/linux/tcp.h.

pub const SOL_SOCKET: u64 = 1;
pub const IPPROTO_IP: u64 = 0;
pub const IPPROTO_TCP: u64 = 6;
pub const IPPROTO_IPV6: u64 = 41;

pub const SO_REUSEADDR: u64 = 2;
pub const SO_TYPE: u64 = 3;
pub const SO_ERROR: u64 = 4;
pub const SO_BROADCAST: u64 = 6;
pub const SO_SNDBUF: u64 = 7;
pub const SO_RCVBUF: u64 = 8;
pub const SO_KEEPALIVE: u64 = 9;
pub const SO_LINGER: u64 = 13;
pub const SO_REUSEPORT: u64 = 15;
pub const SO_RCVTIMEO: u64 = 20;
pub const SO_SNDTIMEO: u64 = 21;
pub const SO_ACCEPTCONN: u64 = 30;
pub const SO_PROTOCOL: u64 = 38;
pub const SO_DOMAIN: u64 = 39;

pub const IP_TOS: u64 = 1;
pub const IP_TTL: u64 = 2;
pub const SO_PRIORITY: u64 = 12;

pub const TCP_NODELAY: u64 = 1;
pub const TCP_KEEPIDLE: u64 = 4;
pub const TCP_KEEPINTVL: u64 = 5;
pub const TCP_KEEPCNT: u64 = 6;
pub const TCP_QUICKACK: u64 = 12;
pub const TCP_USER_TIMEOUT: u64 = 18;
pub const TCP_FASTOPEN: u64 = 23;

/// net.core.rmem_max and wmem_max as Linux ships them: what SO_RCVBUF and
/// SO_SNDBUF are held to before they are doubled.
pub const BUF_MAX: u32 = 212_992;
/// The least Linux keeps: SOCK_MIN_RCVBUF and SOCK_MIN_SNDBUF.
pub const RCVBUF_MIN: u32 = 2304;
pub const SNDBUF_MIN: u32 = 4608;
/// MAX_TCP_KEEPIDLE, MAX_TCP_KEEPINTVL and MAX_TCP_KEEPCNT.
pub const KEEP_MAX: u32 = 32767;
pub const KEEPCNT_MAX: u32 = 127;
