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

//! The console lines. They name the VMBus version and each channel by its
//! interface, never an instance GUID: those name this virtual machine.

use nonos_libc::mk_debug;

use crate::connect::Bus;
use crate::guid::name;
use crate::msg::Offer;
use crate::offers::Offers;

const NAME: &[u8] = b"driver.vmbus0: ";

pub fn up(bus: Bus, offers: &Offers) {
    let mut a = [0u8; 10];
    let mut b = [0u8; 10];
    let mut c = [0u8; 10];
    let mut d = [0u8; 10];
    line(&[
        b"VMBus ",
        dec(bus.version >> 16, &mut a),
        b".",
        dec(bus.version & 0xFFFF, &mut b),
        b" on connection ",
        dec(bus.connection, &mut c),
        b", ",
        dec(offers.count() as u32, &mut d),
        b" channels offered, none opened",
    ]);
    for o in offers.iter() {
        offered(o);
    }
}

pub fn offered(o: &Offer) {
    let mut a = [0u8; 10];
    let mut b = [0u8; 10];
    if o.sub_channel == 0 {
        line(&[b"channel ", dec(o.relid, &mut a), b": ", name(o.interface).as_bytes()]);
    } else {
        line(&[
            b"channel ",
            dec(o.relid, &mut a),
            b": ",
            name(o.interface).as_bytes(),
            b", sub-channel ",
            dec(o.sub_channel as u32, &mut b),
        ]);
    }
}

pub fn withdrawn(o: &Offer) {
    let mut a = [0u8; 10];
    line(&[b"channel ", dec(o.relid, &mut a), b" (", name(o.interface).as_bytes(), b") withdrawn"]);
}

pub fn full(o: &Offer) {
    let mut a = [0u8; 10];
    line(&[b"channel ", dec(o.relid, &mut a), b" not kept: the table is full"]);
}

fn line(parts: &[&[u8]]) {
    let mut out = [0u8; 120];
    let mut n = 0;
    for &b in NAME.iter().chain(parts.iter().flat_map(|p| p.iter())) {
        if n == out.len() - 1 {
            break;
        }
        out[n] = b;
        n += 1;
    }
    out[n] = b'\n';
    let _ = mk_debug(out.as_ptr(), n + 1);
}

fn dec(mut v: u32, buf: &mut [u8; 10]) -> &[u8] {
    let mut i = buf.len();
    loop {
        i -= 1;
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
        if v == 0 {
            return &buf[i..];
        }
    }
}
