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

//! A transport's report, made from facts it checked itself. Each transport
//! collects the facts from its own state; the mapping to a report lives here
//! so it is the same mapping the verdict was proven against.

use crate::report::{Network, RouteReport, Stage};

/// How far net.anon's directory bootstrap has got.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnyoneBootstrap {
    Cold,
    Anchored,
    Joining,
    Ready,
}

/// What net.anon knows about its own route at one instant.
#[derive(Clone, Copy, Debug)]
pub struct AnyoneFacts {
    pub bootstrap: AnyoneBootstrap,
    /// The relay set may be used now (directory ready, non-empty, not expired).
    pub usable: bool,
    /// Distinct authorities whose consensus signature verified.
    pub signatures_verified: u8,
    pub signatures_required: u8,
    pub relays: usize,
    /// Seconds until the consensus stops being valid, on the manager's clock.
    pub valid_for_s: u64,
    pub open_circuits: usize,
    /// The newest open circuit: hops it is built over, and hops whose ntor
    /// handshake completed (keyed).
    pub newest_path: usize,
    pub newest_keyed: usize,
}

/// The report for those facts. Ready needs a usable directory and an open
/// circuit; before the directory is joined no directory figure is claimed.
pub fn anyone_report(f: &AnyoneFacts) -> RouteReport {
    let directory = matches!(f.bootstrap, AnyoneBootstrap::Joining | AnyoneBootstrap::Ready);
    let stage = match f.bootstrap {
        AnyoneBootstrap::Cold => Stage::Cold,
        AnyoneBootstrap::Anchored => Stage::Bootstrapping,
        AnyoneBootstrap::Joining => Stage::Joining,
        AnyoneBootstrap::Ready if f.open_circuits > 0 && f.usable => Stage::Ready,
        AnyoneBootstrap::Ready => Stage::Joining,
    };
    let open = f.open_circuits > 0;
    let hops = if open { sat_u8(f.newest_path) } else { 0 };
    let keyed = if open { sat_u8(f.newest_keyed.min(f.newest_path)) } else { 0 };
    RouteReport {
        network: Network::Anyone,
        stage,
        signatures_verified: if directory { f.signatures_verified } else { 0 },
        signatures_required: f.signatures_required,
        nodes: if directory { u32::try_from(f.relays).unwrap_or(u32::MAX) } else { 0 },
        valid_for_ms: if directory { f.valid_for_s.saturating_mul(1_000) } else { 0 },
        routes_open: u16::try_from(f.open_circuits).unwrap_or(u16::MAX),
        hops_authenticated: keyed,
        hops,
        surbs: 0,
        cover_traffic: false,
        reason: 0,
    }
}

fn sat_u8(n: usize) -> u8 {
    u8::try_from(n).unwrap_or(u8::MAX)
}

/// Where net.nym's topology stands, as its own directory status says.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NymDirectory {
    /// No topology held yet.
    Missing,
    /// The clock could not be read, so freshness cannot be judged.
    NoClock,
    /// The topology held is outside its window; a new one is being fetched.
    Expired,
    /// The topology was signed by an authority this build does not trust.
    Untrusted,
    /// Signed by a trusted authority, or carried in the verified image, and in date.
    Ready,
}

/// What net.nym knows about its own route at one instant.
#[derive(Clone, Copy, Debug)]
pub struct NymFacts {
    pub directory: NymDirectory,
    /// Mix nodes and gateways the topology names.
    pub nodes: usize,
    /// Milliseconds until the topology leaves its window.
    pub valid_for_ms: u64,
    /// A gateway is bound and its handshake derived a shared key against the
    /// identity the topology gives it.
    pub gateway_authenticated: bool,
    /// Hops every packet crosses (`topology::ROUTE_HOPS`).
    pub route_hops: usize,
    /// Reply blocks the far end is believed to hold.
    pub surbs: u32,
    /// Cover traffic went out recently.
    pub cover_traffic: bool,
}

/// The reason byte for a topology from an authority this build does not trust.
pub const NYM_REASON_UNTRUSTED: u8 = 1;
/// The reason byte for a clock that could not be read.
pub const NYM_REASON_NO_CLOCK: u8 = 2;

/// The report for those facts. Ready needs a topology in date from a trusted
/// signer and an authenticated gateway; every hop of a packet is sealed to a
/// key the signed topology names, so the hops count as authenticated only
/// then. One signature is what the topology carries and what is required.
pub fn nym_report(f: &NymFacts) -> RouteReport {
    let directory = f.directory == NymDirectory::Ready;
    let (stage, reason) = match f.directory {
        NymDirectory::Missing => (Stage::Cold, 0),
        NymDirectory::NoClock => (Stage::Bootstrapping, NYM_REASON_NO_CLOCK),
        NymDirectory::Expired => (Stage::Bootstrapping, 0),
        NymDirectory::Untrusted => (Stage::Failed, NYM_REASON_UNTRUSTED),
        NymDirectory::Ready if f.gateway_authenticated => (Stage::Ready, 0),
        NymDirectory::Ready => (Stage::Joining, 0),
    };
    let up = directory && f.gateway_authenticated;
    let hops = sat_u8(f.route_hops);
    RouteReport {
        network: Network::Nym,
        stage,
        signatures_verified: u8::from(directory),
        signatures_required: 1,
        nodes: if directory { u32::try_from(f.nodes).unwrap_or(u32::MAX) } else { 0 },
        valid_for_ms: if directory { f.valid_for_ms } else { 0 },
        routes_open: u16::from(up),
        hops_authenticated: if up { hops } else { 0 },
        hops: if up { hops } else { 0 },
        surbs: u16::try_from(f.surbs).unwrap_or(u16::MAX),
        cover_traffic: f.cover_traffic,
        reason,
    }
}
