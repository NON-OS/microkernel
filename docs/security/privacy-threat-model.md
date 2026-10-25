# Privacy: The threat model

Part of [Privacy](privacy.md).

What NONOS's privacy promise is, against whom, and which test holds each part.
A claim without a test that runs is marked so, and nothing here says more than
its test shows.

## Who we defend against

| Who | What they hold |
|---|---|
| A1, the finder | The machine or its disk, powered off: every sector |
| A2, the cold reader | The machine's RAM right after a shutdown or restart |
| A4, a network's relay | One Nym gateway, an Anyone guard or exit, or an RPC node |
| A5, a hostile capsule | Code running with the capabilities its manifest asked for |
| A6, the verifier | The signed health report this machine hands out |
| A7, the far end | Each site, mirror and peer you choose to reach |

## The claims

| Claim | Against | What holds it |
|---|---|---|
| C1. Until you choose to keep things, the kernel refuses every write to the disk store, whichever capsule calls the store write; only the policy capsule the kernel spawned can turn keeping on, and nothing turns it off within a boot | A1, A5 | `zerostate_proofs`, with the mutant "keeping turned on by another capsule" caught; `src/security/zerostate/keep.rs` |
| C2. An installed disk's data volume opens only under the TPM's key for this machine in this boot state | A1 | the key is derived by the TPM under PCRs 0, 4, 7 and 9 (`src/security/tpm/machine_key/pcrs.rs`); no booted test in this unit |
| C3. After Power Off, what you typed this boot is not in RAM | A2 | the zero-state boot test (`tools/nonos-zero-state-check`) |
| C4. Nothing that lasts across boots reaches the wire: hardware address, DHCP, Nym and Anyone identities are new each boot | A3, A4 | `nonos_mac`, held by `e1000_proofs`, `e1000e_proofs` and `igb_proofs`; DHCP's options are not yet held by a test |
| C5. Every connection made without your asking is listed, with why and how to stop it | A3 | `tools/nonos-egress-audit`, in the static checks |
| C6. A capsule without Network is refused every network service by the kernel | A5 | the send gate in `src/syscall/microkernel/ipc/send_caps.rs`; a booted refusal test is still to come |
| C7. The health report states the privacy posture and carries no serial, address or other lasting identifier | A6 | still to come |

## What we do not claim

- A power cut, or holding the power button, skips the RAM wipe. A panic wipes
  the kernel heap but not process memory (`src/security/zerostate/panic_wipe.rs`).
  Cold-boot reading after any of these is not fully defended against.
- A verifier who holds this TPM's endorsement certificate can link reports the
  attestation key signs, since that key is the same every boot.
- Someone watching both this machine and the far end at once can match the
  traffic by its timing, through Nym as through Anyone.
- Under the Direct network, names are looked up in the clear at the DNS server
  the DHCP lease named.
- The local network sees that this machine runs Nym and Anyone, under every
  network choice (see [What leaves without your asking](privacy-unprompted.md)).
- A capsule holding StoreWrite can still reach a disk's raw sectors through its
  driver. Every Linux role holds StoreWrite today; narrowing that to the install

## See also

- [Privacy guarantees](privacy-guarantees.md): each promise at the level its
  proof reaches, with the booted evidence
- [What NONOS protects against](protections-and-limits.md)
- [Device secrets and keys](device-secrets-and-keys.md)
