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
    /// What the chain last said of this account (`chain_reads`).
    pub reads: super::chain_reads::ChainReads,
    pub view: u8,
    /// How far an Etna screen has scrolled, reset when the view changes.
    pub scroll: u32,
    /// What an Etna screen refused, in one sentence, until dismissed.
    pub failure: Option<&'static str>,
    pub status: &'static [u8],
    /// Whether the wallet is locked.
    pub locked: bool,
    /// Which key screen is open (`key_screens`).
    pub keys: super::key_screens::KeyScreens,
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
    /// The framebuffer size at the last paint (`frame`).
    pub frame: super::frame::Frame,
    /// Whether a shield capsule answered the service probe this session.
    ///
    /// Starts `Unknown` and is only ever set from a real lookup, so the
    /// shielded screens cannot enable themselves by default.
    pub shield: crate::wallet::shield::probe::Shield,
    /* The Shield screens: which is up and what was picked on it. */
    pub shield_ui: super::shield_ui::ShieldUi,
    pub meter: crate::wallet::meter::MeterUi,
    pub publisher: crate::wallet::publish::PublisherUi,
    /// The guard on a screen whose presses move value or sign (`arm`).
    pub arm: crate::wallet::arm::look::Arm,
    /// Whether the frame last shown drew that guard armed (`arm::tick`).
    pub arm_lit: bool,
}
