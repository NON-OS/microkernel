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
//! net.nym's sender, run against a scripted gateway: the real clocks, queue
//! and step, simulated time. What leaves, and when, is all a link observer
//! sees; these hold it to the Loopix model and to the client's contract.

use std::collections::VecDeque;

use crate::cover_model::{gaps, ks_two_sample, loopix, Kind, Rates};
use crate::mixnet::cover_clock::{Clocks, Profile, Tick, LOW_RATE, NYM_RATE, RESYNC_NS};
use crate::outbound::queue::{Frame, Job, Queue, QUEUE_MAX_FRAMES};
use crate::outbound::step::{step, Step};

const SECOND: u64 = 1_000_000_000;
const HOUR: u64 = 3600 * SECOND;

struct SplitMix(u64);

impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Left {
    Real(u32),
    Loop,
    Drop,
}

/// The gateway double: it records every frame with the time it arrived, and
/// refuses the write numbered `fail_at`.
struct Gateway {
    left: Vec<(u64, Left)>,
    fail_at: Option<usize>,
}

impl Gateway {
    fn write(&mut self, now: u64, frame: &Frame) -> Result<(), u16> {
        if Some(self.left.len()) == self.fail_at {
            return Err(5);
        }
        let bytes = match frame {
            Frame::Sphinx(b) | Frame::Raw(b) => b,
        };
        let left = match bytes[0] {
            b'L' => Left::Loop,
            b'D' => Left::Drop,
            _ => Left::Real(u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]])),
        };
        self.left.push((now, left));
        Ok(())
    }
}

fn job(id: u32, frames: usize) -> Job<u32> {
    let frames: VecDeque<Frame> = (0..frames)
        .map(|_| {
            let mut f = vec![b'R'];
            f.extend_from_slice(&id.to_le_bytes());
            Frame::Sphinx(f)
        })
        .collect();
    Job { owner: Some((7, id)), frames, done: id }
}

struct Run {
    gateway: Gateway,
    /// Each job's completion: (job id, time).
    done: Vec<(u32, u64)>,
    /// Arrival time of each job.
    arrived: Vec<u64>,
    failed: Option<(Vec<u32>, Option<(u32, u32)>)>,
    empty_ticks: usize,
}

/// Run the sender for `horizon` with jobs arriving at `arrivals` (time,
/// frames), the cover builder failing when `no_cover`.
fn run(profile: Profile, horizon: u64, arrivals: &[(u64, usize)], seed: u64, fail_at: Option<usize>, no_cover: bool) -> Run {
    let mut rng = SplitMix(seed);
    let mut clocks = Clocks::start(0, profile, rng.next(), rng.next());
    let mut queue: Queue<u32> = Queue::new();
    let mut out = Run {
        gateway: Gateway { left: Vec::new(), fail_at },
        done: Vec::new(),
        arrived: Vec::new(),
        failed: None,
        empty_ticks: 0,
    };
    let mut next_job = 0usize;
    loop {
        let now = clocks.next_ns();
        if now >= horizon {
            return out;
        }
        while next_job < arrivals.len() && arrivals[next_job].0 <= now {
            let (at, frames) = arrivals[next_job];
            if queue.push(job(next_job as u32, frames)).is_ok() {
                out.arrived.push(at);
            } else {
                out.arrived.push(u64::MAX);
            }
            next_job += 1;
        }
        let cover = |tick: Tick| (!no_cover).then(|| vec![if tick == Tick::Loop { b'L' } else { b'D' }]);
        let gateway = &mut out.gateway;
        match step(now, profile, &mut clocks, &mut queue, rng.next(), cover, |f| gateway.write(now, f)) {
            Step::Idle => panic!("a tick was due at {now}"),
            Step::Real { done } => {
                if let Some((_, id)) = done {
                    out.done.push((id, now));
                }
            }
            Step::Cover(_) => {}
            Step::NoCover(_) => out.empty_ticks += 1,
            Step::Failed { dropped, owner, .. } => {
                out.failed = Some((dropped.into_iter().map(|j| j.done).collect(), owner));
                return out;
            }
        }
    }
}

fn poisson(rng: &mut SplitMix, mean: u64, horizon: u64, frames: usize) -> Vec<(u64, usize)> {
    let mut at = 0u64;
    let mut out = Vec::new();
    loop {
        at += crate::exp_delay::exp_delay_ns(rng.next(), mean).max(1);
        if at >= horizon {
            return out;
        }
        out.push((at, frames));
    }
}

fn departures(r: &Run) -> Vec<(u64, Kind)> {
    r.gateway.left.iter().map(|&(t, l)| (t, if matches!(l, Left::Real(_)) { Kind::Real } else { Kind::Drop })).collect()
}

fn per_second(r: &Run, horizon: u64) -> Vec<u64> {
    let mut counts = vec![0u64; (horizon / SECOND) as usize];
    for &(t, _) in &r.gateway.left {
        counts[(t / SECOND) as usize] += 1;
    }
    counts
}

#[test]
fn the_sender_leaves_as_the_model_says() {
    let real = run(LOW_RATE, HOUR, &[], 1, None, false);
    let mut m = SplitMix(2);
    let mut draw = || m.next();
    let rates = Rates { send_mean_ns: LOW_RATE.send_mean_ns, loop_mean_ns: LOW_RATE.loop_mean_ns };
    let (model, _) = loopix(&[], HOUR, rates, &mut draw);
    let (d, p) = ks_two_sample(&gaps(&departures(&real)), &gaps(&model));
    println!("sender vs model, low rate, gaps: n={} vs {} D={d:.4} p={p:.3}", real.gateway.left.len(), model.len());
    assert!(p > 0.01, "D={d} p={p}");
    let rate = real.gateway.left.len() as f64 / 3600.0;
    assert!((5.0..6.0).contains(&rate), "{rate} packets a second");
}

#[test]
fn idle_and_active_are_one_distribution_at_each_profile() {
    for (name, profile, mean) in [("low", LOW_RATE, SECOND), ("nym", NYM_RATE, SECOND / 10)] {
        let idle = run(profile, HOUR, &[], 10, None, false);
        let mut a = SplitMix(11);
        let arrivals = poisson(&mut a, mean, HOUR, 1);
        let active = run(profile, HOUR, &arrivals, 12, None, false);
        let (dg, pg) = ks_two_sample(&gaps(&departures(&idle)), &gaps(&departures(&active)));
        let (dc, pc) = ks_two_sample(&per_second(&idle, HOUR), &per_second(&active, HOUR));
        let mut waits: Vec<u64> = active.done.iter().map(|&(id, at)| at - active.arrived[id as usize]).collect();
        waits.sort_unstable();
        let mean_ms = waits.iter().sum::<u64>() as f64 / waits.len() as f64 / 1e6;
        let p50 = waits[waits.len() / 2] as f64 / 1e6;
        let p99 = waits[waits.len() * 99 / 100] as f64 / 1e6;
        println!(
            "{name}: gaps D={dg:.4} p={pg:.3}; per-second D={dc:.4} p={pc:.3}; {} messages, wait mean {mean_ms:.0} ms p50 {p50:.0} ms p99 {p99:.0} ms",
            waits.len()
        );
        assert!(pg > 0.05 && pc > 0.05, "{name}: {pg} {pc}");
        assert_eq!(waits.len(), arrivals.len(), "every message left within the hour");
    }
}

#[test]
fn a_real_message_takes_a_cover_slot_and_moves_nothing() {
    // Same clocks: the departure times are identical to the last nanosecond
    // with and without a message; three drop covers became its three frames.
    let idle = run(LOW_RATE, 60 * SECOND, &[], 21, None, false);
    let active = run(LOW_RATE, 60 * SECOND, &[(SECOND, 3)], 21, None, false);
    let times = |r: &Run| r.gateway.left.iter().map(|e| e.0).collect::<Vec<_>>();
    assert_eq!(times(&idle), times(&active));
    let count = |r: &Run, f: fn(&Left) -> bool| r.gateway.left.iter().filter(|e| f(&e.1)).count();
    assert_eq!(count(&active, |l| matches!(l, Left::Real(0))), 3);
    assert_eq!(count(&idle, |l| *l == Left::Drop), count(&active, |l| *l == Left::Drop) + 3);
    assert_eq!(count(&idle, |l| *l == Left::Loop), count(&active, |l| *l == Left::Loop));
    assert_eq!(active.done.len(), 1, "done once, at its last frame");
    let last_real = active.gateway.left.iter().filter(|e| e.1 == Left::Real(0)).map(|e| e.0).max();
    assert_eq!(Some(active.done[0].1), last_real);
}

#[test]
fn a_burst_fills_the_queue_then_leaves_in_order() {
    let mut queue: Queue<u32> = Queue::new();
    let mut accepted = 0u32;
    while queue.push(job(accepted, 4)).is_ok() {
        accepted += 1;
    }
    assert_eq!(accepted as usize * 4, QUEUE_MAX_FRAMES, "refused once it held the most it may");
    // All at once at t=0, then the clock drains them one a send tick.
    let arrivals: Vec<(u64, usize)> = (0..accepted).map(|_| (0, 4)).collect();
    let r = run(LOW_RATE, 600 * SECOND, &arrivals, 31, None, false);
    let reals: Vec<u32> = r.gateway.left.iter().filter_map(|e| if let Left::Real(id) = e.1 { Some(id) } else { None }).collect();
    assert_eq!(reals.len(), QUEUE_MAX_FRAMES);
    assert!(reals.windows(2).all(|w| w[0] <= w[1]), "jobs left in the order they came");
    let done: Vec<u32> = r.done.iter().map(|d| d.0).collect();
    assert_eq!(done, (0..accepted).collect::<Vec<_>>(), "each done once, in order");
    // While the queue held frames, no send tick carried a drop cover.
    let last_real = r.gateway.left.iter().rposition(|e| matches!(e.1, Left::Real(_))).unwrap_or(0);
    assert!(r.gateway.left[..last_real].iter().all(|e| e.1 != Left::Drop));
}

#[test]
fn a_failed_write_drops_the_queue_and_says_whose_it_was() {
    // Two jobs of three frames; the second real frame's write fails.
    let arrivals = [(0, 3), (0, 3)];
    let mut r = run(LOW_RATE, 60 * SECOND, &arrivals, 41, None, false);
    let first_real = r.gateway.left.iter().position(|e| matches!(e.1, Left::Real(_))).unwrap_or(0);
    r = run(LOW_RATE, 60 * SECOND, &arrivals, 41, Some(first_real + 1), false);
    let (dropped, owner) = r.failed.expect("the write failed");
    assert_eq!(dropped, vec![0, 1], "both jobs dropped, the failing one still queued");
    assert_eq!(owner, None);
    assert!(r.done.is_empty(), "no job is done that did not wholly leave");
    // A failure on a job's last frame names its owner, since it is no longer queued.
    let arrivals = [(0, 1), (0, 2)];
    let ok = run(LOW_RATE, 60 * SECOND, &arrivals, 42, None, false);
    let first_real = ok.gateway.left.iter().position(|e| matches!(e.1, Left::Real(_))).unwrap_or(0);
    let r = run(LOW_RATE, 60 * SECOND, &arrivals, 42, Some(first_real), false);
    let (dropped, owner) = r.failed.expect("the write failed");
    assert_eq!((dropped, owner), (vec![1], Some((7, 0))));
}

#[test]
fn with_no_cover_to_send_a_tick_passes_empty() {
    let r = run(LOW_RATE, 60 * SECOND, &[(SECOND, 2)], 51, None, true);
    assert!(r.gateway.left.iter().all(|e| matches!(e.1, Left::Real(_))));
    assert_eq!(r.gateway.left.len(), 2);
    assert!(r.empty_ticks > 100);
}

#[test]
fn a_clock_far_behind_starts_again_rather_than_bursting() {
    let mut rng = SplitMix(61);
    let mut c = Clocks::start(0, NYM_RATE, rng.next(), rng.next());
    let late = 30 * SECOND;
    let mut fired = 0;
    while c.take(late, NYM_RATE, rng.next()).is_some() {
        fired += 1;
    }
    // Each clock fires once and starts again from now.
    assert_eq!(fired, 2);
    assert!(c.next_ns() > late && late + RESYNC_NS > late);
}

#[test]
fn what_each_profile_lets_a_client_send_and_receive() {
    // From the capsule's own packet constants. Outbound: one message packet
    // a sending tick. Inbound: each reply packet spends one reply block, and
    // blocks go out in top up packets on the same clock, so a tick of top up
    // buys as many replies as blocks fit in a packet. An upper bound: it
    // leaves out acknowledgements and the blocks a request carries itself.
    use crate::sphinx::constants::{
        ACK_PAYLOAD_SIZE, HEADER_SIZE, PAYLOAD_KEY_SEED_SIZE, REGULAR_PACKET_PLAINTEXT,
    };
    let per_packet = REGULAR_PACKET_PLAINTEXT - (19 + HEADER_SIZE + ACK_PAYLOAD_SIZE) - 32 - 7;
    let hops = 4;
    let block = 16 + HEADER_SIZE + 32 + hops * PAYLOAD_KEY_SEED_SIZE;
    let blocks_per_packet = per_packet / block;
    for (name, profile) in [("low", LOW_RATE), ("nym", NYM_RATE)] {
        let ticks = SECOND as f64 / profile.send_mean_ns as f64;
        let up = ticks * per_packet as f64 / 1024.0;
        let down = ticks * blocks_per_packet as f64 * per_packet as f64 / 1024.0;
        println!(
            "{name}: {ticks:.0} sending ticks/s, {per_packet} B a packet, {blocks_per_packet} blocks a top up: \
             up to {up:.1} KiB/s out, {down:.1} KiB/s in"
        );
        assert!(up > 0.0 && down > up);
        // The status op reports the same, in whole KiB a second.
        let (out, inbound) = crate::mixnet::ceiling::ceilings(profile, per_packet as u64, blocks_per_packet as u64);
        assert_eq!((u64::from(out), u64::from(inbound)), (up as u64, down as u64));
    }
    assert_eq!(crate::mixnet::ceiling::ceilings(LOW_RATE, per_packet as u64, blocks_per_packet as u64), (7, 23));
    assert_eq!(crate::mixnet::ceiling::ceilings(NYM_RATE, per_packet as u64, blocks_per_packet as u64), (78, 234));
}
