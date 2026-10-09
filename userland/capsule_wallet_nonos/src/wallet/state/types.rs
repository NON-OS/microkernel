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

use crate::wallet::net::NetStatus;
use alloc::vec::Vec;

pub const MAX_RAILS: usize = 8;
pub const VIEW_HOME: u8 = 0;
pub const VIEW_RECEIVE: u8 = 1;
pub const VIEW_SEND: u8 = 2;
// New hardened-wallet + private-swap + NOX screens.
pub const VIEW_SHIELD: u8 = 6;
pub const VIEW_NOX: u8 = 9;
pub const VIEW_SWAP: u8 = 10;
pub const VIEW_SETTINGS: u8 = 11;
pub const VIEW_IMPORT: u8 = 12;
pub const VIEW_RECOVER: u8 = 13;
pub const VIEW_EXPORT: u8 = 14;
pub const VIEW_ACCOUNTS: u8 = 15;
/* Whether the keyring holds account 0's recovery words this session. */
pub const WORDS_UNREAD: u8 = 0;
pub const WORDS_HELD: u8 = 1;
/* The wallet came from a private key: there are no words. */
pub const WORDS_NONE: u8 = 2;
pub const SEND_FIELD_TO: u8 = 0;
pub const SEND_FIELD_AMOUNT: u8 = 1;

#[derive(Clone, Copy)]
pub struct Rail {
    pub symbol: [u8; 8],
    pub symbol_len: u8,
    pub family: u8,
    pub status: u16,
    pub flags: u32,
    pub chain_id: u64,
    pub contract: [u8; 20],
}

pub struct State {
    /// Which token the trade pays out of, as an index into the token list.
    pub swap_from: u8,
    /// Which token the trade buys.
    pub swap_to: u8,
    /// Amount to pay, in the paying token's smallest unit.
    pub swap_in: u128,
    /// What the pool last said this trade returns.
    pub swap_quote: crate::wallet::swap::Quote,
    /// Slippage the reader will accept, in hundredths of a percent.
    pub swap_slippage_bps: u32,
    /// Zero while the router still needs an allowance, one once it has one.
    pub swap_step: u8,
    /// How many digits the reader has typed, so a correction knows what to
    /// take back.
    pub swap_digits: u32,
    /// Digits typed after the point.
    pub swap_places: u32,
    /// Whether the reader has started a fraction.
    pub swap_point: bool,
    pub keyring_port: u32,
    pub owner_pid: u32,
    pub wallet_id: u32,
    /// Whether this wallet is on the disk sealed to this machine. False on a
    /// machine with no TPM, and the status line says so once.
    pub vault_saved: bool,
    /// One attempt per window. A vault that will not open must not be retried
    /// on every hydrate, which would put a TPM derivation on a timer.
    pub vault_restore_tried: bool,
    /// The uptime the vault's store first went unanswered, while it has
    /// not answered since: past the patience a new wallet is asked twice
    /// rather than refused (`event::replace_rule`).
    pub vault_silent_since: Option<i64>,
    /// A vault is on the disk that this boot could not open or name: a new
    /// wallet over it is asked twice (`event::may_replace`).
    pub vault_present: bool,
    /// What the last keep came to, when it did not keep the wallet across
    /// reboots: drawn on the phrase screen and on home, where the person
    /// decides whether to write the words down (`event::keep_plan`).
    pub kept_note: Option<&'static str>,
    /// The first press of a replace was made and said; the next goes on.
    pub custody_armed: bool,
    /// The recovery words being typed are drawn, not masked.
    pub recover_shown: bool,
    pub address: [u8; 20],
    pub address_ready: bool,
    pub balance_ready: bool,
    pub balance_wei: [u8; 32],
    /// USDC held on the picked network, in its six-place units.
    pub usdc_ready: bool,
    pub usdc_units: [u8; 32],
    pub nonce_ready: bool,
    pub live_nonce: u64,
    pub fee_ready: bool,
    pub fee_wei: u64,
    pub view: u8,
    /// How far an Etna screen has scrolled, reset when the view changes.
    pub scroll: u32,
    pub send_focus: u8,
    /// Form, review or sent (`send::STAGE_*`).
    pub send_stage: u8,
    /// The payment the review shows and the confirm signs.
    pub send_draft: Option<crate::wallet::send::Draft>,
    /// What an Etna screen refused, in one sentence, until dismissed.
    pub failure: Option<&'static str>,
    pub send_to_hex: [u8; 40],
    pub send_to_len: usize,
    /// What to send, typed at chain precision on the shared keypad. It replaced
    /// a `u32` of thousandths of an ether, which could not express a figure
    /// smaller than 0.001 and had no decimal point at all.
    pub send_amount: crate::wallet::num::Amount,
    pub send_nonce: u64,
    pub tx_hash: [u8; 32],
    /// The signed transaction's nonce, the most ETH it can take, and the
    /// token it moves, for the ledger once it goes.
    pub tx_nonce: u64,
    /// The tip and the cap per gas the last review read from the network's
    /// recent blocks (`send::fees`), or None when it did not come.
    pub offered_fees: Option<(u128, u128)>,
    /// The send form's amount is everything held, the fee left out of an
    /// ETH payment at the fees the review reads.
    pub send_all: bool,
    /// The last whole read of mainnet (0) and Sepolia (1), for the screens.
    pub last_read: [Option<crate::wallet::net::last_read::LastRead>; 2],
    pub tx_cost: u128,
    pub tx_token: Option<(u8, u128)>,
    /// Every transaction sent and not yet settled, per network and account,
    /// kept across a switch (`send::sent`).
    pub sent: crate::wallet::send::sent::Ledger,
    pub tx_len: u32,
    pub tx_raw: Vec<u8>,
    pub tx_ready: bool,
    pub tx_kind: &'static [u8],
    /// A signed transaction waiting for its confirming second press. Sending is
    /// the one act in this window that cannot be undone, so it is armed rather
    /// than fired: the same shape the process manager uses before it ends a
    /// process, which is a far more recoverable thing to do.
    pub broadcast_armed: bool,
    pub broadcast_ready: bool,
    pub broadcast_hash: [u8; 32],
    pub receipt_ready: bool,
    /// The broadcast broke once the transaction had begun to go, so whether
    /// the node took it is not known; it is never sent again, and its
    /// receipt is followed all the same.
    pub broadcast_unknown: bool,
    pub receipt_ok: bool,
    pub proof_count: u8,
    pub proof_eth_hash: [u8; 32],
    pub proof_eth_len: u32,
    pub proof_nox_hash: [u8; 32],
    pub proof_nox_len: u32,
    pub net: NetStatus,
    pub rails: [Rail; MAX_RAILS],
    pub rail_count: usize,
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
     * UI selections driven by pointer clicks: fee tier (0..2), stake/unstake
     * mode (0/1), proof filter (0..2), and the amount ETH/USD toggle.
     */
    pub fee_tier: u8,
    pub stake_unstake: u8,
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
     * NOX to stake, in wei. Held at chain precision rather than whole tokens
     * so any amount can be typed, fractions included, with no ceiling beyond
     * what the wallet actually holds.
     */
    pub stake_amount: u128,
    /*
     * Decimal entry for the amount above: digits typed, places after the
     * point, and whether a point has been started.
     */
    pub stake_digits: u32,
    pub stake_places: u32,
    pub stake_point: bool,
    /*
     * Which staked position the Unstake tab acts on. The contract closes a
     * position by index, not by amount.
     */
    pub stake_position: u64,
    /*
     * Chosen lock term, as an index into the contract lock table. Zero is no
     * lock, which is what plain stake() does.
     */
    pub stake_lock: u8,
    /*
     * Two-step staking: 0 = needs the approve, 1 = ready to stake. Advances once
     * the approve broadcasts and resets after the stake.
     */
    pub stake_step: u8,
    /*
     * The staking transaction the stake screen reviews, made from fresh
     * reads; Confirm signs exactly this. None while the form is up.
     */
    pub stake_draft: Option<crate::wallet::send::Draft>,
    /* What the receipt followed now is for, and when it went. */
    pub follow_purpose: crate::wallet::act::Purpose,
    pub sent_at_ms: i64,
    /*
     * Which asset the send screen transfers: 0 = ETH, 1 = NOX.
     */
    pub send_token: u8,
    /*
     * Private-key import entry. The typed hex never renders and is wiped the
     * moment the key is handed to the keyring or the field is cancelled.
     */
    pub import_active: bool,
    pub import_hex: [u8; 64],
    pub import_len: usize,
    /*
     * One-time mnemonic backup: the word indices exist here only while the
     * backup screen is showing and are volatile-wiped the moment the user
     * confirms. They are never persisted, logged, or kept past that screen.
     */
    pub backup_active: bool,
    pub backup_words: [u16; 24],
    pub backup_count: u8,
    /*
     * Recovery-phrase entry: typed words, space separated, shown while typing
     * so the user can check them, wiped on submit or cancel.
     */
    pub recover_active: bool,
    pub recover_buf: [u8; 240],
    pub recover_len: usize,
    /*
     * Private-key reveal for backup/export. Held only while shown, wiped the
     * moment it is hidden. `export_hex` is the 0x-prefixed key when revealed.
     */
    pub export_active: bool,
    pub export_hex: [u8; 66],
    /*
     * Live NOX token and staking readout from mainnet eth_call.
     */
    pub nox: crate::wallet::nox::NoxStatus,
    /*
     * One when a refresh is wanted at the next idle tick rather than at the
     * next interval: after a wallet arrives, or the network changes.
     */
    pub probe_step: u8,
    /*
     * The refresh under way, stepped a slice per tick so the window never
     * waits on the network. None between refreshes.
     */
    pub net_job: Option<crate::wallet::net::step::Job>,
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
    /* The accounts of this phrase in use, account 0 first, and which one is
     * open. Empty until a wallet exists. */
    pub accounts: alloc::vec::Vec<crate::wallet::accounts::Account>,
    /* Receive shows the private nox1 address, else the public 0x one. */
    pub receive_private: bool,
    pub account_open: u8,
    /* The account list read at boot, while some of its accounts are not
     * derived yet: how many, and which was open. */
    pub accounts_owed: Option<(u8, u8)>,
    /* When the owed accounts are asked for again (uptime ms). */
    pub accounts_retry_at: i64,
    /* WORDS_*: the shield opens from the words, and from the key only for
     * a wallet known to have come from one. */
    pub words: u8,
    /* The Swap screen's no-price banner, dismissed until the amount changes. */
    pub swap_note_hidden: bool,
}
