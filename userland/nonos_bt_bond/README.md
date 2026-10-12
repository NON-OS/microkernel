# nonos_bt_bond

Bonds for an LE-only host (Decision 21), written from the Bluetooth Core
Specification 5.4. Every fact in the source cites its volume, part and
section. A bond is what pairing leaves behind so the peer can be found and
the link encrypted again without pairing (Vol 3, Part C, 9.4.4).

| Module | What it holds |
|---|---|
| `record` | `BondRecord`: one bonded peer as 48 octets, version 1. Its LTK, its IRK if it sent one, its identity address, whether the pairing was authenticated, and the key size. Made from a finished pairing (`from_pairing`), encoded, and decoded with every octet checked. |
| `local` | `LocalIrk`: this host's own identity resolving key as 17 octets, version 1. |
| `book` | `Book`: up to 15 bonds, found by identity or, for a resolvable private address, by the bond whose IRK resolves it (Vol 6, Part B, 1.3.2.3). |
| `rotation` | `Rotation`: this host's resolvable private address, renewed every `ROTATE_MS`, the 15 minutes Vol 3, Part C, Appendix A recommends for TGAP(private_addr_int). |

The bond record:

```text
[0]      version, 1
[1]      flags: bit 0 authenticated, bit 1 IRK present, bit 2 LE Secure
         Connections (always set); the rest zero
[2]      identity address type: 0 public, 1 static random
[3..9]   identity address, least significant octet first
[9..25]  LTK, most significant octet first
[25..41] IRK, most significant octet first; zero when absent
[41]     key size, 7 to 16
[42..48] zero
```

Pairing here is LE Secure Connections only, so EDIV and Rand are zero and
are not kept (Vol 3, Part H, 2.4.4). A peer is kept by the identity it
distributed; one that sent none is kept by the address it connected from
if that address is public or static random. One that connected from a
private address and sent no IRK could never be found again, and is refused
(`NoIdentity`).

Reading a record back refuses a wrong length, an unknown version, an
unknown or missing flag, a reserved octet not zero, an IRK field that
disagrees with its flag, a key size outside 7 to 16, a zero LTK or IRK,
and an identity that is neither public nor a valid static random address
(Vol 6, Part B, 1.3.2.1). The keys are overwritten when a record, a local
key or a rotation is dropped.

Where the records are kept: `capsule_keyring` is to hold them, one record
per bonded peer and one for this host's IRK, owned by the Bluetooth
capsule's endpoint name. Its record API cannot yet: records are owned by a
pid, live in memory only, and cannot be listed (letter E-21). Until it can,
a `Book` lives for one boot.

The proofs live in `nonos_bt_bond_proofs`.
