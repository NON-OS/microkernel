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

//! The line the server's start leaves on the log, and the smoketest's tones.

use crate::mark::mark;
use crate::mixer::Mixer;
use crate::sink::Sink;

// The mixer plays the smoketest's tones; a shipped build only marks.
#[cfg_attr(not(feature = "nonos-audio-smoketest"), allow(unused_variables))]
pub(super) fn announce(sink: Option<&Sink>, mixer: &mut Mixer) {
    match sink {
        #[cfg(feature = "nonos-audio-smoketest")]
        Some(s) => crate::selftest::boot(mixer, s),
        #[cfg(not(feature = "nonos-audio-smoketest"))]
        Some(s) => mark(s.output_line()),
        /* A fixed sink driver is registered before this capsule is spawned
         * and dropped when it finds no controller, and driver.usb_audio0 is
         * registered only where an xHCI controller is, so none after the
         * lookup's two seconds means this machine has no output. A
         * client asking why is told there is no sound hardware, a stream
         * open is refused with E_NODEV, everything else with E_INVAL, and
         * the desktop's chime and alerts are skipped without a sound. A
         * machine the driver found but cannot play on keeps the driver,
         * which says why (OP_OUTPUT_STATUS). */
        None => mark("[AUDIO] no output device, playback refused\n"),
    }
}
