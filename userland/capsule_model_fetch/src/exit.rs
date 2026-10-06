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

/*
 * What a run ends in, as its exit status. The Terminal shows a nonzero one
 * as a failure. The Linux installer starts this for the Qwen tier a person
 * installs from the store and reads the reason in it, so the store can say
 * it in words; it takes these numbers from this file by #[path], so the two
 * sides never disagree. Pure, so model_fetch_proofs holds the mapping.
 */

/* Every file of every tier named is on the data volume, verified. */
pub const DONE: i32 = 0;
/*
 * A file refused for a reason not named below: no mirror served it, or the
 * kernel refused the stream otherwise. What came is kept for the next run.
 */
pub const REFUSED: i32 = 1;
/* Usage, or a word that is no tier of the signed catalogue. */
pub const USAGE: i32 = 2;
/* No signed catalogue is built in, or the one built in did not check. */
pub const NO_CATALOGUE: i32 = 3;
/* The network a download must leave through is not running. */
pub const NO_NETWORK: i32 = 4;
/* No data volume to feed: no disk carries NONOS at all (ENODEV). */
pub const NO_VOLUME: i32 = 5;
/* The data volume is locked by its passphrase. */
pub const LOCKED: i32 = 6;
/*
 * The bytes were not the signed pin, or the name holds a file no record of
 * the pin vouches for; the kernel kept and linked none of them.
 */
pub const MISMATCH: i32 = 7;
/* The data volume has no room for what is still to come. */
pub const NO_ROOM: i32 = 8;
/* Another download holds the data volume's one stream. */
pub const BUSY: i32 = 9;
/*
 * A file's name is too long for the volume to keep a mark beside it, so it
 * cannot be fetched on this system at all; trying again changes nothing.
 */
pub const UNKEPT: i32 = 10;
/*
 * The tier needs more memory to run than this machine has in all, so it
 * was not downloaded; a smaller tier, or more memory, is the way on.
 */
pub const TOO_LITTLE_MEMORY: i32 = 11;
/*
 * The data volume could not be reached: the disk failed (EIO) or is not
 * ready yet (EAGAIN). Unlike NO_VOLUME, asking again may work.
 */
pub const VOLUME_FAILED: i32 = 12;
/*
 * Too little free memory to hold a volume at all (ENOMEM): a live session
 * keeps its volume in memory. Asking again does not make memory.
 */
pub const NO_MEMORY: i32 = 13;
/*
 * The chosen network runs, and no connection through it reached a mirror
 * before a byte came this run: through Nym, through Anyone, or directly.
 * Not a download that started and stopped (REFUSED): the network is not
 * reachable from this machine, which a retry alone may not mend.
 */
pub const NYM_UNREACHABLE: i32 = 14;
pub const ANYONE_UNREACHABLE: i32 = 15;
pub const MIRROR_UNREACHABLE: i32 = 16;
/*
 * Through Nym, or Anyone, connections opened or the network had a session,
 * but every try in a row (get/retry.rs: up to six, over at most three
 * minutes, each a new connection) found no exit that answered. Distinct
 * from the network being unreachable from the start: another try may find
 * a working exit, and a direct download is the person's other choice.
 */
pub const NYM_NO_EXIT: i32 = 17;
pub const ANYONE_NO_EXIT: i32 = 18;
/*
 * The Anyone network, which downloads installs, did not build a circuit in
 * the three minutes a download waits for it (get/anyone_wait.rs). Retry, or
 * a direct download, which the person chooses.
 */
pub const ANYONE_NOT_UP: i32 = 19;

const EIO: i64 = -5;
const EAGAIN: i64 = -11;
const ENOMEM: i64 = -12;
const EACCES: i64 = -13;
const EBUSY: i64 = -16;
const EEXIST: i64 = -17;
const ENODEV: i64 = -19;
const ENOSPC: i64 = -28;
const EBADMSG: i64 = -74;

/*
 * The status a kernel refusal of the data volume stands for. ENODEV is no
 * disk carrying NONOS at all, which no retry mends. A live session's volume
 * is held in memory: ENOSPC is that memory filling, which freeing memory
 * gets past, and ENOMEM is too little free to hold a volume at all. EIO and
 * EAGAIN are a volume that failed or a disk not ready yet, which a retry
 * may get past; a machine with no disk the block layer could choose still
 * answers EIO today, so the words for VOLUME_FAILED name that too.
 */
pub const fn of_errno(rc: i64) -> i32 {
    match rc {
        ENODEV => NO_VOLUME,
        EIO | EAGAIN => VOLUME_FAILED,
        EACCES => LOCKED,
        ENOSPC => NO_ROOM,
        ENOMEM => NO_MEMORY,
        EBADMSG | EEXIST => MISMATCH,
        EBUSY => BUSY,
        _ => REFUSED,
    }
}
