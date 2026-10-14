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

pub use super::rail::Rail;
pub use super::views::*;

pub struct State {
    /// The keyring and the vault (`vault_state`).
    pub vault: super::vault_state::VaultState,
    /// This phrase's accounts and address (`account_set`).
    pub acct: super::account_set::AccountSet,
    /// The first press of a replace was made and said; the next goes on.
    pub custody_armed: bool,
    /// The recovery words being typed are drawn, not masked.
    pub recover_shown: bool,
    /// What the chain last said of this account (`chain_reads`).
    pub reads: super::chain_reads::ChainReads,
    pub view: u8,
    /// How far an Etna screen has scrolled, reset when the view changes.
    pub scroll: u32,
    /// What an Etna screen refused, in one sentence, until dismissed.
    pub failure: Option<&'static str>,
    pub proof_count: u8,
    pub proof_eth_hash: [u8; 32],
    pub proof_eth_len: u32,
    pub proof_nox_hash: [u8; 32],
    pub proof_nox_len: u32,
    pub status: &'static [u8],
    /*
     * Live input readout, refreshed on each discrete key/button event so the
     * running UI can show whether pointer clicks actually reach the capsule.
     */
    pub in_count: u32,
    pub in_kind: u32,
    pub in_x: i32,
    pub in_y: i32,
    /*
     * UI selections driven by pointer clicks: fee tier (0..2), proof filter
     * (0..2), and the amount ETH/USD toggle. Stake or unstake is `stake.unstake`.
     */
    pub fee_tier: u8,
    pub proof_filter: u8,
    pub usd_mode: bool,
    pub light_mode: bool,
    /*
     * Header controls: which dropdown/overlay is open (0 none, 1 command,
     * 2 messages, 3 account) and whether the wallet is locked.
     */
    pub panel: u8,
    pub locked: bool,
    pub account: u8,
    /*
     * Whether the import, the one-time backup, the restore and the export
     * screens are open. What they hold is in `custody`.
     */
    pub import_active: bool,
    pub backup_active: bool,
    pub recover_active: bool,
    pub export_active: bool,
    /// The typed and shown secrets, each in a Zeroizing buffer.
    pub custody: super::custody::Custody,
    /// The send form, the transaction it signed, and that transaction's way
    /// out: `send_form`, `signed_tx` and `broadcast`.
    pub send: super::send_form::SendForm,
    pub tx: super::signed_tx::SignedTx,
    pub broadcast: super::broadcast::BroadcastState,
    /// The route, its rails and the refresh under way (`net_view`).
    pub net: super::net_view::Network,
    /// The swap form (`swap_form`).
    pub swap: super::swap_form::SwapForm,
    /// The stake form (`stake_form`).
    pub stake: super::stake_form::StakeForm,
    /*
     * Live NOX token and staking readout from mainnet eth_call.
     */
    pub nox: crate::wallet::nox::NoxStatus,
    /*
     * The network work a press started, stepped like the refresh, which
     * waits while it runs. None when no press is being answered.
     */
    pub action: Option<crate::wallet::act::Action>,
    /*
     * The framebuffer width recorded on the last paint, so pointer handlers can
     * hit-test the same width-relative layout the screens draw.
     */
    pub view_w: u32,
    /*
     * Framebuffer height recorded on the last paint, so the pointer handlers
     * hit-test the same height-relative layout the screens draw.
     */
    pub view_h: u32,
    /// Whether a shield capsule answered the service probe this session.
    ///
    /// Starts `Unknown` and is only ever set from a real lookup, so the
    /// shielded screens cannot enable themselves by default.
    pub shield: crate::wallet::shield::probe::Shield,
    /* The Shield screens: which is up and what was picked on it. */
    pub shield_ui: super::shield_ui::ShieldUi,
    pub meter: crate::wallet::meter::MeterUi,
    pub publisher: crate::wallet::publish::PublisherUi,
}
