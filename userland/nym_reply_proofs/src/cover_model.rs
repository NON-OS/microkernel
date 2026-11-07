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
//! A model of net.nym's sender on the wire (,
//! 4.4). Not capsule code: `shipped` is how the capsule sent before the
//! Loopix sender (server/outbound), and `loopix` is the specification that
//! sender is held to in sender_tests.rs. The delays are drawn with the
//! capsule's own `exp_delay_ns`.
//!
//! An observer of the gateway link sees when packets leave and nothing else:
//! every Sphinx packet is the same size and sealed. So a sender hides whether
//! its user is active exactly when the departure times have one distribution
//! either way.

use crate::exp_delay::exp_delay_ns;

/// What left, and when.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Real,
    /// Drop cover: a packet to a random recipient, discarded there.
    Drop,
    /// Loop cover: a packet routed back to this client.
    Loop,
    /// The WebSocket ping the shipped keepalive sends.
    Ping,
}

/// Loopix rates (Piotrowska et al., 2017), as Nym's client names them: the
/// mean gap of the sending stream, real or drop, and of the loop stream.
#[derive(Clone, Copy)]
pub struct Rates {
    pub send_mean_ns: u64,
    pub loop_mean_ns: u64,
}

/// Nym's client defaults: a send every 20 ms on average, a loop every 200 ms.
pub const NYM_DEFAULT: Rates = Rates { send_mean_ns: 20_000_000, loop_mean_ns: 200_000_000 };

/// The shipped keepalive interval (server/keepalive.rs).
pub const PING_EVERY_NS: u64 = 20_000_000_000;

/// The sender before server/outbound: a real packet the moment its message
/// arrived, and a ping every 20 s. Nothing in the tree sent OP_COVER_TICK, so
/// no cover.
pub fn shipped(arrivals: &[u64], horizon_ns: u64) -> Vec<(u64, Kind)> {
    let mut out: Vec<(u64, Kind)> = arrivals.iter().map(|&t| (t, Kind::Real)).collect();
    out.extend((1..).map(|k| k * PING_EVERY_NS).take_while(|&t| t < horizon_ns).map(|t| (t, Kind::Ping)));
    out.sort_unstable_by_key(|e| e.0);
    out
}

/// The Loopix sender. Two independent Poisson clocks: at each tick of the
/// sending clock the oldest queued real packet leaves, or a drop cover when
/// none waits; at each tick of the loop clock a loop cover leaves. Returns
/// the departures and each real packet's wait in the queue.
pub fn loopix(
    arrivals: &[u64],
    horizon_ns: u64,
    rates: Rates,
    random: &mut impl FnMut() -> u64,
) -> (Vec<(u64, Kind)>, Vec<u64>) {
    let mut out = Vec::new();
    let mut waits = Vec::new();
    let mut queued = 0usize;
    let mut send_at = exp_delay_ns(random(), rates.send_mean_ns);
    while send_at < horizon_ns {
        // Real packets that arrived by this tick join the queue.
        while queued < arrivals.len() && arrivals[queued] <= send_at {
            queued += 1;
        }
        if waits.len() < queued {
            waits.push(send_at - arrivals[waits.len()]);
            out.push((send_at, Kind::Real));
        } else {
            out.push((send_at, Kind::Drop));
        }
        send_at += exp_delay_ns(random(), rates.send_mean_ns).max(1);
    }
    let mut loop_at = exp_delay_ns(random(), rates.loop_mean_ns);
    while loop_at < horizon_ns {
        out.push((loop_at, Kind::Loop));
        loop_at += exp_delay_ns(random(), rates.loop_mean_ns).max(1);
    }
    out.sort_unstable_by_key(|e| e.0);
    (out, waits)
}

/// The gaps between consecutive departures, which is all the link shows.
pub fn gaps(departures: &[(u64, Kind)]) -> Vec<u64> {
    departures.windows(2).map(|w| w[1].0 - w[0].0).collect()
}

/// Two-sample Kolmogorov-Smirnov: the statistic D and its asymptotic
/// p-value (Kolmogorov distribution, with Stephens' small-sample term).
pub fn ks_two_sample(a: &[u64], b: &[u64]) -> (f64, f64) {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    a.sort_unstable();
    b.sort_unstable();
    let (n, m) = (a.len() as f64, b.len() as f64);
    let (mut i, mut j, mut d) = (0usize, 0usize, 0f64);
    while i < a.len() && j < b.len() {
        let x = a[i].min(b[j]);
        while i < a.len() && a[i] == x {
            i += 1;
        }
        while j < b.len() && b[j] == x {
            j += 1;
        }
        d = d.max((i as f64 / n - j as f64 / m).abs());
    }
    let ne = n * m / (n + m);
    let lambda = (ne.sqrt() + 0.12 + 0.11 / ne.sqrt()) * d;
    (d, kolmogorov_q(lambda))
}

/// Q(l) = 2 sum (-1)^(k-1) exp(-2 k^2 l^2), the chance a D this large or
/// larger comes from one distribution.
fn kolmogorov_q(lambda: f64) -> f64 {
    if lambda < 0.2 {
        return 1.0;
    }
    let mut sum = 0.0;
    for k in 1..=100 {
        let k = k as f64;
        let term = (-2.0 * k * k * lambda * lambda).exp();
        sum += if k as u64 % 2 == 1 { term } else { -term };
        if term < 1e-12 {
            break;
        }
    }
    (2.0 * sum).clamp(0.0, 1.0)
}
