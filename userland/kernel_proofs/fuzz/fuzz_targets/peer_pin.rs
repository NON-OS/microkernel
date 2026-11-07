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

#![no_main]

use libfuzzer_sys::fuzz_target;

#[path = "../../../../src/kernel_core/surface_registry/pin/pin_rule.rs"]
mod pin_rule;
use pin_rule::{
    in_grant, may_read, pin_frames, pin_pages, room, GuestPage, PinRefusal, PinShape, PAGE,
    PINS_PER_SUPERVISOR, READER,
};

/// What only the registry or the readers' check answers, never the rule
/// over a shape or a walk.
const REGISTRY_ONLY: [PinRefusal; 4] =
    [PinRefusal::DeviceWindow, PinRefusal::AlreadyHeld, PinRefusal::Full, PinRefusal::Reader];

/// Endpoint names a process may serve, the compositor's and a display
/// driver's among them.
const NAMES: [&str; 5] = [READER, "driver.virtio_gpu0", "wm", "linux", "compositorx"];
const DISPLAYS: &[&str] = &["driver.virtio_gpu0"];

fn word(b: &[u8], i: usize) -> u64 {
    let mut w = [0u8; 8];
    for (k, x) in w.iter_mut().enumerate() {
        *x = b.get(i * 8 + k).copied().unwrap_or(0);
    }
    u64::from_le_bytes(w)
}

fn allowed(s: PinShape, user_end: u64, max_pages: usize) -> Option<usize> {
    let (len, end) = (u128::from(s.byte_len), u128::from(s.addr) + u128::from(s.byte_len));
    let row = u128::from(s.width) * 4;
    let ok = s.addr % PAGE == 0
        && s.byte_len != 0
        && s.width != 0
        && s.height != 0
        && row <= u128::from(u32::MAX)
        && u128::from(s.stride) >= row
        && u128::from(s.stride) * u128::from(s.height) <= len
        && end <= u128::from(user_end);
    let pages = len.div_ceil(u128::from(PAGE));
    (ok && pages <= max_pages as u128).then_some(pages as usize)
}

fuzz_target!(|data: &[u8]| {
    // Small values reach the edges more often than any u64 does.
    let shape = |x: u64, sel: u8| match sel % 4 {
        0 => x,
        1 => x % 64 * PAGE,
        2 => x % (1 << 24),
        _ => u64::MAX - x % 64,
    };
    let sel = data.get(40).copied().unwrap_or(0);
    let user_end = 0x0000_8000_0000_0000u64 - (word(data, 4) % 4) * PAGE;
    let s = PinShape {
        addr: shape(word(data, 0), sel),
        byte_len: shape(word(data, 1), sel >> 2),
        width: word(data, 2) as u32 % (1 << (sel >> 4 & 3) * 8 + 4),
        height: (word(data, 2) >> 32) as u32,
        stride: word(data, 3) as u32,
    };
    let max_pages = 1 + (word(data, 3) >> 32) as usize % 4096;
    let got = pin_pages(s, user_end, max_pages);
    assert_eq!(got.ok(), allowed(s, user_end, max_pages), "{s:?} {user_end:#x} {max_pages}");
    if let Ok(pages) = got {
        assert!(pages as u64 * PAGE >= s.byte_len && (pages as u64 - 1) * PAGE < s.byte_len);
    } else {
        assert_eq!(got, Err(PinRefusal::Shape));
    }

    // The pages a walk found: each byte past the words above is one page.
    let walked: Vec<Option<GuestPage>> = data
        .iter()
        .skip(41)
        .take(64)
        .enumerate()
        .map(|(i, &b)| {
            (b & 1 == 0).then_some(GuestPage {
                frame: (u64::from(b >> 3) + i as u64) * PAGE,
                user: b & 2 == 0,
                exec: b & 4 != 0,
            })
        })
        .collect();
    let first_bad = walked.iter().find_map(|p| match p {
        None => Some(PinRefusal::Unmapped),
        Some(p) if !p.user => Some(PinRefusal::Unmapped),
        Some(p) if p.exec => Some(PinRefusal::Executable),
        Some(_) => None,
    });
    match pin_frames(&walked) {
        Ok(frames) => {
            assert_eq!(first_bad, None, "a page no guest may pin was taken");
            assert_eq!(frames.len(), walked.len());
            assert!(walked.iter().zip(&frames).all(|(p, &f)| p.is_some_and(|p| p.frame == f)));
            let grant = (word(data, 4) % (1 << 20), word(data, 1) % (1 << 20));
            let meets = frames.iter().any(|&f| f >= grant.0 && f < grant.0.saturating_add(grant.1));
            assert_eq!(in_grant(&frames, [grant]), meets);
        }
        Err(refused) => {
            assert!(!REGISTRY_ONLY.contains(&refused));
            assert_eq!(Some(refused), first_bad);
        }
    }
    let owned = usize::from(sel) % (2 * PINS_PER_SUPERVISOR);
    assert_eq!(room(owned), owned < PINS_PER_SUPERVISOR);
    // Whatever a process serves, a pin's reader is the compositor and its
    // scanout reader a display driver, and nobody else.
    let served: Vec<&str> =
        NAMES.iter().enumerate().filter(|(i, _)| sel & (1 << i) != 0).map(|(_, n)| *n).collect();
    assert_eq!(may_read(&served, DISPLAYS, false), served.contains(&READER));
    assert_eq!(may_read(&served, DISPLAYS, true), served.contains(&"driver.virtio_gpu0"));
});
