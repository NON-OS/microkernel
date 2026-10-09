# Discovery

How the wallet decides which notes in the pool are its own, and what a hostile RPC or sender can do
about it. What is checked and what is not is in [20-security-status.md](../../../docs/20-security-status.md).

## What this owns

Deciding which notes on the pool belong to this wallet, from the events of the pool and nothing
else, and the checks that turn a decrypted blob into a note the wallet can spend. `chain.rs` reads
three events:

| Event | What it gives |
|---|---|
| `NoteCommitted(commitment, leafIndex)` | every leaf and its commitment |
| `OutputNote(leafIndex, clientData)` | the 1,186-byte X-Wing blob of each settlement output. A deposit has none, since the depositor already holds its opening |
| `NullifierSpent(nullifier)` | a note retired |

From those it finds the notes paid to this wallet, its own deposits once the pool has stored them,
and which of its notes are spent (`ChainScan`). Nothing is asked of a server, and every wallet reads
the same events, so no request says which notes are whose.

The choice between reading everything, filtering and private information retrieval is written up
in [04-discovery.md](../../../docs/04-discovery.md). What is built is reading everything.

## What it trusts

The receive key from `keys`, to be the only key that opens a blob sealed to this wallet.

The note cipher in `notes`, for the property that a blob which opens under that key was sealed to
it. It does not trust that alone: an opening becomes a note only if it recomputes to the leaf the
pool stored and carries a spend key this account derives (`watch.rs`). The pool cannot make that
check itself, since it stores a leaf from an opaque owner digest, so a leaf built from a wrong digest
is one nobody can ever spend. The check lives here.

The pool hash, for recomputing commitments. A wallet whose hash disagreed with the pool would find
nothing, which fails loudly.

Its own derivation of nullifiers (`spent.rs`). The pool publishes a nullifier without saying which
commitment it retires, which is the link the proof hides, so the wallet derives the nullifier of
each note it holds from its own key and the position of the note, and marks the note spent when
that nullifier appears.

## What it hands its neighbours

To `wallet`: a `ChainScan` with the notes received, the deposits stored, and the commitments now
spent. Each note is already checked.

## What an attacker who controls a neighbour can do

**An RPC that serves crafted events.** It can serve a blob that opens to an opening which does not
recompute to its leaf, and that blob is dropped. It can serve an event twice, and the store keys
notes by commitment, so a duplicate is one note. It cannot make the wallet record a note it cannot
spend, and it cannot learn which events the wallet kept.

**An RPC that withholds events.** The wallet reads the whole history and refuses any whose leaves
are not `0` to `n` (`first_gap` in `logs.rs`), trying the next RPC. An RPC that withholds a
nullifier makes a spent note look unspent until another RPC serves it. A spend of that note is then
refused by the pool. The same history tells `follow_spend` whether a published spend landed, so a
withheld nullifier makes a landed spend look unlanded: it is published again, which the pool
refuses as a second spend, and a self settlement it offers fails its simulation and costs nothing.

**A sender who wants to mark a recipient.** The view tag is derived from a shared secret the sender
can only compute for the address they were given. They can send a note with an absurd value or an
asset this wallet does not count. The balance is arithmetic over checked notes, and a value above
`MAX_VALUE` is discarded.

**Anything that reads the counts.** How many blobs were opened says roughly how many notes this
wallet holds. The counts stay on the device.
