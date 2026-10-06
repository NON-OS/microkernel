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

use super::{
    empty_rail::empty_rail,
    types::{State, MAX_RAILS},
};
pub fn new_state() -> State {
    State {
        /*
         * NOX out, USDC in: the asset this wallet is for is the one a reader
         * most often pays with.
         */
        swap_from: 0,
        swap_to: 2,
        swap_in: 0,
        swap_quote: crate::wallet::swap::Quote::default(),
        /*
         * Half a percent, which is where a pool with real depth sits. A
         * tolerance set high enough to always succeed protects nobody.
         */
        swap_slippage_bps: 50,
        swap_step: 0,
        swap_digits: 0,
        swap_places: 0,
        swap_point: false,
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
        send_focus: super::types::SEND_FIELD_TO,
        send_stage: 0,
        send_draft: None,
        stake_draft: None,
        vault_present: false,
        kept_note: None,
        custody_armed: false,
        recover_shown: false,
        words: super::types::WORDS_UNREAD,
        follow_purpose: crate::wallet::act::Purpose::Pay,
        sent_at_ms: 0,
        failure: None,
        send_to_hex: [0; 40],
        send_to_len: 0,
        send_amount: crate::wallet::num::Amount::new(),
        send_nonce: 0,
        tx_hash: [0; 32],
        tx_nonce: 0,
        offered_fees: None,
        send_all: false,
        last_read: [None, None],
        tx_cost: 0,
        tx_token: None,
        sent: crate::wallet::send::sent::Ledger::default(),
        tx_len: 0,
        tx_raw: alloc::vec::Vec::new(),
        tx_ready: false,
        tx_kind: b"none",
        broadcast_armed: false,
        broadcast_ready: false,
        broadcast_hash: [0; 32],
        receipt_ready: false,
        broadcast_unknown: false,
        receipt_ok: false,
        proof_count: 0,
        proof_eth_hash: [0; 32],
        proof_eth_len: 0,
        proof_nox_hash: [0; 32],
        proof_nox_len: 0,
        net: super::default_net(),
        rails: [empty_rail(); MAX_RAILS],
        rail_count: 0,
        status: b"keyring pending",
        in_count: 0,
        in_kind: 0,
        in_x: 0,
        in_y: 0,
        fee_tier: 1,
        stake_unstake: 0,
        proof_filter: 0,
        usd_mode: false,
        light_mode: false,
        panel: 0,
        locked: false,
        account: 0,
        stake_amount: 0,
        stake_digits: 0,
        stake_places: 0,
        stake_point: false,
        stake_position: 0,
        stake_lock: 0,
        stake_step: 0,
        send_token: 0,
        import_active: false,
        import_hex: [0; 64],
        import_len: 0,
        backup_active: false,
        backup_words: [0; 24],
        backup_count: 0,
        recover_active: false,
        recover_buf: [0; 240],
        recover_len: 0,
        export_active: false,
        export_hex: [0; 66],
        nox: crate::wallet::nox::NoxStatus::empty(),
        probe_step: 0,
        net_job: None,
        action: None,
        view_w: 1280,
        view_h: 0,
        /*
         * Not probed yet, which reads as unavailable until a lookup says
         * otherwise. Absence is the default all the way down.
         */
        shield: crate::wallet::shield::probe::Shield::Unknown,
        shield_ui: super::shield_ui::ShieldUi::default(),
        accounts: alloc::vec::Vec::new(),
        /* Mainnet is picked at start, and it has no shield pool. */
        receive_private: false,
        account_open: 0,
        accounts_owed: None,
        accounts_retry_at: 0,
        swap_note_hidden: false,
    }
}
