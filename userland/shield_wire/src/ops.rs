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

//! The operations and the statuses.

/// The service name the shield registers and the wallet looks up.
pub const SERVICE: &[u8] = b"nonos.shield";

/// Whether a wallet is stored, unlocked, its nox1 address, balances, where
/// the store is kept (`kept`, memory or volume) and the job the worker is
/// running (`job`), and the account's history from its store: `history=1`
/// then one `entry` value an entry, `kind|coin|amount|stage|tx|at`, oldest
/// first. Answered at once.
pub const OP_STATE: u16 = 1;
/// Open the shield from the wallet's recovery words, or unlock the one
/// stored. Fields: the 0x account, then the words, space separated.
pub const OP_OPEN_WORDS: u16 = 2;
/// Open the shield from a private key, for a wallet imported from one.
/// Fields: the 0x account, then the key in hex.
pub const OP_OPEN_KEY: u16 = 3;
/// Read the pool's whole history and find this wallet's notes. A job.
pub const OP_SYNC: u16 = 4;
/// Review a deposit into the pool. Fields: coin (ETH or NOX), amount. A job.
pub const OP_REVIEW_SHIELD: u16 = 5;
/// Send the transaction a review holds. Fields: the review id. A job.
pub const OP_CONFIRM: u16 = 6;
/// The fee a spend pays now. Fields: coin, amount, withdraw (0 or 1). A job.
pub const OP_QUOTE: u16 = 7;
/// Prove and publish a private payment. Fields: coin, nox1 address,
/// amount, early (0 or 1). A job.
pub const OP_SEND: u16 = 8;
/// Prove and publish a withdrawal. Fields: coin, 0x address, amount,
/// early. A job.
pub const OP_WITHDRAW: u16 = 9;
/// Follow the last spend: landed, waiting, republished, or spent
/// elsewhere, and whether settling it yourself is offered. A job; in its
/// RESULT the follow's own state is `job_state` (`Values::put_job`). Each
/// spend kept before the last is one `earlier` value,
/// `id|state|minutes|self_settle|tx`, oldest first; one that landed is
/// reported once and then forgotten.
pub const OP_FOLLOW: u16 = 10;
/// Review settling the last spend from the public account, or with a
/// field, the earlier one kept under that id. A job.
pub const OP_REVIEW_SELF_SETTLE: u16 = 11;
/// Return to the balance every pending note the chain shows unspent. A job.
pub const OP_TAKE_BACK: u16 = 12;
/// The next unused 0x account of these words, for a withdrawal. Answered
/// at once from what was read after the last job.
pub const OP_FRESH_ADDRESS: u16 = 13;
/// Stop the proof in flight at its next phase.
pub const OP_CANCEL: u16 = 14;
/// What the last job answered, or that it is still running.
pub const OP_RESULT: u16 = 15;
/// Forget the open session and stop a proof in flight; the sealed store
/// stays. Answered at once, whatever the store is doing.
pub const OP_LOCK: u16 = 16;

/// Done, and the values are the answer.
pub const STATUS_OK: i32 = 0;
/// A job started; ask for RESULT.
pub const STATUS_STARTED: i32 = 1;
/// The worker is busy with another job.
pub const STATUS_BUSY: i32 = 2;
/// Refused, with `why` naming the reason in one sentence.
pub const STATUS_REFUSED: i32 = 3;
/// Not a request this service reads.
pub const STATUS_MALFORMED: i32 = -22;
/// Not from the wallet.
pub const STATUS_DENIED: i32 = -13;
