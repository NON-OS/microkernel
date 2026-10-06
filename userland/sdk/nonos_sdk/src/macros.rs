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

/// Declare an app's entry point and everything it is permitted to do.
///
/// ```ignore
/// sdk_main!(app);                            // can run, and nothing else
/// sdk_main!(app, caps: [WINDOW]);            // and can draw
/// sdk_main!(app, caps: [WINDOW, NETWORK]);   // and can reach the network
/// ```
///
/// The list is not advice to the runtime. It is written into a `.nonos.caps`
/// section of the binary. After signing, the seal's verify phase and
/// `nonos-mk-verify-image` run `scripts/check_declared_caps.py`, which fails
/// when the section and the manifest's required capabilities differ. Signing
/// itself does not run that check, so a mismatch is caught after the manifest
/// is signed, before the image is called ready.
///
/// A section rather than a symbol because release builds strip symbols, and a
/// declaration that vanishes under `--strip-all` is worse than none: it would
/// verify in development and silently stop being checked in the build that
/// ships.
///
/// Omitting the list is not a shortcut to a working app. It declares that the
/// app needs nothing, and it will be held to that.
#[macro_export]
macro_rules! sdk_main {
    ($entry:path) => {
        $crate::sdk_main!($entry, caps: []);
    };
    ($entry:path, caps: [$($group:ident),* $(,)?]) => {
        #[no_mangle]
        #[used]
        #[link_section = ".nonos.caps"]
        pub static NONOS_DECLARED_CAPS: u64 =
            $crate::caps::BASE $(| $crate::caps::$group)*;

        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            $crate::__run(NONOS_DECLARED_CAPS, $entry)
        }
    };
}
