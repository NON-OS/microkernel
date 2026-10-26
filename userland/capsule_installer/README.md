# capsule_installer

## Role

`capsule_installer` is the userland package install authority, service
`installer`. It verifies signed NONOS packages with the kernel and writes their
four files into the capsule store, removes them, lists what is installed, and
loads a stored capsule through the kernel's verified spawn path for a caller.
For an install a person pays for, it asks the meter (`payment`) for a signed
voucher. It owns no storage device and writes nothing to a disk itself. The handbook page is
[docs/handbook/apps/market-and-store.md](../../docs/handbook/apps/market-and-store.md).
The disk installer is a different capsule, `capsule_install`.

```text
desktop_shell / terminal (nox pkg, exec, install)
        |
        | OP_PKG_QUERY / OP_PKG_COMMIT / OP_LOAD_BY_NAME / ...
        v
capsule_installer -- MkCapsuleVerify, MkCapsuleLoad --> kernel
        |-- store_install / store_uninstall --> vfs_pool
        `-- PAY v2 (payment_proto) --> payment, the meter
```

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x800059
```

CoreExec for `MkCapsuleLoad`, `MkCapsuleVerify` and `MkGetPid`, IPC, Memory,
FileSystem for the vfs calls, and SpawnBroker, which lets a load name the
requester as the child's parent (`on_behalf_of`). Service
`service:4112:installer`; replies go to the kernel reply endpoint. The capsule
is turned on by `nonos-capsule-installer` in `microkernel-desktop-base`, so the
standard desktop images carry it and the offline desktop does not. vfs accepts
`store_install` only from the pid registered as `installer`.

## Interface contract

Every request is a sequence word, an op and a payload; every reply carries the
same sequence word and a status.

| Op | Name | What it does |
|---|---|---|
| 1 | `OP_HEALTHCHECK` | liveness |
| 2 | `OP_INSTALL` | the body is a payment_proto PAY v2 request for exactly one unit of "install"; the installer refuses anything else with `EINVAL`, relays it to the meter and answers with the meter's signed voucher (265 bytes), or the meter's status. A free listing needs no call |
| 3 | `OP_LOAD_FROM_STORE` | retired (S-13): answers EINVAL and loads nothing; the number stays taken (`protocol/retired.rs`) |
| 4 | `OP_LOAD_BY_NAME` | read `/capsules/<name>` (four files, up to 16 MiB each) and load it as the caller's child |
| 5 | `OP_LIST_INSTALLED` | names under `/capsules/` that have all four files, at most 64 |
| 6 | `OP_PKG_QUERY` | verify a `.nonos` package with the kernel and return its BLAKE3 digest, capability word, tier, name and namespace |
| 7 | `OP_PKG_COMMIT` | re-read and re-verify the package, refuse it unless the digest is the one consented to, then write its four files to the store |
| 8 | `OP_PKG_REMOVE` | delete whichever of an installed name's four files exist |
| 9 | `OP_LOAD_OUTCOME` | the body is a load's own (caps, name, argv); the status is how the sender's last load of that name ended, `ENOENT` when none is kept. The last 8 loads are kept, each told only to its sender, so the Terminal, which waits on a load for 40 ms, still shows a refusal's reason (`server/outcomes.rs`) |

Ops 6, 7 and 8 need a sender holding SpawnWindow, and 2 AppInstall or
SpawnWindow, as the kernel answers `mk_caps_of` for that message (`server/gate.rs`,
S-13b); else `EPERM`. An unknown op or a malformed body is `EINVAL`, as is a
frame shorter than the 8-byte header, under sequence 0. A commit refuses a name
that already holds any artifact or that a running service answers to
(`EEXIST`). The install name is the verified namespace's last part.

## Authority

The capsule may talk to vfs and the payment capsule over IPC and may invoke
`MkCapsuleVerify` and `MkCapsuleLoad`. It has no PCI, MMIO, IRQ, DMA, PIO,
network, display, or focus-routing authority.

## Privacy and persistence

The installer keeps nothing between requests. What it writes lives in the
vfs store: in RAM on an amnesic boot, on the disk store otherwise.

## Limits

The installer checks only that a request pays for one install; the meter
checks every other word against the release's signed commerce block, its
budget and its channels. The installer believes the meter's reply only for its
own call and only as one whole voucher. Until a channel is open (the meter's
OPEN) a paid `OP_INSTALL` answers the meter's refusal.

A paid install's voucher reaches the chain before the install goes on
(v1.4.7): after the meter's PAY, the installer asks nox_settle for a drain
round (SETTLE, op 10) and answers the voucher only when the settle() it
broadcast carries it. ENOENT, an error, or a settle() without it fails the
install. This costs the meter one settle()'s gas per install.
