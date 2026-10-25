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

use super::types::State;

pub fn new_state() -> State {
    State {
        vault: super::vault_state::VaultState::default(),
        acct: super::account_set::AccountSet::new(),
        reads: super::chain_reads::ChainReads::default(),
        view: super::types::VIEW_HOME,
        scroll: 0,
        failure: None,
        status: b"keyring pending",
        locked: false,
        keys: super::key_screens::KeyScreens::default(),
        custody: super::custody::Custody::new(),
        send: super::send_form::SendForm::new(),
        tx: super::signed_tx::SignedTx::new(),
        broadcast: super::broadcast::BroadcastState::new(),
        net: super::net_view::Network::new(),
        swap: super::swap_form::SwapForm::new(),
        stake: super::stake_form::StakeForm::new(),
        nox: crate::wallet::nox::NoxStatus::empty(),
        action: None,
        frame: super::frame::Frame::new(),
        /*
         * Not probed yet, which reads as unavailable until a lookup says
         * otherwise. Absence is the default all the way down.
         */
        shield: crate::wallet::shield::probe::Shield::Unknown,
        shield_ui: super::shield_ui::ShieldUi::default(),
        meter: crate::wallet::meter::MeterUi::default(),
        publisher: crate::wallet::publish::PublisherUi::default(),
        arm: crate::wallet::arm::look::Arm::default(),
        arm_lit: false,
    }
}
