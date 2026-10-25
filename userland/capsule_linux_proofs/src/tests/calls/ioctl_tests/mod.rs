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

//! ioctl: every descriptor answers the file requests, only a console on a
//! terminal answers the terminal's, and every other request on every other
//! descriptor is ENOTTY; TCFLSH and TCXONC refuse a bad selector EINVAL;
//! TIOCSWINSZ holds until the terminal itself changes size; getrandom
//! refuses the flags Linux refuses.

mod args;
mod requests;
mod terminal;
