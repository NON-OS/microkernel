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

## Kali: not pinned

Held to the same three sources before anything is pinned.
