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

//! verify_delivery(container, expected), the plan's order. Each step refuses
//! with its own reason, and nothing later runs on bytes an earlier step did
//! not accept. Only kind 2: a kind-1 container is refused here, and a
//! delivery never passes the kind-1 verify (its record is not final, and
//! ctx2 is not ctx).

use super::deliver_expected::{DeliveryExpected, DeliveryVerdict};
use super::deliver_steps::{body, live_record, measurements};
use super::device_step::device;
use super::expected::accepts_day;
use super::refusal::Refusal;
use crate::container::{parse, tag, Kind};
use crate::delivery::{ctx2, Delivery};
use crate::record::{decode_record, record_id};
use crate::scope::delivery_scope;

pub fn verify_delivery(
    container: &[u8],
    ex: &DeliveryExpected,
) -> Result<DeliveryVerdict, Refusal> {
    let c = parse(container).ok_or(Refusal::Format)?;
    if c.kind != Kind::Delivery {
        return Err(Refusal::Kind);
    }
    if !accepts_day(ex.days, c.window) {
        return Err(Refusal::Window);
    }
    let need = |t| c.get(t).ok_or(Refusal::Format);
    let raw: &[u8; 240] = need(tag::RECORD)?.try_into().map_err(|_| Refusal::Format)?;
    let rec = decode_record(raw).ok_or(Refusal::Record)?;
    let view = (ex.read_policy)(ex.policy).ok_or(Refusal::Policy)?;
    live_record(&rec, ex, view.mask)?;
    let body_raw: &[u8; 208] = need(tag::DELIVERY)?.try_into().map_err(|_| Refusal::Format)?;
    let b = Delivery::decode(body_raw).ok_or(Refusal::Format)?;
    let id = record_id(raw);
    body(&b, &rec, &id, ex)?;
    let scope = delivery_scope(&ex.listing_id, c.window);
    let tag = device(need(tag::DEVICE)?, &ctx2(raw, body_raw), scope, c.window, ex.pins)?;
    measurements(need(tag::MEASUREMENTS)?, &rec)?;
    let (from, to, service_meas) = (b.from, b.to, b.service_meas);
    Ok(DeliveryVerdict { window: c.window, record_id: id, service_meas, tag, from, to })
}
