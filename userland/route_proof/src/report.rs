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

//! The report and its fixed wire form.

/// Bumped on any change to the layout below; a reader refuses any other.
pub const REPORT_VERSION: u8 = 1;
/// Bytes on the wire. Fixed, so a short or long report is refused outright.
pub const REPORT_LEN: usize = 40;

/// Which network the report is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Network {
    Nym = 1,
    Anyone = 2,
}

/// How far the transport has got. Only `Ready` means traffic can flow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// Nothing fetched yet.
    Cold = 0,
    /// The directory is being fetched or checked.
    Bootstrapping = 1,
    /// Directory accepted, the route itself not yet up.
    Joining = 2,
    /// Carrying traffic.
    Ready = 3,
    /// Stopped on an error the transport will not retry past by itself.
    Failed = 4,
}

/// One transport's account of itself at one instant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteReport {
    pub network: Network,
    pub stage: Stage,
    /// Directory authorities whose signature over the current directory the
    /// transport verified against its built-in keys.
    pub signatures_verified: u8,
    /// How many it requires before it trusts a directory at all.
    pub signatures_required: u8,
    /// Relays (Anyone) or mix nodes (Nym) the verified directory names.
    pub nodes: u32,
    /// How long the directory stays valid from the moment of the report, in
    /// milliseconds. Zero once it has expired.
    pub valid_for_ms: u64,
    /// Anyone: open circuits. Nym: routes ready to send over (0 or 1).
    pub routes_open: u16,
    /// Hops on the open route that proved who they are: a ntor handshake each
    /// for Anyone, the gateway handshake plus layered Sphinx keys for Nym.
    pub hops_authenticated: u8,
    /// Hops the route is built with (three for both networks).
    pub hops: u8,
    /// Nym: reply blocks in stock. Anyone: 0.
    pub surbs: u16,
    /// Nym: cover traffic is being sent. Anyone: 0.
    pub cover_traffic: bool,
    /// Why the transport is not ready, when it says (transport specific, 0 none).
    pub reason: u8,
}

/// The report's 40 bytes.
pub fn encode(r: &RouteReport) -> [u8; REPORT_LEN] {
    let mut b = [0u8; REPORT_LEN];
    b[0] = REPORT_VERSION;
    b[1] = r.network as u8;
    b[2] = r.stage as u8;
    b[3] = r.signatures_verified;
    b[4] = r.signatures_required;
    b[5] = r.hops_authenticated;
    b[6] = r.hops;
    b[7] = r.cover_traffic as u8;
    b[8..12].copy_from_slice(&r.nodes.to_le_bytes());
    b[12..20].copy_from_slice(&r.valid_for_ms.to_le_bytes());
    b[20..22].copy_from_slice(&r.routes_open.to_le_bytes());
    b[22..24].copy_from_slice(&r.surbs.to_le_bytes());
    b[24] = r.reason;
    b
}

/// The report in `b`, or `None` for anything that is not exactly one well
/// formed report: a wrong length, an unknown version, network or stage, a
/// boolean that is not 0 or 1, more authenticated hops than hops, or any
/// reserved byte set.
pub fn decode(b: &[u8]) -> Option<RouteReport> {
    if b.len() != REPORT_LEN || b[0] != REPORT_VERSION {
        return None;
    }
    let network = match b[1] {
        1 => Network::Nym,
        2 => Network::Anyone,
        _ => return None,
    };
    let stage = match b[2] {
        0 => Stage::Cold,
        1 => Stage::Bootstrapping,
        2 => Stage::Joining,
        3 => Stage::Ready,
        4 => Stage::Failed,
        _ => return None,
    };
    let cover_traffic = match b[7] {
        0 => false,
        1 => true,
        _ => return None,
    };
    if b[5] > b[6] || b[25..].iter().any(|&x| x != 0) {
        return None;
    }
    Some(RouteReport {
        network,
        stage,
        signatures_verified: b[3],
        signatures_required: b[4],
        hops_authenticated: b[5],
        hops: b[6],
        cover_traffic,
        nodes: u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
        valid_for_ms: u64::from_le_bytes([
            b[12], b[13], b[14], b[15], b[16], b[17], b[18], b[19],
        ]),
        routes_open: u16::from_le_bytes([b[20], b[21]]),
        surbs: u16::from_le_bytes([b[22], b[23]]),
        reason: b[24],
    })
}
