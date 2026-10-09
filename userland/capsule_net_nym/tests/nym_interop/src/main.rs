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


//! Build with net.nym's own source, read with the code the network runs.
//!
//! Offline: no gateway, no network. Every packet, reply block and repliable
//! message is produced by the shipping capsule source and then parsed,
//! applied and peeled hop by hop by sphinx-packet 0.6.0 (what nym-node 1.41
//! locks) and nym-sphinx-anonymous-replies 1.22.1. A mixnet that cannot read
//! a packet drops it in silence; this says which layer it would have been.

extern crate alloc;
#[path = "../../live_gateway/src/crypto_shim.rs"]
mod crypto;
#[path = "../../live_gateway/src/sphinx_mod.rs"]
mod sphinx_root;
use sphinx_root::sphinx;
#[path = "../../live_gateway/src/surb_mod.rs"]
mod surb;
mod message;

use nym_sphinx_anonymous_replies::requests::{RepliableMessage, RepliableMessageContent};
use nym_sphinx_anonymous_replies::ReplySurb as RefSurb;
use nym_sphinx_params::{PacketSize, PacketType, SphinxKeyRotation};
use rand::RngCore;
use sphinx_packet::crypto::{PrivateKey, PublicKey};
use sphinx_packet::{ProcessedPacketData, SphinxPacket as RefPacket};

use sphinx::node::{Destination, Node};

fn secret() -> PrivateKey {
    let mut b = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut b);
    PrivateKey::from(b)
}

fn address(i: u8) -> [u8; 32] {
    let mut a = [0u8; 32];
    a[0] = 4;
    a[1] = i;
    a
}

fn route(hops: usize) -> (Vec<PrivateKey>, Vec<Node>) {
    let secrets: Vec<PrivateKey> = (0..hops).map(|_| secret()).collect();
    let nodes = secrets
        .iter()
        .enumerate()
        .map(|(i, s)| Node { address: address(i as u8 + 1), pub_key: PublicKey::from(s).to_bytes() })
        .collect();
    (secrets, nodes)
}

fn delays(n: usize) -> Vec<[u8; 8]> {
    (0..n).map(|_| 15_000_000u64.to_be_bytes()).collect()
}

/// Unwrap with the reference, hop by hop. Returns the final destination and
/// the recovered plaintext.
fn unwrap(wire: &[u8], secrets: &[PrivateKey]) -> Result<([u8; 32], Vec<u8>), String> {
    let mut packet = RefPacket::from_bytes(wire).map_err(|e| format!("from_bytes: {e}"))?;
    for (i, s) in secrets.iter().enumerate() {
        let p = packet.process(s).map_err(|e| format!("hop {}: {e}", i + 1))?;
        match p.data {
            ProcessedPacketData::ForwardHop { next_hop_packet, next_hop_address, delay } => {
                if i + 1 == secrets.len() {
                    return Err("forward at the last hop".into());
                }
                if next_hop_address.as_bytes() != &address(i as u8 + 2) {
                    return Err(format!("hop {} routes to the wrong node", i + 1));
                }
                if delay.to_nanos() != 15_000_000 {
                    return Err(format!("hop {} read delay {} ns", i + 1, delay.to_nanos()));
                }
                packet = next_hop_packet;
            }
            ProcessedPacketData::FinalHop { destination, payload, .. } => {
                if i + 1 != secrets.len() {
                    return Err(format!("final at hop {}", i + 1));
                }
                let text = payload.recover_plaintext().map_err(|e| format!("recover: {e}"))?;
                return Ok((destination.as_bytes(), text));
            }
        }
    }
    Err("ran out of hops".into())
}

fn forward(hops: usize) -> Result<(), String> {
    let (secrets, nodes) = route(hops);
    let dest = Destination { address: [7u8; 32], identifier: [0u8; 16] };
    let mut initial = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut initial);
    let msg = b"nonos forward packet, version 259";
    let built = sphinx::packet::build_packet(&initial, &nodes, &dest, &delays(hops), sphinx::constants::PACKET_VERSION, msg)
        .map_err(|e| format!("our build: {e:?}"))?;
    let wire = built.to_bytes().ok_or("serialise")?;
    let (to, text) = unwrap(&wire, &secrets)?;
    if to != [7u8; 32] { return Err("wrong destination".into()); }
    if text != msg { return Err("payload changed".into()); }
    Ok(())
}

fn reply_block(hops: usize) -> Result<(), String> {
    let (secrets, nodes) = route(hops);
    let me = [9u8; 32];
    let ours = surb::build_surb(&nodes, &delays(hops), &me).map_err(|e| format!("our surb: {e:?}"))?;
    let bytes = surb::surb_bytes(&ours);
    let theirs = RefSurb::from_bytes(&bytes).map_err(|e| format!("reference parse: {e}"))?;
    let size = PacketSize::RegularPacket;
    let mut reply = vec![0u8; size.plaintext_size()];
    rand::thread_rng().fill_bytes(&mut reply);
    let applied = theirs
        .with_key_rotation(SphinxKeyRotation::Unknown)
        .apply_surb(&reply, size, PacketType::Mix)
        .map_err(|e| format!("apply: {e}"))?;
    let packet = applied.into_packet();
    let wire = packet.to_bytes().map_err(|e| format!("ser: {e}"))?;
    let (to, text) = unwrap(&wire, &secrets)?;
    if to != me { return Err("reply did not come home".into()); }
    if text != reply { return Err(format!("reply changed on the way ({} bytes)", text.len())); }
    Ok(())
}

fn message() -> Result<(), String> {
    let (_, nodes) = route(4);
    let blocks: Vec<Vec<u8>> = (0..24)
        .map(|_| surb::surb_bytes(&surb::build_surb(&nodes, &delays(4), &[9u8; 32]).unwrap()))
        .collect();
    let tag = [0x5au8; 16];
    let req = b"socks5 connect request bytes";
    let ours = message::repliable::repliable_data(&tag, &blocks, req);
    // Byte 0 is the outer message type; the reference reads what follows.
    if ours[0] != 1 { return Err("not marked repliable".into()); }
    let parsed = RepliableMessage::try_from_bytes(&ours[1..]).map_err(|e| format!("parse: {e}"))?;
    if parsed.sender_tag.to_bytes() != tag { return Err("sender tag".into()); }
    match parsed.content {
        RepliableMessageContent::DataV2(d) => {
            if d.message != req { return Err("request changed".into()); }
            if d.reply_surbs.len() != 24 { return Err(format!("{} blocks", d.reply_surbs.len())); }
            for s in &d.reply_surbs {
                if s.key_rotation() != SphinxKeyRotation::Unknown { return Err("rotation".into()); }
            }
        }
        other => return Err(format!("parsed as {other:?}")),
    }
    let top = message::repliable::repliable_additional_surbs(&tag, &blocks[..5]);
    let parsed = RepliableMessage::try_from_bytes(&top[1..]).map_err(|e| format!("parse top up: {e}"))?;
    match parsed.content {
        RepliableMessageContent::AdditionalSurbsV2(a) if a.reply_surbs.len() == 5 => {}
        other => return Err(format!("top up parsed as {other:?}")),
    }
    println!("  24 blocks: {} bytes as v2 (each {} bytes)", ours.len(), blocks[0].len());
    Ok(())
}

fn main() {
    let mut ok = true;
    let mut check = |name: &str, r: Result<(), String>| {
        match r {
            Ok(()) => println!("PASS {name}"),
            Err(e) => { ok = false; println!("FAIL {name}: {e}"); }
        }
    };
    for hops in 1..=5 {
        check(&format!("forward packet, {hops} hops, unwrapped by sphinx-packet 0.6.0"), forward(hops));
    }
    for hops in 1..=5 {
        for _ in 0..20 {
            check(&format!("reply block, {hops} hops, used by the reference and peeled home"), reply_block(hops));
        }
    }
    check("repliable message and top up, parsed by nym-sphinx-anonymous-replies 1.22.1", message());
    println!("{}", if ok { "ALL PASS" } else { "FAILURES" });
}
