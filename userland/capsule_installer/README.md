# capsule_installer

## Role

`capsule_installer` is the userland package install authority, service
`installer`. It verifies signed NONOS packages with the kernel and writes their
four files into the capsule store, removes them, lists what is installed, and
loads a stored capsule through the kernel's verified spawn path for a caller.
For paid listings it asks the `payment` service for a receipt. It owns no
storage device and writes nothing to a disk itself. The handbook page is
[docs/handbook/apps/market-and-store.md](../../docs/handbook/apps/market-and-store.md).
The disk installer is a different capsule, `capsule_install`.

```text
desktop_shell / terminal (nox pkg, exec, install)
        |
        | OP_PKG_QUERY / OP_PKG_COMMIT / OP_LOAD_BY_NAME / ...
        v
capsule_installer -- MkCapsuleVerify, MkCapsuleLoad --> kernel
        |-- store_install / store_uninstall --> vfs_pool
        `-- PAYMENT_OP_PAY --> payment
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
| 2 | `OP_INSTALL` | a free listing gets a zero hash at once; a priced one is sent to `payment` and answered with the receipt's struct hash, or `EAGAIN` when no payment service runs |
| 3 | `OP_LOAD_FROM_STORE` | load a capsule from the ELF, certificate, manifest and trailer bytes in the request, as the installer's own child |
| 4 | `OP_LOAD_BY_NAME` | read `/capsules/<name>` (four files, up to 16 MiB each) and load it as the caller's child |
| 5 | `OP_LIST_INSTALLED` | names under `/capsules/` that have all four files, at most 64 |
| 6 | `OP_PKG_QUERY` | verify a `.nonos` package with the kernel and return its BLAKE3 digest, capability word, tier, name and namespace |
| 7 | `OP_PKG_COMMIT` | re-read and re-verify the package, refuse it unless the digest is the one consented to, then write its four files to the store |
| 8 | `OP_PKG_REMOVE` | delete whichever of an installed name's four files exist |

An unknown op or a malformed body is answered `EINVAL`, and a frame shorter
than the 8-byte header is answered `EINVAL` under sequence 0. A commit
refuses a name that already holds any artifact or that a running service
answers to (`EEXIST`). The install name is the last part of the verified
namespace, never the file name.

## Authority

The capsule may talk to vfs and the payment capsule over IPC and may invoke
`MkCapsuleVerify` and `MkCapsuleLoad`. It has no PCI, MMIO, IRQ, DMA, PIO,
network, display, or focus-routing authority.

## Privacy and persistence

The installer keeps nothing between requests. What it writes lives in the
vfs store: in RAM on an amnesic boot, on the disk store otherwise.

## Limits

No standard image carries `payment`, so a priced `OP_INSTALL` answers
`EAGAIN` on every image. The installer does not check the receipt it relays;
it trusts the payment service's answer.
