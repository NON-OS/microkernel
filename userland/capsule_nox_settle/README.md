# capsule_nox_settle

## Role

`capsule_nox_settle` reads the chain and broadcasts for the meter, the wallet
and the store. It holds no key. It reads views from a fixed table,
checks and quotes intents against a fixed kind table, fetches a release's
bytes and verifies their hash, and sends transactions a key holder signed. Its
wire is `settle_proto`.

```text
meter (payment) / wallet / store
        |  settle_proto: READ, INTENT, GAS, CHANNEL_STATE, OPENED, BROADCAST, FETCH, SETTLE
        v
capsule_nox_settle --> Route::for_wallet(): the Nym mixnet or the Anyone network
                   --> an RPC host (TLS by nonos_tls, the chain checked)
```

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x3C
```

Network, IPC, Memory and Crypto (nonos_tls draws its randomness through
Crypto). There is no FileSystem and no key. Service `service:5200:nox_settle`;
replies go to its own kernel reply endpoint, 0x1_0000_001C (port 5201).
Every argument reaches an RPC host or a URL, so each op answers only its
callers (settle_proto's `callers`, v1.4.7), by which registry names the
kernel-stamped sender owns: HEALTHCHECK anyone; BROADCAST payment and the
wallet; CHANNEL_STATE, GAS, OPENED, READ and INTENT those two, app.store,
market.index and the installer; FETCH app.store and market.index; SETTLE
the wallet and the installer.

`nonos-capsule-nox-settle` turns it on. It is in `networkFeatures`
(tools/nix/config.nix), so the airgapped image drops it, and no profile array
names it: the owner adds it to one after signing. The kernel mirror
is `src/userspace/capsule_nox_settle`; init spawns it after the meter
(`spawn_plan/nox_settle.rs`).

## The route

Every connection goes over `Route::for_wallet()`: Nym, else Anyone, never
Direct. A route that is down answers ENETDOWN. One RPC host is asked per
chain, the wallet's own list; a failure moves to the next, and a host swapped
in must first answer eth_chainId with the chain's id. CHANNEL_STATE reads
`settled` from a second host as well and answers EIO when the two disagree.
A release is fetched over https with the host's chain checked, at most 1 MiB,
and a redirect is refused, never followed.

Limits on log reads: by address and topic0 only, from each
chain's scanned-to mark (kept in memory, so the first read after a start
is from deploy_block) to a head two hosts confirm. A deploy_block of 0 is
refused, never read from genesis; a range past 20 years of the chain's
blocks is EPROTO. Past MAX_KEYS or MAX_ROWS a chain's cache goes full: each
miss then reads that chain's whole history again, and settle logs one line,
once per chain, naming the chain and the cache and no ids.

Stated: settle's reads share net.anon's circuits, so an RPC operator can link
what one device reads. No unlinkability at the RPC is claimed.

## The contracts it names

A config contract is used only if it is deployed and passed its checks, made
at start and again on first use when the network was not there:

- it is feeExempt and not isBlacklisted on its chain's NOX (D11);
- its eip712Domain() is exactly the config's domain;
- the escrow's MAX_DELIVERY() and DISPUTE_WINDOW() are the config's, and
  the bond's UNBOND_DELAY() is at least UNBOND_DELAY_MIN;
- the bond's escrow() is the config's escrow.

One that fails is not deployed (ENOENT) until the next start. Before each
intent the flags of `to`, and of an approve's spender, are read again.

## Ops

ASYNC (header flags bit 0) is taken from payment's registry pid only, else
EINVAL; its answer goes back with mk_ipc_send_to_pid, echoing op, flags and
id, into the meter's service inbox (v1.4.15).

- LISTING (11): a listing's current releases from the config registry's
  Published and Withdrawn logs (topic0 only, pages of 10000 blocks from
  deploy_block), newest first, at most 16, a withdrawn release never. A
  per-chain cursor keeps the rows read so far (at most 1024), so a later call
  reads only new blocks; once full, each call rescans for its own listing.
  Rows are hints for the market's own checks.
- READ: a view of the table by number; the caller never names `to` or
  calldata. The raw ABI return, as words.
- INTENT: the calldata's selector must belong to its kind, and an approve's
  spender must be the config Settlement, Escrow or Bond. The quote is `to`,
  the sender's next nonce, the node's estimate times 6/5 and fee suggestions.
  Settle returns no calldata and changes none.
- GAS: balance, next nonce and fee suggestions (eth_feeHistory: the median
  tip, at least 0.01 gwei, and twice the next base fee plus the tip). The
  signer bounds them.
- CHANNEL_STATE, OPENED, BROADCAST: as the plan. A broadcast's hash must be
  keccak256 of the bytes sent.
- FETCH: RELEASE fetches a release document (one schema 3 release in the
  index's encoding, the plan) and holds it only if it decodes whole and
  keccak256 of its v3 signing form, recomputed from the decoded release, is
  the releaseHash asked; it answers the first page; PAGE pages through the one
  release held. ESCROW is EINVAL until B5.
- SETTLE (op 10): one drain round on a chain. Settle reads the meter's
  vouchers (DRAIN LIST), keeps those whose channel the config Settlement
  holds open with settled below their cumulative (at most twelve), quotes
  the meter's gas, estimates settle() (encoded here only for the estimate),
  has the meter build and sign its own (DRAIN SIGN), and broadcasts it. The
  reply names the transaction and what it carries; ENOENT when nothing is
  owed. Settle also drains at start and every six hours.
