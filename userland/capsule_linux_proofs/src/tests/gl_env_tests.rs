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

//! OpenGL in a box is Mesa's software driver, drawn through Wayland, named
//! only where the run's store has the NØNOS-built Mesa: a driver path to
//! nothing had every Qt program say Mesa could not open its driver.

use crate::linux::env::{default, with_pwd, MESA};

const MESA_ENV: [&[u8]; 2] =
    [b"LIBGL_ALWAYS_SOFTWARE=1", b"LIBGL_DRIVERS_PATH=/usr/lib/nonos-mesa"];

fn count(env: &[Vec<u8>], want: &[u8]) -> usize {
    env.iter().filter(|v| v.as_slice() == want).count()
}

#[test]
fn a_run_whose_store_has_mesa_is_sent_to_its_software_driver() {
    assert_eq!(MESA, b"/usr/lib/nonos-mesa");
    let env = with_pwd(b"/root", true);
    for want in MESA_ENV.into_iter().chain([&b"EGL_PLATFORM=wayland"[..]]) {
        assert_eq!(count(&env, want), 1, "{want:?}");
    }
}

#[test]
fn a_run_without_mesa_is_told_nothing_of_gl_drivers() {
    for env in [default(), with_pwd(b"/root", false)] {
        assert!(!env.iter().any(|v| v.starts_with(b"LIBGL_")), "{env:?}");
        assert_eq!(count(&env, b"EGL_PLATFORM=wayland"), 1);
    }
}

#[test]
fn a_run_without_mesa_hears_nothing_of_the_driver_it_has_not_got() {
    let quiet = [&b"EGL_LOG_LEVEL=fatal"[..], b"QT_LOGGING_RULES=qt.qpa.wayland.warning=false"];
    let (without, with) = (with_pwd(b"/root", false), with_pwd(b"/root", true));
    for want in quiet {
        assert_eq!((count(&without, want), count(&with, want)), (1, 0), "{want:?}");
    }
}

#[test]
fn no_guest_is_pointed_at_a_gpu_driver() {
    for env in [with_pwd(b"/", false), with_pwd(b"/", true)] {
        let named = |key: &[u8]| env.iter().any(|v| v.starts_with(key));
        assert!(!named(b"GALLIUM_DRIVER="), "softpipe for all would pin out llvmpipe");
        assert!(!named(b"MESA_LOADER_DRIVER_OVERRIDE="));
    }
}
