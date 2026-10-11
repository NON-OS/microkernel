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

/* A meter job the wallet started (v1.4.14), and its result in words. */

use alloc::format;
use alloc::string::String;

use nonos_payment_proto::{EAGAIN, EBUSY, EGAS, ENOENT, ETIME};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum JobKind {
    TopUp,
    Close,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MeterJob {
    pub id: u32,
    pub kind: JobKind,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Outcome {
    Running,
    Sent(String),
    Failed(&'static str),
}

fn short(h: &[u8]) -> String {
    let hex: String = h.iter().take(4).map(|b| format!("{b:02x}")).collect();
    format!("0x{hex}…")
}

pub fn outcome(kind: JobKind, r: Result<&[u8], i32>) -> Outcome {
    match (kind, r) {
        (_, Err(EAGAIN)) => Outcome::Running,
        (JobKind::TopUp, Ok(b)) if b.len() == 64 => Outcome::Sent(format!(
            "top-up sent: approve {}, topUp {}",
            short(&b[..32]),
            short(&b[32..])
        )),
        (JobKind::Close, Ok(b)) if b.len() == 32 => {
            Outcome::Sent(format!("close sent: {}", short(b)))
        }
        (_, Ok(_)) => Outcome::Failed("The meter's answer could not be read; check the channel."),
        (_, Err(ENOENT)) => Outcome::Failed(
            "The meter restarted, so this job's answer is gone. Refresh: the channel shows \
             whether it went through.",
        ),
        (_, Err(EBUSY)) => Outcome::Failed("The meter is busy with another job; try again soon."),
        (_, Err(EGAS)) => {
            Outcome::Failed("The meter's ETH is short of gas, or the fees are above its caps.")
        }
        (JobKind::Close, Err(ETIME)) => Outcome::Failed(
            "This channel cannot be closed before its expiry plus the contract's grace.",
        ),
        (_, Err(_)) => Outcome::Failed("The meter could not send it."),
    }
}
