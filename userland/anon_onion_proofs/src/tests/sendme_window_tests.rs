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


//! Flow control against an exit that enforces it the way a Tor exit does.
//!
//! The simulated exit stops at an empty window, records the digest of every
//! hundredth DATA cell it sends, and destroys the circuit on a circuit SENDME
//! that is not version 1 naming the next recorded digest. Two megabytes then
//! have to come through with the client's own pump and SENDME code, once with
//! a reader that keeps up and once with one that falls behind: that is a model
//! download, and the case where a SENDME held back is overtaken by the next
//! hundred cells.

use super::world::{net, Faults, Net};
use crate::manager::{send_data, sendme_tick};
use std::vec::Vec;

const BODY: usize = 2 * 1024 * 1024;
/// A tier's download in miniature: hundreds of SENDME boundaries, not two.
const LONG_BODY: usize = 32 * 1024 * 1024;

/// Run the stream to its end, reading at most `per_round` bytes a round, and
/// return everything read.
fn download(n: &mut Net, id: u16, per_round: usize) -> Vec<u8> {
    let mut got = Vec::new();
    let rounds = n.world.service_reply.len() / 64;
    let circuit = n.state.streams.iter().find(|s| s.id == id).unwrap().circuit;
    for _ in 0..rounds.max(20_000) {
        n.exchange();
        let Some(stream) = n.state.streams.iter_mut().find(|s| s.id == id) else { break };
        let take = stream.inbound.len().min(per_round);
        got.extend(stream.inbound.drain(..take));
        // The capsule's idle turn: SENDMEs held while the reader was behind
        // are paid once it has read.
        sendme_tick(&mut n.state);
        let (stage, rest) = n.stream(id);
        if stage.starts_with("ended") && rest.is_empty() {
            break;
        }
        if !n.world.sendme_refused.is_empty() || n.world.destroyed.contains(&circuit) {
            break;
        }
    }
    got
}

fn start(per_round: usize) -> (Net, Vec<u8>, u32) {
    start_sized(BODY, per_round)
}

fn start_sized(body: usize, per_round: usize) -> (Net, Vec<u8>, u32) {
    let mut n = net(Faults::default(), |_| {});
    n.world.strict_windows = true;
    n.world.service_reply = (0..body).map(|i| (i * 7 + i / 498) as u8).collect();
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(40);
    assert_eq!(n.stream(id).0, "open");
    assert!(send_data(&mut n.state, id, b"GET /model.gguf HTTP/1.1\r\n\r\n").is_ok());
    let circuit = n.state.streams.iter().find(|s| s.id == id).unwrap().circuit;
    let got = download(&mut n, id, per_round);
    (n, got, circuit)
}

fn check(n: &Net, got: &[u8], circuit: u32) {
    assert!(n.world.sendme_refused.is_empty(), "the exit refused a SENDME and destroyed the circuit");
    assert!(!n.world.destroyed.contains(&circuit), "the client tore the circuit down");
    assert_eq!(got.len(), n.world.service_reply.len(), "the whole body came through");
    assert!(got == &n.world.service_reply[..], "byte for byte");
    // The body's own cells; the descriptor lookup before it sent a few more.
    let cells = n.world.service_reply.len().div_ceil(498);
    assert!(n.world.data_cells_sent >= cells);
    // The exit only sends past a thousand cells on a circuit SENDME every
    // hundred, so a whole body means every one of them was paid and matched.
    assert!(n.world.circuit_sendmes >= (cells - 1000) / 100, "{} circuit SENDMEs", n.world.circuit_sendmes);
    assert!(n.world.stream_sendmes >= (cells - 500) / 50, "{} stream SENDMEs", n.world.stream_sendmes);
}

#[test]
fn two_megabytes_through_an_exit_that_enforces_the_windows() {
    let (n, got, circuit) = start(usize::MAX);
    check(&n, &got, circuit);
}

#[test]
fn a_reader_that_falls_behind_still_pays_each_sendme_with_its_own_digest() {
    // Eight kilobytes a round against hundreds of cells a round: the buffer
    // sits above both high water marks, so SENDMEs are held while the exit
    // spends what is left of its windows. Held ones are then paid together,
    // each naming the cell at its own boundary.
    let (n, got, circuit) = start(8 * 1024);
    check(&n, &got, circuit);
}

#[test]
fn thirty_two_megabytes_to_a_reader_far_behind_keep_one_circuit() {
    // Sixty seven thousand cells, over six hundred SENDME boundaries, read
    // sixteen kilobytes a round: the model fetcher writing a tier to disk.
    let (n, got, circuit) = start_sized(LONG_BODY, 16 * 1024);
    check(&n, &got, circuit);
    assert!(n.world.circuit_sendmes >= 660, "{}", n.world.circuit_sendmes);
}
