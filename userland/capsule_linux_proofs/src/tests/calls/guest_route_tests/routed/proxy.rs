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

//! A proxy as net.socks5 is to the frames of one stream (capsule_socks5
//! server/kept.rs): a frame it takes is carried to the far end, and its
//! answer may be lost; asked again under the same number with the very
//! bytes it carried, it gives back the answer it kept and carries nothing
//! twice, while other bytes under that number are carried as new. A call
//! may also be turned away before it reaches the proxy at all.

pub struct Proxy {
    /// What reached the far end, in the order it was carried.
    pub far: Vec<u8>,
    /// What the frame under the current number carried, its answer kept.
    kept: Option<Vec<u8>>,
    s: u64,
    answers: bool,
}

impl Proxy {
    /// A proxy that answers some calls and loses others, or none at all.
    pub fn new(seed: u64, answers: bool) -> Proxy {
        let s = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
        Proxy { far: Vec::new(), kept: None, s, answers }
    }

    pub fn next(&mut self) -> u64 {
        self.s ^= self.s << 13;
        self.s ^= self.s >> 7;
        self.s ^= self.s << 17;
        self.s
    }

    /// One call carrying `frame`: whether its answer came back.
    pub fn call(&mut self, frame: &[u8]) -> bool {
        let roll = self.next() % 4;
        if !self.answers || roll == 0 {
            return false;
        }
        if self.kept.as_deref() != Some(frame) {
            self.far.extend_from_slice(frame);
            self.kept = Some(frame.to_vec());
        }
        if roll == 1 {
            return false;
        }
        /* Answered: the caller moves on to its next number. */
        self.kept = None;
        true
    }
}
