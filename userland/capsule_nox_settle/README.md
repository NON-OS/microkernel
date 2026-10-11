# capsule_nox_settle

## Role

`capsule_nox_settle` reads the chain and broadcasts for the meter, the wallet
and the store. It holds no key. It reads views from a fixed table,
checks and quotes intents against a fixed kind table, fetches a release's
bytes and verifies their hash, and sends transactions a key holder signed. Its
wire is `settle_proto`.

```text
meter (payment) / wallet / store
        |  settle_proto: READ, INTENT, GAS, CHANNEL_STATE, OPENED, BROADCAST, FETCH
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
Every op is open to any caller: what settle answers is the chain's, or a
broadcast of bytes another signed.

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

Stated: settle's reads share net.anon's circuits, so an RPC operator can link
what one device reads. No unlinkability at the RPC is claimed.

## The contracts it names

A config contract is used only if it is deployed and passed its checks, made
at start and again on first use when the network was not there:

- it is feeExempt and not isBlacklisted on its chain's NOX (D11);
- its eip712Domain() is exactly the config's domain;
- the bond's escrow() is the config's escrow.

One that fails is not deployed (ENOENT) until the next start. Before each
intent the flags of `to`, and of an approve's spender, are read again. The
escrow and bond constants join the checks when the config holds them (B5).

## Ops

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
- FETCH: RELEASE holds a release only if keccak256 of its bytes is the
  releaseHash asked, and answers its first page; PAGE pages through the one
  release held. ESCROW is EINVAL until B5.
