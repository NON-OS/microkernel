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

//! A VMBus host, as QEMU's hw/hyperv/vmbus.c answers a guest: the contact
//! checked and a version taken or refused, the offers sent on request and
//! then the end of them, withdrawals, released relids and the unload. A
//! message a guest should not send fails the test.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use nonos_libc::{set_host, Host};

pub const ALL_VERSIONS: [u32; 8] =
    [0x5_0003, 0x5_0002, 0x5_0001, 0x5_0000, 0x4_0001, 0x4_0000, 0x3_0000, 0x2_0004];
const V5: u32 = 0x5_0000;

#[derive(Clone, Copy)]
pub struct Channel {
    pub relid: u32,
    pub interface: [u8; 16],
    pub instance: [u8; 16],
    pub sub: u16,
    pub connection: u32,
}

pub struct Vmbus {
    /// The versions this host takes.
    pub supported: Vec<u32>,
    /// The message connection a 5.0+ answer names (0 names none).
    pub named: u32,
    /// Answers nothing at all.
    pub silent: bool,
    /// Ends the offers with ALLOFFERS_DELIVERED.
    pub deliver_all: bool,
    /// Withdraws this relid after the offers, before their end.
    pub rescind_among: Option<u32>,
    /// Sends each offer cut to 100 bytes.
    pub short_offers: bool,
    pub channels: Vec<Channel>,
    /// Posts answered -EAGAIN before the next is taken.
    pub busy: u32,
    /// The connection the bus took, once a version was.
    pub connection: Option<u32>,
    /// Every message taken: (connection, kind).
    pub log: Vec<(u32, u32)>,
    pub contacts: Vec<Vec<u8>>,
    pub released: Vec<u32>,
    pub unloaded: bool,
    outbox: VecDeque<Vec<u8>>,
}

#[derive(Clone)]
pub struct Model(pub Rc<RefCell<Vmbus>>);

/// A SynIC message: VMBus's type, the size, and the payload.
pub fn synic(payload: &[u8]) -> Vec<u8> {
    let mut m = vec![0u8; 16];
    m[0] = 1;
    m[4] = payload.len() as u8;
    m.extend_from_slice(payload);
    m
}

fn u32_at(p: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(p[at..at + 4].try_into().unwrap())
}

fn u64_at(p: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(p[at..at + 8].try_into().unwrap())
}

pub fn offer_bytes(c: &Channel) -> Vec<u8> {
    let mut m = vec![0u8; 196];
    m[0] = 1;
    m[8..24].copy_from_slice(&c.interface);
    m[24..40].copy_from_slice(&c.instance);
    m[180..182].copy_from_slice(&c.sub.to_le_bytes());
    m[184..188].copy_from_slice(&c.relid.to_le_bytes());
    m[192..196].copy_from_slice(&c.connection.to_le_bytes());
    m
}

pub fn rescind_bytes(relid: u32) -> Vec<u8> {
    let mut m = vec![0u8; 12];
    m[0] = 2;
    m[8..12].copy_from_slice(&relid.to_le_bytes());
    m
}

impl Model {
    pub fn new(channels: Vec<Channel>) -> Self {
        Self(Rc::new(RefCell::new(Vmbus {
            supported: ALL_VERSIONS.to_vec(),
            named: 4,
            silent: false,
            deliver_all: true,
            rescind_among: None,
            short_offers: false,
            channels,
            busy: 0,
            connection: None,
            log: Vec::new(),
            contacts: Vec::new(),
            released: Vec::new(),
            unloaded: false,
            outbox: VecDeque::new(),
        })))
    }

    pub fn install(&self) {
        let h: Rc<RefCell<dyn Host>> = self.0.clone();
        set_host(h);
    }

    /// A message the host sends of its own accord.
    pub fn send(&self, payload: &[u8]) {
        self.0.borrow_mut().outbox.push_back(synic(payload));
    }

    /// Bytes as they come out of the slot, header and all.
    pub fn send_raw(&self, bytes: Vec<u8>) {
        self.0.borrow_mut().outbox.push_back(bytes);
    }
}

impl Vmbus {
    fn answer(&mut self, payload: &[u8]) {
        if !self.silent {
            self.outbox.push_back(synic(payload));
        }
    }

    fn contact(&mut self, connection: u32, p: &[u8]) {
        assert_eq!(p.len(), 40, "the contact is 40 bytes");
        let version = u32_at(p, 8);
        let v5 = version >= V5;
        assert_eq!(connection, if v5 { 4 } else { 1 }, "the contact's connection");
        assert_eq!(u32_at(p, 12), 0, "aimed at VP 0");
        if v5 {
            assert_eq!((p[16], p[17]), (2, 0), "SINT 2, VTL 0");
        } else {
            let page = u64_at(p, 16);
            assert!(page != 0 && page.is_multiple_of(4096), "an interrupt page");
        }
        let (m1, m2) = (u64_at(p, 24), u64_at(p, 32));
        assert!(
            m1 != 0 && m2 != 0 && m1 != m2 && m1.is_multiple_of(4096) && m2.is_multiple_of(4096)
        );
        self.contacts.push(p.to_vec());
        let taken = self.supported.contains(&version);
        if taken {
            self.connection = Some(if !v5 {
                1
            } else if self.named == 0 {
                4
            } else {
                self.named
            });
        }
        let mut r = vec![0u8; 16];
        r[0] = 15;
        r[8] = taken as u8;
        r[9] = taken as u8;
        if v5 {
            r[12..16].copy_from_slice(&self.named.to_le_bytes());
        }
        self.answer(&r);
    }
}

impl Host for Vmbus {
    fn post(&mut self, connection: u32, p: &[u8]) -> i64 {
        if self.busy > 0 {
            self.busy -= 1;
            return -11;
        }
        let kind = u32_at(p, 0);
        self.log.push((connection, kind));
        match kind {
            14 => self.contact(connection, p),
            3 => {
                assert_eq!(
                    Some(connection),
                    self.connection,
                    "offers asked on the bus's connection"
                );
                let offers: Vec<Vec<u8>> = self.channels.iter().map(offer_bytes).collect();
                for o in offers {
                    let n = if self.short_offers { 100 } else { o.len() };
                    self.answer(&o[..n]);
                }
                if let Some(r) = self.rescind_among {
                    self.answer(&rescind_bytes(r));
                }
                if self.deliver_all {
                    let mut end = vec![0u8; 8];
                    end[0] = 4;
                    self.answer(&end);
                }
            }
            12 => {
                assert_eq!(Some(connection), self.connection);
                self.released.push(u32_at(p, 8));
            }
            16 => {
                assert_eq!(Some(connection), self.connection, "unload on the bus's connection");
                self.unloaded = true;
                self.connection = None;
                let mut r = vec![0u8; 8];
                r[0] = 17;
                self.answer(&r);
            }
            k => panic!("a guest message the core should not send: {k}"),
        }
        0
    }

    fn recv(&mut self) -> Option<Vec<u8>> {
        self.outbox.pop_front()
    }
}
