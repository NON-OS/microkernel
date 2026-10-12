# nonos_bt_smp

The Security Manager Protocol for an LE-only host (Decision 21), written
from the Bluetooth Core Specification 5.4, Vol 3, Part H. Every fact in the
source cites its section. It offers LE Secure Connections only. Legacy
pairing's key exchange can be broken by anyone who listens to it (Vol 3,
Part H, 2.3.5.6), so a peer that offers only legacy pairing is refused
rather than paired with.

| Module | What it holds |
|---|---|
| `cmac` | AES-CMAC (RFC 4493): the system's one, from `nonos_aes`, re-exported. |
| `funcs` | f4, f5, f6 and g2 (Vol 3, Part H, 2.2.6 to 2.2.9), and the 56-bit address form f5 and f6 take. |
| `ecdh` | P-256 key pairs and the peer's public key, on `nonos_p256`, which checks the key on the curve and clears the secret on drop. The published debug key is refused here (2.3.5.6.1). |
| `method` | IO capabilities and the choice of pairing method from both sides (2.3.5.1). |
| `pdu` | SMP PDUs with exact lengths, Pairing Request and Response, the Pairing Failed reasons, and conversion between air order and function order. |
| `pairing` | `Initiator`, the central's side of pairing as a state machine. It is fed the peer's PDUs, the user's answers and the time. It returns the PDUs to send, what to show or ask the user, and the outcome. |
| `privacy` | ah, the random address hash (Vol 3, Part H, 2.2.2); prand, a resolvable private address made from an IRK, and resolving one (Vol 6, Part B, 1.3.2.2 and 1.3.2.3). |

The functions take values most significant octet first, as the
specification prints them. PDUs carry them least significant octet first.
The LTK in `Outcome::Encrypt` and `Bond`, and the IRK in `Bond`, are in
function order. The identity address in `Bond` is in air order, as HCI
takes addresses.

Pairing, step by step:
1. `start` sends the Pairing Request (`REQUEST`): KeyboardDisplay, with
   bonding, MITM and Secure Connections, a 16-octet key, and the
   responder's identity key.
2. The response must carry Secure Connections and a 16-octet key, and no
   OOB flag.
3. The public keys are exchanged. A key off the curve, the debug key, or
   our own key sent back each end pairing.
4. Stage 1 runs by the chosen method:
   - Just Works.
   - Numeric comparison: the user confirms six digits.
   - Passkey entry: twenty rounds, with the passkey shown here (drawn
     without bias) or typed here.
5. Stage 2 runs the DHKey checks. The caller then encrypts the link with
   the LTK and calls `encrypted`.
6. The responder's IRK and identity address arrive. If this host started
   with `start_private`, offering its own identity (init_keys ID), and the
   response asked for it, this host's IRK and identity address follow, in
   that order (3.6.1, 3.6.4, 3.6.5). The result is `Outcome::Paired(Bond)`.

`start` offers no key of this host's own. `start_private` refuses an
offer of the identity with no identity to send, an identity with no offer,
and any other key: under Secure Connections the LTK is derived, not sent,
and this host signs no data.

A PDU out of place, an unknown code, or a malformed PDU ends pairing with
Pairing Failed and the reason 3.5.5 gives. Pairing also ends when 30 s
pass with no PDU (3.4). After that nothing more is sent.

Every draw from the random source is bounded: eight tries for the secret
and sixteen for the passkey. A broken source ends pairing instead of
spinning.

The proofs live in `nonos_bt_smp_proofs`.
