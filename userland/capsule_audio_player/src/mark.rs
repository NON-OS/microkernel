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

#[cfg(feature = "nonos-audio-player-smoketest")]
use nonos_libc::mk_debug;

/// A boot-harness marker ("[PLAYER] up", "pause-sent", "eof"). Printed in the
/// smoketest build, whose harness reads them; the image a person runs wrote
/// one to the serial console for every pause, resume and track end.
#[cfg(feature = "nonos-audio-player-smoketest")]
pub fn mark(s: &str) {
    let _ = mk_debug(s.as_ptr(), s.len());
}

#[cfg(not(feature = "nonos-audio-player-smoketest"))]
pub fn mark(_s: &str) {}

#[cfg(feature = "nonos-audio-player-smoketest")]
pub fn mark_frames(n: u64) {
    let mut buf = [0u8; 40];
    let prefix = b"[PLAYER] frames=";
    let mut i = 0;
    while i < prefix.len() {
        buf[i] = prefix[i];
        i += 1;
    }
    let mut digits = [0u8; 20];
    let mut d = 0;
    let mut v = n;
    loop {
        digits[d] = b'0' + (v % 10) as u8;
        v /= 10;
        d += 1;
        if v == 0 {
            break;
        }
    }
    while d > 0 {
        d -= 1;
        buf[i] = digits[d];
        i += 1;
    }
    buf[i] = b'\n';
    i += 1;
    let _ = mk_debug(buf.as_ptr(), i);
}

#[cfg(feature = "nonos-audio-player-smoketest")]
pub fn mark_mp3_frames(n: u64) {
    let mut buf = [0u8; 44];
    let prefix = b"[PLAYER] mp3-frames=";
    let mut i = 0;
    while i < prefix.len() {
        buf[i] = prefix[i];
        i += 1;
    }
    let mut digits = [0u8; 20];
    let mut d = 0;
    let mut v = n;
    loop {
        digits[d] = b'0' + (v % 10) as u8;
        v /= 10;
        d += 1;
        if v == 0 {
            break;
        }
    }
    while d > 0 {
        d -= 1;
        buf[i] = digits[d];
        i += 1;
    }
    buf[i] = b'\n';
    i += 1;
    let _ = mk_debug(buf.as_ptr(), i);
}
