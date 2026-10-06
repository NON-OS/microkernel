# Note storage

The append-only log of sealed rows that holds the notes of the wallet, and what an attacker with the
file can learn or change. What is checked and what is not is in [20-security-status.md](../../../docs/20-security-status.md).

## What this owns

The note store: an append-only log of sealed rows, and the state the rows fold into. It owns the
row format, the nonce discipline, the balance arithmetic, and the rule that a spent note stops
counting.

A row is one of four kinds (`row/mod.rs`): a note found, a change of status, the position the scan
reached, and a deposit sent and not yet seen stored. A note has three statuses: unspent, pending
and spent. A kind or a status this build does not know is refused and never skipped, because
reading a newer store as though its unknown rows were absent would make a spent note spendable
again.

The balance is per asset (`query.rs`): each coin counts only notes of its own asset id, in its own
note units, and notes of an unknown asset count nothing. Unspent notes are spendable, pending notes
are in flight, and spent notes are neither. The sums saturate and never wrap.

## What it trusts

The seed, for the key every row is sealed under: BLAKE3 in derive-key mode with the context
`nox-shield 2026 note store key v1`, and the associated data `nox-shield/note-store/v1`. Nothing
else: a row that does not authenticate stops the replay.

The file system, for appends and for `sync_data` meaning what it says. A store on a file system that
loses acknowledged writes can lose its last row, which is the failure this format is built to
survive: a half-written last row stops the replay at that row with the rest intact.

`nonos_seal`, for never repeating a nonce under one key. The counter rides in the header of each row
(`frame.rs`: a 4-byte length, then the 8-byte counter), so a reload continues the sequence rather
than guessing it from the size of the file. A counter that would wrap is refused.

## What it hands its neighbours

To `wallet`: a `StoreState`, which answers what is held, what each coin balances to, which deposits
are pending, and where the scan reached. The only way to change it is to append a row, and
`Session::record` writes the row to disk before it folds it into memory.

To `discovery`: nothing. Discovery finds notes and the wallet records them.

## What an attacker who controls a neighbour can do

**An attacker with the file, on another device.** They hold rows sealed under a key derived from a
seed they do not have. They learn the number of rows, their sizes, and when the file was written.
That is a transaction count and a rough schedule, which is real metadata: the format does not pad
and does not pretend to hide it.

**An attacker who can modify the file.** Every row is authenticated, so an edit is detected and
stops the replay. What they can do is truncate, and a truncated store is a wallet that has forgotten
recent notes. It cannot forget that a note was spent while remembering the note, because a status
row lands after the row it refers to: cutting it also cuts everything after it, the next scan finds
the note again from the pool, and the nullifier the chain already holds marks it spent.

**An RPC that lies to the scan.** It cannot make this module store an unspendable note:
`discovery` recomputes every commitment before the wallet records it. It can withhold events, and
the wallet refuses any history whose leaves are not `0` to `n`, so a hole is a refused scan and
not a missing note.

**A spend nobody lands.** The notes the spend used are held as pending. Open settlement publishes
the spend again after 15 minutes and offers the owner to settle it after 30
(`wallet/publish`). Take back (`wallet/take_back.rs`) reads the whole history of the pool and
returns every pending note whose nullifier is not in it to the balance. A hand-off settled later is refused by the pool as a second
spend. `spec/Notes.tla` checks that no note is paid out twice and none stays frozen.

**A caller that folds a row without writing it.** The interface makes this awkward: a
wallet that folded first would, after a crash, show a balance its store disagrees with, and notes
would come back as spendable.
