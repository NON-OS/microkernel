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

//! Marketplace installs: there is no installer on the host, so every ask
//! is refused and no failed install has a line to tell.

const ENOSYS: i64 = -38;

pub const MK_APP_INSTALL_DETAIL_MAX: usize = 120;

pub fn mk_app_install(_listing: &[u8], _release: &[u8]) -> i64 {
    ENOSYS
}
pub fn mk_app_install_status(_listing: &[u8]) -> i64 {
    ENOSYS
}
pub fn mk_app_install_detail(_listing: &[u8], _out: &mut [u8; MK_APP_INSTALL_DETAIL_MAX]) -> i64 {
    ENOSYS
}
pub fn mk_app_uninstall(_listing: &[u8]) -> i64 {
    ENOSYS
}
