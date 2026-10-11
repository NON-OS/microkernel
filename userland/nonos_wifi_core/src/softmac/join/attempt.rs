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

//! One run of the state machine under one policy, with fresh randomness, and
//! the rule for running it again with WPA2.

use super::drive::drive;
use super::report::{Outcome, Report};
use super::request::ConnectRequest;
use super::run::JoinError;
use crate::mlme::{Entropy, JoinRequest, Mlme, MlmeFailure, SAE_ENTROPY};
use crate::regdom::Domain;
use crate::rsn::JoinPolicy;
use crate::sae::SaeFailure;
use crate::softmac::radio::{Env, Radio};
use crate::wpa::akm::Akm;

/// Each attempt's budget: long enough for a slow router's SAE.
pub const JOIN_BUDGET_MS: u64 = 8_000;

pub(super) fn attempt<R: Radio, E: Env>(
    r: &mut R,
    env: &mut E,
    domain: Domain,
    req: &ConnectRequest,
    policy: JoinPolicy,
    beacon: &[u8],
) -> Result<Report, JoinError> {
    let mut entropy = Entropy { snonce: [0u8; 32], sae: [0u8; SAE_ENTROPY] };
    if !env.random(&mut entropy.snonce) || !env.random(&mut entropy.sae) {
        return Err(JoinError::NoEntropy);
    }
    let our_mac = r.mac();
    let mut m =
        Mlme::join(&JoinRequest { our_mac, ssid: req.ssid, passphrase: req.pass, policy, entropy });
    m.set_domain(domain);
    Ok(drive(r, env, m, beacon, JOIN_BUDGET_MS))
}

pub(super) fn retry_with_psk(report: &Report, wpa3_only: bool) -> bool {
    let ran_sae = report.akm == Some(Akm::Sae);
    let wrong_password = matches!(report.failure, Some(MlmeFailure::Sae(SaeFailure::BadConfirm)));
    !wpa3_only && ran_sae && !wrong_password && !matches!(report.outcome, Outcome::Joined { .. })
}
