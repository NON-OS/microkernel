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

//! verify(container, expected), plan/13's order. Each step refuses with its
//! own reason, and nothing later runs on bytes an earlier step did not accept.

use super::bind::{measurements, publics};
use super::device_step::device;
use super::expected::Expected;
use super::policy_step::policy;
use super::record_step::record;
use super::refusal::Refusal;
use super::verdict::Verdict;
use crate::anchor::anchor;
use crate::container::{parse, tag, Kind};
use crate::record::{ctx, decode_record, record_id};
use crate::scope::market_scope;
use crate::syscall_root::allows;

/// Isolation containers only: a delivery waits for A8, and authenticity is
/// the ATTEST trailer's own check at the gate.
pub fn verify(container: &[u8], ex: &Expected) -> Result<Verdict, Refusal> {
    let c = parse(container).ok_or(Refusal::Format)?;
    if c.kind != Kind::Isolation {
        return Err(Refusal::Kind);
    }
    if !ex.accepts_day(c.window) {
        return Err(Refusal::Window);
    }
    let need = |t| c.get(t).ok_or(Refusal::Format);
    let raw: &[u8; 240] = need(tag::RECORD)?.try_into().map_err(|_| Refusal::Format)?;
    let rec = decode_record(raw).ok_or(Refusal::Record)?;
    let view = (ex.read_policy)(ex.policy).ok_or(Refusal::Policy)?;
    record(&rec, ex.pins, view.mask)?;
    let cx = ctx(c.kind, raw);
    let scope = market_scope(&ex.listing_id, c.window);
    let device_tag = device(need(tag::DEVICE)?, &cx, scope, c.window, ex.pins)?;
    measurements(need(tag::MEASUREMENTS)?, &rec, &view.program_blake3)?;
    policy(need(tag::POLICY)?, &rec, ex)?;
    let words = publics(need(tag::PUBLICS)?, &rec, &view)?;
    nox_verify::verify(ex.pins.isolation, need(tag::ISOLATION)?, &words)
        .map_err(|r| Refusal::Isolation(r.code()))?;
    let id = record_id(raw);
    Ok(Verdict {
        kind: c.kind,
        window: c.window,
        record_id: id,
        app_id: rec.app_id,
        tag: device_tag,
        declared: (0..513).filter(|&b| allows(&view.syscall_set, b)).count() as u32,
        anchor: anchor(c.kind, &ex.listing_id, &id, &device_tag, container),
    })
}
