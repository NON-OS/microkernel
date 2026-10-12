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

//! W1: a dead link is noticed. The access point unheard, then unanswered;
//! deauthentication taken only when it may be (in the clear without
//! management frame protection, protected with it); SA Query both ways; BIP;
//! and the IGTK required whenever protection was negotiated.

mod alive_tests;
mod bip_tests;
mod frame_tests;
mod frames;
mod hostile_tests;
mod igtk_tests;
mod leave_tests;
mod protected_tests;
mod sim;
