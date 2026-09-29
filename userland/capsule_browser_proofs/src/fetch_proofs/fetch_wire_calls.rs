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

//! The fake socket's side of the fetch machine's calls.

use super::fetch_wire::FakeWire;
use crate::browser::fetch::wire::{Resolved, Wire};

impl FakeWire {
    /// Everything sent on `handle`, in order.
    pub fn sent_on(&self, handle: u32) -> Vec<u8> {
        self.sent.iter().filter(|(h, _)| *h == handle).flat_map(|(_, b)| b.clone()).collect()
    }
}

impl Wire for FakeWire {
    fn rtc_now(&self) -> u64 {
        20260601000000
    }
    fn mixnet(&self) -> bool {
        self.mixnet
    }
    fn open(&mut self) -> Result<u32, ()> {
        self.next += 1;
        self.opened.push(self.next);
        Ok(self.next)
    }
    fn cached(&self, _host: &str) -> Option<[u8; 4]> {
        None
    }
    fn resolve(&mut self, host: &str) -> Resolved {
        self.resolves += 1;
        match self.names.iter().find(|(name, _)| name == host) {
            Some((_, ip)) => Resolved::Ip(*ip),
            None => Resolved::Unknown,
        }
    }
    fn connect_nb(&mut self, handle: u32, ip: [u8; 4], port: u16) -> Result<(), ()> {
        self.connects.push((handle, ip, port));
        Ok(())
    }
    fn connect_host(&mut self, handle: u32, host: &str, _port: u16) -> Result<(), ()> {
        self.waited.push((handle, String::from(host)));
        Ok(())
    }
    fn poll(&mut self, _handle: u32) -> Result<u8, ()> {
        self.polls += 1;
        Ok(if self.writable { 2 } else { 0 })
    }
    fn send(&mut self, handle: u32, bytes: &[u8]) -> Result<(), ()> {
        self.sent.push((handle, bytes.to_vec()));
        Ok(())
    }
    fn close(&mut self, handle: u32) {
        self.closed.push(handle);
    }
    fn trace(&mut self, _line: &[u8]) {}
}
