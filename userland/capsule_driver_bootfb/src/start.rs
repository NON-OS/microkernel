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

//! How bootfb's start ends when it does not serve: the reason it gives
//! the shared bring-up, and the one line and exit when a GPU driver already
//! has the scanout.

use nonos_libc::mk_exit;

use crate::firmware;

/// A GPU driver has the scanout and the claim is refused for good (L79):
/// one line, no output served, and gone at once rather than retried.
pub fn refused() -> ! {
    let line = b"driver.bootfb: the boot framebuffer is a GPU driver's now; not started\n";
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    mk_exit(nonos_libc::EXIT_ABSENT)
}

pub fn why(w: firmware::Why) -> &'static str {
    use firmware::Why::*;
    match w {
        Record => "not the boot framebuffer",
        Announce => "driver.bootfb0 not registered",
        Claim => "claim refused",
        Refused => "claim refused for good",
        Geometry => "mode not given",
        Format => "three-byte pixels",
        Map => "bar 0 not mapped",
        Shape => "mode outside the range",
    }
}
