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

/* Noise and plain text are not Brotli streams: each must come back as
an error or a bounded output, never a panic. */

#[path = "common/inputs.rs"]
mod inputs;

use inputs::input;
use nonos_brotli::decompress;

#[test]
fn noise_and_text_are_refused_safely() {
    let mut refused = 0;
    for n in 0..20_000u64 {
        let s = input((n % 2) as u8, (n % 97) as usize, n);
        match decompress(&s, 1 << 16) {
            Ok(v) => assert!(v.len() <= 1 << 16),
            Err(_) => refused += 1,
        }
    }
    assert!(refused > 19_000, "only {refused} refused");
}
