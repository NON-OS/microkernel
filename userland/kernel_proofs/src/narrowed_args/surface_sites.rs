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

/// The surface call sites (vsync's display among them).
pub(super) fn surface_sites() -> Vec<(&'static str, &'static str, Vec<&'static str>)> {
    vec![
        (
            "surface_handlers/display_mode.rs",
            include_str!(
                "../../../../src/syscall/dispatch/router/surface_handlers/display_mode.rs"
            ), // A digit of a u32 being printed, not a register.
            vec!["tmp[k] = b'0' + (v % 10) as u8;"],
        ),
        (
            "surface_handlers/frames.rs",
            include_str!("../../../../src/syscall/dispatch/router/surface_handlers/frames.rs"),
            vec![],
        ),
        (
            "surface_handlers/present.rs",
            include_str!("../../../../src/syscall/dispatch/router/surface_handlers/present.rs"),
            vec![],
        ),
        (
            "surface_handlers/register.rs",
            include_str!("../../../../src/syscall/dispatch/router/surface_handlers/register.rs"),
            vec![],
        ),
        (
            "surface_handlers/release.rs",
            include_str!("../../../../src/syscall/dispatch/router/surface_handlers/release.rs"),
            vec![],
        ),
        (
            "surface_handlers/share.rs",
            include_str!("../../../../src/syscall/dispatch/router/surface_handlers/share.rs"),
            vec![],
        ),
    ]
}
