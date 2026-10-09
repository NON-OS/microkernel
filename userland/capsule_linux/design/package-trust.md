# Package trust anchors

A pacman or Debian backend verifies nothing until its keyring is pinned at
build time (`NONOS_PACMAN_KEYRING`, `NONOS_DEB_KEYRING`). Without one it
refuses every install and says so. That is the correct state until an anchor
has been established, and it is where both families stand.

## The rule

A trust anchor comes from outside the channel it protects, and from more than
one place agreeing:

1. The distribution's keyring package from its repository, with the
   fingerprints read out of the package itself.
2. The distribution's published bootstrap (for BlackArch, `strap.sh`) and the
   key it pins, fetched over TLS with the certificate checked.
3. The distribution's own announcement of the key, or of a rotation.

If all three agree, the key is pinned and the commit records where each came
from and on what date. If they disagree, the disagreement is the finding and
nothing is pinned. A fingerprint recalled from memory, a model's included, is
not a source.

## What a pinned keyring is trusted for

Two claims, which the code must not blur:

- "Key X says these are the distribution's signing keys." That is what a
  keyring package signed by X establishes.
- "This package is authentic." That is established only by a signature over
  the package, or over the index that names its checksum, from a key the
  anchor admits.

Today `install/pgp` accepts a signature from any key in the pinned file. When
an anchor is pinned, that must narrow to the keys the distribution marks as
trusted for signing (for pacman, the `-trusted` list), not every key the file
happens to contain.

## BlackArch, 2026-09-27: not pinned

- Source 2, `https://blackarch.org/strap.sh` over TLS: pins master key
  `4345771566D76038C7FEB43863EC0ADBEA87E4E3` (Evan Teitelman) and keyring
  version 20251011.
- The keyring tarball `blackarch-keyring-20251011.tar.gz` is signed by
  `CBA3C7D4798912702DCF568E67D8BDF42AD93F4E`, not by the key `strap.sh` pins.
- Source 1, `blackarch-keyring-20251011-2-any.pkg.tar.zst` from the BlackArch
  repository (SHA-256 matching its database record): `blackarch.gpg` holds
  eight primary keys, including both of the above. Its `blackarch-trusted`
  list marks four as trusted: `8F9A9793CB8591147C2EC70566E0CDBD1E01F333`,
  `A0917C4147A37007CB54C1CFD295AA940EFDDF62`,
  `4345771566D76038C7FEB43863EC0ADBEA87E4E3`,
  `F9A6E68A711354D84A9B91637533BAFE69A25079`. The tarball's signer, `CBA3…`,
  is not among them.
- Source 3: not obtained. The keyring's history is on GitHub, which this
  environment's network policy refuses (403), and the news and blog pages on
  blackarch.org name no key or rotation.
- Sources 1 and 2 are both served from blackarch.org, the channel the anchor
  would protect.

Finding: the keyring's signer is outside the keyring's own trusted list, and
no independent source was reachable. The backend stays keyless.

## Kali, 2026-09-27: pinned

Pinned: `827C8569F2518CC677FECA1AED65462EC8D5E4C5`, "Kali Linux Archive
Automatic Signing Key (2025)", RSA 4096, created 2025-04-17, expires
2028-04-17, in `keys/kali/archive-key-2025.asc` (SHA-256 `bbaef4b3...71b1`).
All three sources name it:

1. The `kali-archive-keyring` 2025.2 package from kali-rolling (SHA-256
   `9250b08f...c8cf0`, matching its Packages record): `kali-archive-keyring.gpg`
   holds this key, and also the old repository key `ED444FF07D8D0BF6`.
2. `https://archive.kali.org/archive-key.asc` over TLS: this key alone.
3. Kali's announcement, `https://www.kali.org/blog/new-kali-archive-signing-key/`:
   names this key as the new signing key, and says Kali lost access to the old
   one.

Only the 2025 key is pinned: the old key's holder says they no longer control
it, so a signature under it proves nothing. The kali-rolling `Release` fetched
the same day verifies under the pin, and the proof crate checks that against
the committed copy.

What it is trusted for: that a `Release` for the suite is Kali's. A package is
authentic only through that `Release`, by the checksum it gives the Packages
file, and the checksum that file gives the `.deb`.

Known limit: kali-rolling's `Release` has no `Valid-Until`, so an older, validly
signed `Release` replayed by a mirror is not caught.

## In-tree tools, 2026-10-03

The tools built in this tree that an image does not carry (`LINUX_USERLAND_PACKAGE`
in `userland/linux_userland/Userland.mk`) need no distribution anchor. Their
anchor is the signed market index the image already embeds: each is listed as
`linux.nonos-<tool>` with the BLAKE3 of its content (its program and the data it
reads, one tar the seal makes the same way every time,
`tools/nonos_market_catalogue/packages.py`). The installer fetches that content
from the NONOS package mirror at the path its pin names and refuses any other
bytes (`install/tools_install.rs`), so the mirror is not trusted with anything:
it can serve the pinned bytes or nothing.

What the content does not carry is the tool's proof files. The market index is
made before the seal signs, and the trailers prove against a policy root that
covers the capsule the index is embedded in, so the index cannot pin them; and
`place_entry` refuses a package writing its own proof. An in-tree tool is placed
and vouched for exactly like any other installed package, and runs under the
same rule: on a machine with an enrolled local root, vouched; on a live boot,
installed but unvouched, and refused by the exec gate. Carrying the publisher's
proofs with the content, so that a live boot runs it with nothing vouched, is a
change to what an install may write and is ek's decision
(docs/handbook/linux/tools-0.9.2.md, "Trust").
