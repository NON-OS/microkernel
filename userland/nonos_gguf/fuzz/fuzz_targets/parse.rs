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

//! Any bytes as a GGUF file: the reader must refuse or accept without a
//! panic, and an accepted file must keep the summary's promises.

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let len = data.len() as u64;
    if let Ok(s) = nonos_gguf::parse(&mut &data[..], len, &nonos_gguf::DEFAULT) {
        assert!(s.data_bytes <= len);
        assert_eq!(s.data_start % u64::from(s.meta.alignment), 0);
        assert!(s.tensors <= nonos_gguf::DEFAULT.max_tensors);
    }
});
