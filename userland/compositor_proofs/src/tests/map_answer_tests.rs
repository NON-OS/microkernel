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

//! What every userland caller makes of mk_mmap's answer. The kernel answers
//! a refused map with a negative errno (sys_mmap: ERRNO_PERM, ERRNO_NOMEM,
//! ERRNO_INVAL), never null, and libc's mk_mmap hands that back as the
//! pointer; a caller that checked for null alone took the errno for an
//! address and faulted on its first write. Each caller here is held to a
//! check that refuses a negative answer.

const CALLERS: &[(&str, &str)] = &[
    (
        "app_skeleton/src/setup/backing.rs",
        include_str!("../../../app_skeleton/src/setup/backing.rs"),
    ),
    (
        "capsule_boot_splash/src/surface.rs",
        include_str!("../../../capsule_boot_splash/src/surface.rs"),
    ),
    (
        "capsule_desktop_shell/src/setup/prime/overlay.rs",
        include_str!("../../../capsule_desktop_shell/src/setup/prime/overlay.rs"),
    ),
    (
        "capsule_image_codec/src/server/handlers/surface.rs",
        include_str!("../../../capsule_image_codec/src/server/handlers/surface.rs"),
    ),
    (
        "capsule_image_viewer/src/viewer/decode_surface.rs",
        include_str!("../../../capsule_image_viewer/src/viewer/decode_surface.rs"),
    ),
    (
        "capsule_input_probe/src/setup/run.rs",
        include_str!("../../../capsule_input_probe/src/setup/run.rs"),
    ),
    (
        "capsule_install/src/install/full/surface.rs",
        include_str!("../../../capsule_install/src/install/full/surface.rs"),
    ),
    (
        "capsule_login/src/setup/backing.rs",
        include_str!("../../../capsule_login/src/setup/backing.rs"),
    ),
    (
        "capsule_setup_wizard/src/setup/run.rs",
        include_str!("../../../capsule_setup_wizard/src/setup/run.rs"),
    ),
    (
        "capsule_wallpaper/src/setup/prime/backing.rs",
        include_str!("../../../capsule_wallpaper/src/setup/prime/backing.rs"),
    ),
    ("compositor/src/setup/canvas.rs", include_str!("../../../compositor/src/setup/canvas.rs")),
    (
        "compositor/src/setup/gop_backing.rs",
        include_str!("../../../compositor/src/setup/gop_backing.rs"),
    ),
    ("compositor/src/setup/layout.rs", include_str!("../../../compositor/src/setup/layout.rs")),
    ("libc/src/heap/span.rs", include_str!("../../../libc/src/heap/span.rs")),
    ("libc/src/thread/seat.rs", include_str!("../../../libc/src/thread/seat.rs")),
    ("nonos_gfx_pool/src/anon.rs", include_str!("../../../nonos_gfx_pool/src/anon.rs")),
];

/// The text after each mk_mmap call in `src`: up to the next call, and at
/// most 600 characters, which holds the check that follows a call.
fn after_calls(src: &str) -> impl Iterator<Item = &str> {
    src.split("mk_mmap(")
        .skip(1)
        .map(|rest| rest.char_indices().nth(600).map_or(rest, |(i, _)| &rest[..i]))
}

fn refuses_negative(after: &str) -> bool {
    ["as isize) <= 0", "as i64) < 0", "as i64) >= 0", "mapped("].iter().any(|c| after.contains(c))
}

#[test]
fn every_caller_refuses_a_negative_answer() {
    for (name, src) in CALLERS {
        let mut calls = 0;
        for after in after_calls(src) {
            calls += 1;
            assert!(refuses_negative(after), "{name}: a map's answer is checked for null alone");
        }
        assert!(calls > 0, "{name} no longer calls mk_mmap; take it off the list");
    }
}

#[test]
fn a_null_check_alone_is_caught() {
    let only_null = "mk_mmap(core::ptr::null_mut(), n, 3, 0x22, -1, 0);\n    if va.is_null() {";
    assert!(after_calls(only_null).any(|a| !refuses_negative(a)));
    let negative =
        "mk_mmap(core::ptr::null_mut(), n, 3, 0x22, -1, 0);\n    if (va as isize) <= 0 {";
    assert!(after_calls(negative).all(refuses_negative));
}
