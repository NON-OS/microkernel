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

//! GTK 3 is framed by NØNOS only when KDE's server-decoration manager says
//! Server: it binds no xdg-decoration and reads GTK_CSD nowhere on Wayland,
//! so it drew its own title bar inside the NØNOS frame.

use crate::kde_mode::{answer, CLIENT, NONE, SERVER};

#[test]
fn the_kde_modes_are_the_protocols_numbers() {
    assert_eq!((NONE, CLIENT, SERVER), (0, 1, 2));
}

#[test]
fn a_client_asking_server_or_none_gets_the_nonos_frame() {
    assert_eq!(answer(SERVER), (false, SERVER));
    assert_eq!(answer(NONE), (false, SERVER));
    assert_eq!(answer(7), (false, SERVER));
}

#[test]
fn a_client_asking_client_draws_its_own() {
    assert_eq!(answer(CLIENT), (true, CLIENT));
}
