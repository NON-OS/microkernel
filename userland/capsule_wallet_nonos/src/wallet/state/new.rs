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
        /*
         * NOX out, USDC in: the asset this wallet is for is the one a reader
         * most often pays with.
         */
        /*
         * Half a percent, which is where a pool with real depth sits. A
         * tolerance set high enough to always succeed protects nobody.
         */
        keyring_port: 0,
        owner_pid: 0,
        wallet_id: 0,
        vault_saved: false,
        vault_restore_tried: false,
        vault_silent_since: None,
        address: [0; 20],
        address_ready: false,
        balance_ready: false,
        balance_wei: [0; 32],
        usdc_ready: false,
        usdc_units: [0; 32],
        nonce_ready: false,
        live_nonce: 0,
        fee_ready: false,
        fee_wei: 0,
        view: super::types::VIEW_HOME,
        scroll: 0,
        vault_present: false,
        kept_note: None,
        custody_armed: false,
        recover_shown: false,
        words: super::types::WORDS_UNREAD,
        failure: None,
        proof_count: 0,
        proof_eth_hash: [0; 32],
        proof_eth_len: 0,
        proof_nox_hash: [0; 32],
        proof_nox_len: 0,
        status: b"keyring pending",
        in_count: 0,
        in_kind: 0,
        in_x: 0,
        in_y: 0,
        fee_tier: 1,
        proof_filter: 0,
        usd_mode: false,
        light_mode: false,
        panel: 0,
        locked: false,
        account: 0,
        import_active: false,
        backup_active: false,
        recover_active: false,
        export_active: false,
        custody: super::custody::Custody::new(),
        send: super::send_form::SendForm::new(),
        tx: super::signed_tx::SignedTx::new(),
        broadcast: super::broadcast::BroadcastState::new(),
        net: super::net_view::Network::new(),
        swap: super::swap_form::SwapForm::new(),
        stake: super::stake_form::StakeForm::new(),
        nox: crate::wallet::nox::NoxStatus::empty(),
        action: None,
        view_w: 1280,
        view_h: 0,
        /*
         * Not probed yet, which reads as unavailable until a lookup says
         * otherwise. Absence is the default all the way down.
         */
        shield: crate::wallet::shield::probe::Shield::Unknown,
        shield_ui: super::shield_ui::ShieldUi::default(),
        meter: crate::wallet::meter::MeterUi::default(),
        publisher: crate::wallet::publish::PublisherUi::default(),
        accounts: alloc::vec::Vec::new(),
        /* Mainnet is picked at start, and it has no shield pool. */
        receive_private: false,
        account_open: 0,
        accounts_owed: None,
        accounts_retry_at: 0,
    }
}
