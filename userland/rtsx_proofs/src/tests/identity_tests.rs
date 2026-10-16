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

//! CAPACITY and IDENTIFY: the size in blocks, the identify layout the SATA
//! driver uses, and the card's serial told to the installer and to no one
//! else, on the wire, on the console or in a Debug print (C5).

use nonos_libc::{grant_cap, said};

use super::rig::{sdhc, serving};
use super::service_rig::{call, span, INSTALLER, KERNEL, STORE_WRITE};
use crate::model::with;
use crate::protocol::*;
use crate::serve::medium::Medium;

#[test]
fn identify_lays_out_size_model_and_medium_and_the_serial_goes_to_the_installer() {
    let (mut drv, mut medium) = serving(sdhc(0x1234_ABCD));
    let cap = call(&mut drv, &mut medium, KERNEL, OP_CAPACITY, &[]);
    assert_eq!(cap.payload, (1u64 << 20).to_le_bytes());
    grant_cap(INSTALLER, STORE_WRITE);
    let id = call(&mut drv, &mut medium, INSTALLER, OP_IDENTIFY, &[]);
    assert_eq!((id.status, id.payload.len()), (E_OK, IDENTIFY_LEN));
    let p = &id.payload;
    assert_eq!(p[..8], (1u64 << 20).to_le_bytes());
    assert_eq!(p[8..12], 512u32.to_le_bytes());
    assert_eq!((p[12], p[13], p[14], p[15]), (5, 8, 2, 0));
    assert_eq!((&p[16..21], &p[56..64]), (&b"SC32G"[..], &b"1234ABCD"[..]));
    let kernel = call(&mut drv, &mut medium, KERNEL, OP_IDENTIFY, &[]);
    assert_eq!((kernel.payload[13], &kernel.payload[16..21]), (0, &b"SC32G"[..]));
    assert!(kernel.payload[56..].iter().all(|&b| b == 0), "the serial went to the kernel");
}

#[test]
fn no_console_line_and_no_debug_print_names_the_card() {
    let (mut drv, mut medium) = serving(sdhc(0xDEAD_BEEF));
    let _ = call(&mut drv, &mut medium, KERNEL, OP_READ, &span(0, 4));
    let card = with(|r| r.pull()).expect("a card");
    let _ = call(&mut drv, &mut medium, KERNEL, OP_READ, &span(0, 4));
    with(|r| r.insert(card));
    let back = call(&mut drv, &mut medium, KERNEL, OP_READ, &span(0, 4));
    assert_eq!(back.status, E_OK);
    let lines = said().concat().to_lowercase();
    assert!(lines.contains("served as driver.rtsx0"), "{lines}");
    assert!(!lines.contains("deadbeef") && !lines.contains("sc32g"), "{lines}");
    let Medium::Ready(served) = medium else { panic!("{medium:?}") };
    let debug = format!("{served:?}");
    assert!(debug.contains("Cid(..)") && !debug.to_lowercase().contains("deadbeef"), "{debug}");
}
