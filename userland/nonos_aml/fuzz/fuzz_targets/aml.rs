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

//! A hostile DSDT or SSDT and hostile hardware behind it. The first byte
//! picks how the table arrives (wrapped in a good header with a good
//! checksum, so the fuzzer reaches the parser, or as raw bytes for the
//! header checks) and how many of the rest are AML; what follows the AML
//! answers every region read. The table loads, every space is connected so
//! each _REG runs, and every method it defined is called with integer
//! arguments. Nothing may panic; the interpreter's own budget (steps, call
//! depth, stack, allocation, time) must end every evaluation, which
//! libFuzzer's -timeout and -rss_limit_mb hold it to.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_aml::exec::{ExecLimits, Host, Machine, RegionAccess};
use nonos_aml::load::Aml;
use nonos_aml::namespace::Kind;
use nonos_aml::object::Object;
use nonos_aml::region::Space;

/// Methods called at most, so one input stays well inside the timeout.
const CALLS: usize = 64;

struct Hostile<'a> {
    answers: &'a [u8],
    at: usize,
    clock: u64,
}

impl Hostile<'_> {
    fn next(&mut self) -> u64 {
        let mut w = [0u8; 8];
        for b in &mut w {
            *b = self.answers.get(self.at % self.answers.len().max(1)).copied().unwrap_or(0xff);
            self.at += 1;
        }
        u64::from_le_bytes(w)
    }
}

impl Host for Hostile<'_> {
    fn sleep(&mut self, ms: u64) {
        self.clock = self.clock.saturating_add(ms.saturating_mul(10_000));
    }
    fn stall(&mut self, us: u64) {
        self.clock = self.clock.saturating_add(us.saturating_mul(10));
    }
    fn timer(&mut self) -> u64 {
        self.clock = self.clock.saturating_add(10);
        self.clock
    }
    fn debug(&mut self, _: &Object) {}
    fn fatal(&mut self, _: u8, _: u32, _: u64) {}
    fn region_read(&mut self, _: &RegionAccess<'_>) -> Option<u64> {
        let v = self.next();
        (v & 0xff != 0xee).then_some(v)
    }
    fn region_write(&mut self, _: &RegionAccess<'_>, _: u64) -> bool {
        self.next() & 1 == 0
    }
    fn transaction(&mut self, _: &RegionAccess<'_>, _: [u8; 3], _: Option<&[u8]>, data: &mut Vec<u8>) -> bool {
        let n = (self.next() % 64) as usize;
        data.clear();
        for _ in 0..n {
            data.push(self.next() as u8);
        }
        n != 0
    }
    fn gpio(&mut self, _: &[u8], _: u32, count: u32, _: Option<u64>) -> Option<u64> {
        let v = self.next();
        Some(if count >= 64 { v } else { v & ((1u64 << count) - 1) })
    }
    fn table(&mut self, _: &[u8], _: &[u8], _: &[u8]) -> Option<Vec<u8>> {
        None
    }
    fn admit_table(&mut self, _: &[u8]) -> bool {
        self.next() & 1 == 0
    }
    fn lock_word(&mut self, _: u32, new: u32) -> Option<u32> {
        Some(new)
    }
    fn lock_released(&mut self) {}
}

fn table(sig: &[u8; 4], aml: &[u8]) -> Vec<u8> {
    let mut t = Vec::with_capacity(36 + aml.len());
    t.extend_from_slice(sig);
    t.extend_from_slice(&((36 + aml.len()) as u32).to_le_bytes());
    t.extend_from_slice(&[2, 0]);
    t.extend_from_slice(b"NONOS FUZZTBL1");
    t.extend_from_slice(&[1, 0, 0, 0]);
    t.extend_from_slice(b"FUZZ");
    t.extend_from_slice(&[1, 0, 0, 0]);
    t.extend_from_slice(aml);
    let sum = t.iter().fold(0u8, |s, b| s.wrapping_add(*b));
    t[9] = 0u8.wrapping_sub(sum);
    t
}

fuzz_target!(|data: &[u8]| {
    let Some((&mode, rest)) = data.split_first() else {
        return;
    };
    // The low bits say where the AML ends and the hardware's answers begin.
    let split = rest.len() * usize::from(mode >> 2) / 63;
    let (aml_bytes, answers) = rest.split_at(split.min(rest.len()));
    let bytes = match mode & 3 {
        0 => table(b"DSDT", aml_bytes),
        1 => table(b"SSDT", aml_bytes),
        _ => aml_bytes.to_vec(),
    };
    let mut aml = Aml::default();
    let mut host = Hostile { answers, at: 0, clock: 0 };
    let limits = ExecLimits::DEFAULT;
    if Machine::new(&mut aml, &mut host, limits).load_table(&mut aml, bytes).is_err() {
        return;
    }
    for space in 0..=0x0Au8 {
        let _ = Machine::new(&mut aml, &mut host, limits).connect(&mut aml, Space(space));
    }
    let methods: Vec<(u32, u8)> = (0..aml.ns.len() as u32)
        .filter_map(|id| match aml.ns.get(id).map(|e| &e.kind) {
            Some(Kind::Method(info)) => Some((id, info.args())),
            _ => None,
        })
        .take(CALLS)
        .collect();
    for (id, args) in methods {
        let args = (0..args).map(|i| Object::Integer(u64::from(i) * 0x1111)).collect();
        let _ = Machine::new(&mut aml, &mut host, limits).evaluate(&mut aml, id, args);
    }
});
