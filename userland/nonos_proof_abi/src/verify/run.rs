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

//! The order every verifier checks a container in. Each step
//! refuses with its own reason, and nothing later runs on bytes an earlier
//! step did not accept.

use super::bind::{app_id, measurements, publics};
use super::device_step::device;
use super::pins::Pins;
use super::refusal::Refusal;
use super::verdict::Verdict;
use crate::anchor::anchor;
use crate::container::{parse, tag, Kind};
use crate::record::{ctx, decode_record, record_id};

/// Isolation containers only in v1. A delivery is checked by its buyer
/// against the provider's isolation container; authenticity is the ATTEST
/// trailer's own check at the gate.
pub fn verify(container: &[u8], pins: &Pins) -> Result<Verdict, Refusal> {
    let c = parse(container).ok_or(Refusal::Format)?;
    if c.kind != Kind::Isolation {
        return Err(Refusal::Kind);
    }
    // A section this order does not check would ride under the anchor as if
    // it had been: an isolation container carries none.
    if c.get(tag::ATTEST).is_some() || c.get(tag::DELIVERY).is_some() {
        return Err(Refusal::Format);
    }
    let need = |t| c.get(t).ok_or(Refusal::Format);
    let raw: &[u8; 240] = need(tag::RECORD)?.try_into().map_err(|_| Refusal::Format)?;
    let rec = decode_record(raw).ok_or(Refusal::Record)?;
    let (cx, id) = (ctx(raw), record_id(raw));

    let (device_tag, day) = device(need(tag::DEVICE)?, &cx, pins)?;

    let policy = need(tag::POLICY)?;
    let view = (pins.policy)(policy).ok_or(Refusal::Policy)?;
    measurements(need(tag::MEASUREMENTS)?, &rec, &view.program_blake3)?;

    let app = app_id(policy, &view);
    if rec.app_id != app {
        return Err(Refusal::AppId);
    }

    let words = publics(need(tag::PUBLICS)?, &rec, &view)?;
    nox_verify::verify(pins.isolation, need(tag::ISOLATION)?, &words)
        .map_err(|r| Refusal::Isolation(r.code()))?;

    Ok(Verdict {
        kind: c.kind,
        record_id: id,
        app_id: app,
        tag: device_tag,
        day,
        calls: words[8],
        outside: words[9],
        anchor: anchor(c.kind, &id, &device_tag, container),
    })
}
