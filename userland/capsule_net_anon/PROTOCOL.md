# Anyone Protocol, as verified

Everything below was read out of the network's own source and checked against
the live network on 2026-09-18. Nothing here is taken from the Tor
specification on trust: where the two agree that is recorded as agreement,
and where Anyone has moved the wire is recorded as a difference.

Sources read:

- `github.com/anyone-protocol/ator-protocol`, shallow clone of `main`,
  `configure.ac` reports `AC_INIT([anon],[0.4.10.4-git])`, `ChangeLog` head is
  `0.4.8.11`, so the fork point is Tor 0.4.8.11.
- `gitlab.torproject.org/tpo/core/tor` at tag `tor-0.4.8.11`, diffed against
  the above tree file by file.
- A live microdescriptor consensus and three live microdescriptors, fetched
  from authority `49.13.145.234:9230`.

## What is identical to Tor, and was confirmed identical

These are not assumptions. Each was diffed and came back byte for byte equal,
or the constant was read out of the fork's own header.

| Surface | Evidence |
|---|---|
| Consensus document parsing | `src/feature/dirparse/ns_parse.c` identical |
| Document signature rules | `src/feature/dirparse/sigcommon.c` identical |
| Authority certificate parsing | `src/feature/dirparse/authcert_parse.c` identical |
| Link handshake | `src/core/or/connection_or.c` differs only in two spelling fixes in comments |
| Flow control | `src/core/or/sendme.c` identical |
| ntor handshake | `PROTOID` is still `ntor-curve25519-sha256-1` |
| ntor v3 handshake | `PROTOID` is still `ntor3-curve25519-sha3_256-1` |
| Cell commands | `or.h`: CREATE2 10, CREATED2 11, RELAY 3, RELAY_EARLY 9, VERSIONS 7, NETINFO 8, CERTS 129 |
| Relay commands | `or.h`: BEGIN 1, DATA 2, END 3, CONNECTED 4, SENDME 5, EXTEND2 14, EXTENDED2 15 |
| Cell geometry | `CELL_PAYLOAD_SIZE` 509, `CELL_MAX_NETWORK_SIZE` 514, `RELAY_HEADER_SIZE` 11 |
| Relay key material | `CPATH_KEY_MATERIAL_LEN` is `20*2+16*2`, so Df, Db, Kf, Kb |
| Relay digest | running SHA-1 over the 509 byte payload with the integrity field zeroed, first 4 bytes written back |
| Relay cipher | AES-128-CTR, zero IV, one keystream per direction per hop |
| Directory URL space | still `/tor/...`, both in the client and in the cache dispatch table |
| Window constants | CIRCWINDOW 1000/100, STREAMWINDOW 500/50 |

## Where Anyone differs, and it matters

**1. TAP is gone.** `src/core/crypto/onion_tap.c` and `onion_tap.h` are deleted
in the fork. `circuitbuild.c` no longer has a branch that can choose
`CELL_CREATE`: the cell type is always CREATE2 and the handshake is ntor, or
ntor v3 when the exit advertises congestion control. A client therefore never
needs the RSA-1024 onion skin, and this capsule does not carry one.

**2. The RSA onion key is optional in descriptors and microdescriptors.**
`microdesc_parse.c` moves `onion-key` from `NEED_KEY_1024` to a new
`OPT_KEY_1024` object rule and stops storing the key at all.
`routerparse.c` moves `onion-key` from `T1` to `T01` and pairs it with
`onion-key-crosscert`, also made optional, refusing a descriptor that carries
one without the other. A parser written against Tor's grammar rejects an
Anyone microdescriptor that omits the key. Ours treats it as optional.

Live check: the three microdescriptors fetched today all still carry an
`onion-key`, because the relays answering are on `Tor 0.4.9.11-live`. So the
field is optional in the grammar and present in practice, and only a parser
that accepts both is safe across the fleet.

**3. Ed25519 identity is mandatory.** The same `routerparse.c` change promotes
`identity-ed25519`, `master-key-ed25519`, `router-sig-ed25519` and
`ntor-onion-key-crosscert` from optional to required, where Tor accepts a
descriptor with none of them. Every Anyone relay has an Ed25519 identity, so
the link handshake can insist on an Ed25519 certificate chain and never fall
back to an RSA-only peer.

**4. There are no fallback directory mirrors.** `src/app/config/fallback_dirs.inc`
in the fork contains a comment and nothing else:

    // Temporary list of fallback directory mirrors to prevent ator-protocol
    // connecting to the original Tor network during development

A client bootstraps from the authorities or not at all. That shapes the
directory design: seven addresses, each retried, and no wider mirror set to
fall back on when they are slow.

**5. The authorities and their ports are their own.** From
`src/app/config/auth_dirs.inc`: seven authorities, DirPort 9230, authority
ORPort 9201. Ordinary relays advertise ORPort 9001, which the live consensus
confirms on every `r` line.

**6. There is a naming layer Tor does not have.** `src/feature/anyone/` and
`src/feature/dirparse/anyone_hosts_parse.c` are new. They fetch and verify a
signed `anyone_hosts` document that maps `.anyone` names onto onion addresses,
trust-anchored to six hardcoded onion services in `DEFAULT_ANON_DNS_MAPPING`.
This is their equivalent of a DNS root. It is not on the path of a clearnet
fetch through three relays, so it is out of scope here and named so that the
gap is visible rather than forgotten.

## Live network, measured 2026-09-18 19:00 UTC

    consensus-method 34
    valid-after 2026-09-18 19:00:00, fresh-until 20:00:00, valid-until 22:00:00
    5057 relays: Running 5057, Valid 5057, Fast 4988, Stable 4891,
                 Guard 4092, Exit 3172, HSDir 3946, Authority 7
    required-client-protocols Cons=2 Desc=2 Link=4 Microdesc=2 Relay=2
    params: cc_alg=2, ExtendByEd25519ID=1
    bandwidth-weights Wgg=10000 Wgd=1893 Wmd=4411 Wed=3696 Wee=10000 ...

The microdescriptor consensus inflated to 1,791,489 bytes from 376,598 on the
wire, so the transport has to inflate zlib and hold about 1.8 MB.

## Re-measured 2026-09-19 10:00 UTC, two authorities, both flavours

Fetched from 49.13.145.234 and 5.161.108.187, as `consensus-microdesc` and as
the full `consensus`. All four documents agree exactly, which is the point of
asking more than one: a count that differed between authorities would mean one of
them was serving something the others had not signed.

    5057 relays, 5050 usable non-authority, 7 of 7 authorities signed
    4110 guard eligible, 4996 middle eligible, 3158 exit flagged and fast
    854 distinct /16 networks
    largest /16 holds 1002 relays, 19.84 per cent of the usable pool
    bandwidth-weights Wgg=10000 Wgd=1870 Wmg=0 Wmd=4393 Wme=0 Wmm=10000
                      Wee=10000 Wed=3737

Two things follow that the code depends on.

`Wmg` and `Wme` are published as zero, so a relay carrying only the Guard flag or
only the Exit flag has no weight in the middle position and is dropped from that
candidate set. The middle pool is the relays weighted by `Wmm` and `Wmd`, not the
4996 that merely hold the Fast flag.

One /16 holding a fifth of the network is why a path excludes a whole network and
not only an identity, and 854 networks is why doing so cannot starve a draw. Both
are asserted against a live document in `anon_ntor_proofs`, which reads one from
`ANON_LIVE_CONSENSUS` and passes quietly when there is none.

A headline relay count larger than this is counting nodes seen over a window
rather than the current consensus. Only the consensus is signed, so only the
consensus is something a client can draw a path from.

### Exit policies, 552 live microdescriptors sampled

    3158 exit flagged and fast, every sampled one carrying a p line
    497 published `reject <ranges>`, 55 published `accept <ranges>`
    552 of 552 let both 80 and 443 out
    0 would have passed on port 80 alone

Two conclusions. Requiring both web ports rather than either costs nothing on
this network, so the rule stays as it is: a circuit is drawn before the scheme of
the request is known, and an exit good for a plain fetch and not a secure one
would fail at the last hop after three handshakes.

And nine exits in ten publish the reject form. A summary reader that understood
only `accept` would drop 497 of these 552 and make the network look a tenth of
its size, which is why the verb is read and inverted rather than the ranges being
taken at face value.

## What this capsule implements

ntor only, CREATE2 and EXTEND2, link protocol 4 or 5, a three hop circuit over
guard, middle and exit, RELAY_BEGIN streams with authenticated v1 SENDMEs, and
a directory client that reads the microdescriptor consensus from the seven
authorities over their DirPort. The `.anyone` naming layer, onion services,
conflux, congestion control and ntor v3 are not implemented, and each is
listed here rather than left to be discovered.
