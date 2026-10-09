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

use crate::outputs::{Output, Outputs, HOLD_MS, MAX_HELD};

fn out(client: u32, made_ms: i64) -> Output {
    Output { client, base: 0x1000 * (client as usize + 1), len: 4096, made_ms }
}

fn everyone_alive(_: u32) -> bool {
    true
}

/// Every output still held, by letting them all go.
fn drain(outputs: &mut Outputs) -> Vec<Output> {
    let mut all = Vec::new();
    outputs.sweep(0, 0, |_| false, |o| all.push(o));
    all
}

#[test]
fn a_client_asking_again_lets_its_last_output_go() {
    let mut outputs = Outputs::new();
    assert_eq!(outputs.hold(out(7, 0)), None);
    assert_eq!(outputs.hold(out(8, 0)), None);
    let mut gone = Vec::new();
    outputs.sweep(7, 10, everyone_alive, |o| gone.push(o));
    assert_eq!(gone, vec![out(7, 0)]);
    assert_eq!(drain(&mut outputs), vec![out(8, 0)]);
}

#[test]
fn an_ended_clients_outputs_are_let_go() {
    let mut outputs = Outputs::new();
    outputs.hold(out(3, 0));
    outputs.hold(out(4, 0));
    let mut gone = Vec::new();
    outputs.sweep(9, 10, |pid| pid != 3, |o| gone.push(o));
    assert_eq!(gone, vec![out(3, 0)]);
    assert_eq!(drain(&mut outputs), vec![out(4, 0)]);
}

#[test]
fn an_output_nobody_took_is_let_go_after_the_hold_time() {
    let mut outputs = Outputs::new();
    outputs.hold(out(5, 1_000));
    let mut gone = Vec::new();
    outputs.sweep(9, 1_000 + HOLD_MS - 1, everyone_alive, |o| gone.push(o));
    assert!(gone.is_empty(), "kept until the hold time is up");
    outputs.sweep(9, 1_000 + HOLD_MS, everyone_alive, |o| gone.push(o));
    assert_eq!(gone, vec![out(5, 1_000)]);
}

#[test]
fn a_clock_that_reads_earlier_keeps_the_output() {
    let mut outputs = Outputs::new();
    outputs.hold(out(5, i64::MAX));
    let mut gone = Vec::new();
    outputs.sweep(9, i64::MIN, everyone_alive, |o| gone.push(o));
    assert!(gone.is_empty());
}

#[test]
fn at_most_four_are_held_and_the_oldest_goes_first() {
    let mut outputs = Outputs::new();
    for client in 0..MAX_HELD as u32 {
        assert_eq!(outputs.hold(out(client + 1, 100 + client as i64)), None);
    }
    assert_eq!(outputs.hold(out(50, 500)), Some(out(1, 100)));
    assert_eq!(outputs.hold(out(51, 501)), Some(out(2, 101)));
    let mut held: Vec<u32> = drain(&mut outputs).iter().map(|o| o.client).collect();
    held.sort();
    assert_eq!(held, vec![3, 4, 50, 51]);
}

/// xorshift64, so the run is the same every time.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

#[test]
fn every_output_made_is_held_or_let_go_exactly_once() {
    for seed in 1..=64u64 {
        let mut rng = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        let mut outputs = Outputs::new();
        let mut made = Vec::new();
        let mut gone = Vec::new();
        let mut now = 0i64;
        for step in 0..2000usize {
            now += (rng.next() % 5_000) as i64;
            let client = (rng.next() % 6) as u32 + 1;
            let dead = (rng.next() % 6) as u32 + 1;
            outputs.sweep(client, now, |pid| pid != dead, |o| gone.push(o));
            let o = Output { client, base: step, len: 4096, made_ms: now };
            made.push(o);
            if let Some(oldest) = outputs.hold(o) {
                gone.push(oldest);
            }
            let held = drain_copy(&mut outputs);
            assert!(held.len() <= MAX_HELD);
            assert_eq!(held.iter().filter(|h| h.client == client).count(), 1);
        }
        gone.extend(drain(&mut outputs));
        gone.sort_by_key(|o| o.base);
        assert_eq!(gone, made, "seed {seed}");
    }
}

/// What is held, leaving it held.
fn drain_copy(outputs: &mut Outputs) -> Vec<Output> {
    let all = drain(outputs);
    for o in &all {
        assert_eq!(outputs.hold(*o), None);
    }
    all
}
