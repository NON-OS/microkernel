# capsule_model_fetch

## Role

`capsule_model_fetch` is `tool.model-fetch`, the program behind `qwen get
TIER...` and `qwen tiers` in the Terminal. It checks the signed model
catalogue built into it, downloads the files of the Qwen tiers a person names
over HTTPS, and feeds each file to the kernel, which seals it into the data
volume and links it only when its SHA-256 is the pinned digest. `qwen tiers`
prints each tier's size, the memory it needs, and how much the volume holds.

```text
Terminal: qwen get TIER -> MkToolRun("tool.model-fetch") -> verified spawn
catalogue.bin -> Ed25519 check -> pin check -> files and mirror URLs
each file:
  MkDataFeedBegin(name, sha256, bytes)        -> byte AT to start from
  GET, Range: bytes=AT-  (TLS over net.socks5 | net.anon | net.sockets)
  body -> batches of up to 1 MiB -> MkDataFeed -> kernel seals and hashes
  MkDataFeed(end=1) -> SHA-256 is the pin ? link the file : EBADMSG
```

## How it is spawned

The Terminal checks the tier words and calls `MkToolRun("tool.model-fetch",
argv)`, argv being `get` or `tiers` and the tiers, NUL-separated. The kernel
(`src/userspace/tool_capsules/model_fetch`) spawns it through the verified path,
its signed artifacts baked in with feature `nonos-capsule-model-fetch`, parented
to the Terminal, which shows its lines and stops it on Ctrl-C. Refusals: ENOENT
not built in, ENETDOWN no network this boot, EPERM caller lacks Network or file
access or artifacts fail to verify, EBUSY a fetcher already holds its endpoint
(`service:4960`), EACCES Linux and Qwen off.

The Linux installer (`app.linux.install`, which holds Network and FileSystem
as the Terminal does) starts it the same way with `get TIER` when a Qwen tier
installed from the store lacks a model file (`install/fetcher.rs` in
`capsule_linux`). It drains the output and drops it, so nothing the fetcher
says reaches the serial log through it, and reads the reason in the exit
status.

Exit (`src/exit.rs`, included by the installer by `#[path]`): 0 done, 1 a file
refused otherwise (no mirror served it), 2 usage or unknown tier, 3 no signed
catalogue, 4 the chosen network is not running, 5 no data volume, no NONOS disk
at all (ENODEV), 6 volume locked (EACCES), 7 bytes not the pin or the name taken
(EBADMSG, EEXIST), 8 no room (ENOSPC; on a live session, whose volume is in
memory, that memory filling), 9 another download holds the volume's stream
(EBUSY), 10 a file name too long for the volume to keep, 11 the tier needs more
memory than the machine has in all, 12 the volume could not be reached (EIO,
EAGAIN), which a retry may get past, 13 too little free memory to hold a live
session's volume at all (ENOMEM). With several tiers, the first refusal's.

## Microkernel contract

- `MkDataFeedBegin(name, sha256, bytes, probe)`: returns the byte to feed
  from; EALREADY when `<name>.sha256` records that digest; ENOSPC when the rest
  will not fit; EBUSY while another live process holds the one stream allowed.
  `probe=1` (`qwen tiers`) reports where a stream stands and holds nothing.
- `MkDataFeed(buf, len, 0)`: at most 1 MiB a call, sealed and hashed; EFBIG
  past the declared length. A mark `<name>.partial` is saved every 64 MiB.
- `MkDataFeed(0, 0, 1)` finishes: EINPROGRESS when bytes are missing; EBADMSG,
  stream and mark discarded, when the SHA-256 of what was sealed is not the
  digest named at begin. Else the kernel links the file, reads it back and
  hashes it again (EBADMSG and unlink on a mismatch), then writes the
  `<name>.sha256` record. `MkDataFeed(0, 0, 2)` puts it down, mark saved.

The kernel holds the file to the digest named at begin; the capsule names only
pinned digests. It also uses `MkArgs`, `MkServiceLookup`, `MkIpcCall`,
`MkPrivateWrite`, `MkProcStat` (machine memory), the crypto calls and `MkExit`.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x40000003D`, also declared in `src/caps.rs` (checked
by `scripts/check_declared_caps.py` before signing) and the kernel's `spawn.rs`.

| Bit | Capability | Used for |
|---|---|---|
| `0x001` | CoreExec | run at all |
| `0x004` | Network | reach the services that carry a connection (`net.sockets` admits only Network holders) |
| `0x008` | IPC | `MkIpcCall` to `net.socks5`, `net.anon`, `net.sockets`, `crypto_pool`; `MkPrivateWrite` |
| `0x010` | Memory | the heap holding TLS records and the 1 MiB feed batch |
| `0x020` | Crypto | `CryptoRandom` and X25519 for the TLS handshake |
| `0x400000000` | StreamImport | `MkDataFeedBegin` and `MkDataFeed` |

No FileSystem (`0x040`): `MkDataRead` and `MkDataStat` need it and StreamImport
grants no read, so a process that holds sockets cannot read what the volume
holds. No Debug (`0x100`): its only output is its own inbox, read by the
Terminal that started it, so tier names, mirror hosts and progress do not reach
the serial log through it. A line the inbox cannot take within 5 s is dropped.

## Network route

`Route::chosen()`, from `nonos_route_link`, reads the system's default network
(`Field::NetworkRoute`, set at setup and in Settings) and looks up `net.socks5`
and `net.anon`, once per run (`src/get/run.rs`). `pick` in
`nonos_route_link/src/pick.rs` decides; model_fetch_proofs and route_link_proofs
both hold it for every answer. `src/net/route.rs` then opens the link. Nym
(the default, also when the policy store holds an unknown value or cannot be
asked): `net.socks5`, a SOCKS5 CONNECT that sends the host name unresolved.
Anyone: `net.anon`, a three-hop circuit to an exit that resolves the host.
Direct: TCP through `net.sockets`, only when that is the default. A default
whose network is not running is no route: a file with bytes still to come is
put down with its mark and refused, and nothing is asked of another network.
A route that fails a request fails it. `nonos_tls` checks the certificate chain against built-in
roots for the host, with signature checks done by `crypto_pool`. A 206 must
start at AT and give the pinned total; a 200 must have the pinned length, and
its first AT bytes are skipped. Up to 5 redirects, to `https://` only.

## Signed catalogue and pins

`mk/22-models.mk` runs `tools/nonos-qwen-tier.py catalogue`, which builds
`$(TARGET_DIR)/models/catalogue.bin` from the pins in
`userland/capsule_linux/src/linux/file/models/pinned*.rs` and signs it with the
marketplace operator Ed25519 seed. Without the seed it is empty, and the
capsule says so and exits 3. Layout: `NXQWEN01`, serial, mirror base, tiers
(word, memory, files: name, length, SHA-256, URLs), 64-byte signature. A file's
URLs are `NONOS_MODEL_MIRROR/TIER/FILE` when the build set it, then Hugging
Face, both by the name the Qwen team publishes the file under. `catalogue/check.rs` verifies the signature under
`.keys/marketplace_operator_ed25519.pub`, parses strictly, and `catalogue/admit.rs`
requires each file to be a name the data volume can keep and to equal a
compiled-in pin (`src/pins`, the same tables by `#[path]`) in tier, length and
SHA-256, with a mirror. One mismatch refuses the whole catalogue before any
fetch.

A name the volume keeps is at most 49 bytes with its slash, so that
`<name>.partial` and `<name>.sha256` fit a 56-byte directory entry
(`src/fs/blockfs/dir_consts.rs`, `src/fs/blockfs_volume/import_feed/live.rs`).
`pinned.rs` stops the build on a pin that is longer, `nonos-qwen-tier.py`
refuses to write a catalogue naming one, and the fetcher refuses one at load.
The parts of Coder 7B, 14B and 32B are published under 52 to 54 bytes, so
they are pinned and kept without `-instruct` and downloaded by their
published names (`tools/nonos_qwen_tier/upstream.py`).

## Failure model

Before a tier's first byte, its memory need from the signed catalogue is
held to the machine's total memory (`MkProcStat`): a tier that needs more
is refused in one sentence naming both, and nothing of it is downloaded
(`src/get/memory_need.rs`). Memory the kernel will not report refuses
nothing; qwenchat checks again against free memory before it loads.

Each file ends in one line: `already here, verified`, `verified against the
signed pin`, or `refused:` with the reason and errno name (`src/errno.rs`). A
mirror fault (unreachable, TLS or certificate failure, HTTP status, wrong
length, 60 s without a byte) is printed with the mirror host. The same mirror
is tried again if bytes came, else the next, waiting 1 s more per failure in a
row, up to 10 s. After 3 rounds of every mirror with no byte, the stream is put
down with its mark and the file refused, saying how much is kept. EBADMSG at
finish starts the file again from byte 0 at the next mirror while one is
untried. A refused file stops its tier; later tiers go on. Resume: after
Ctrl-C the kernel keeps the stream and the next `qwen get` goes on from the
last byte fed; after a reboot, from the last mark (at most 64 MiB lost).

## Verification

`userland/model_fetch_proofs` reads a catalogue written by
`tools/nonos-qwen-tier.py` under a test key with the capsule's own
`parse.rs`, `read.rs`, `types.rs` and pins. It checks the signature (and its
failure after one flipped byte), serial, base and 17 tiers, entries equal to
the pins in order, the NONOS URL before Hugging Face, and that a body one byte
short does not parse, that `admit.rs` takes that catalogue and refuses one
naming a file the volume cannot keep, and the memory check on the
catalogue's own numbers. It holds `nonos_route_link`'s `pick.rs` for every answer the policy store
could give, and the store install of a Qwen tier in its pure parts: the
listing as the kernel names it, the personality's tier tables and
`model_dep.rs`, `exit.rs`, and the store's sentences. It does not cover
`check.rs`, the network code or the kernel feed path. Run it, and the
writer's own tests, from the repository root:

```text
(cd userland/model_fetch_proofs && RUSTFLAGS= cargo test --release)
python3 -m unittest discover -s tools/nonos_qwen_tier -t tools
```
