# capsule_driver_virtio_rng

## Role

`capsule_driver_virtio_rng` is the virtio entropy-device capsule. It owns the
device-facing virtqueue and serves raw entropy bytes over IPC. It does not
mix entropy, stretch entropy, run a CSPRNG, or make cryptographic policy
decisions; those belong to entropy and crypto capsules above it.

```text
kernel (the only sender the endpoint admits)
    |
    | IPC request
    v
driver.virtio_rng -- virtqueue DMA --> virtio-rng device
    |
    `-- IPC reply with transient bytes
```

## Microkernel contract

The capsule is a normal signed user process:

- `MkDeviceList` discovers the virtio RNG device record.
- `MkDeviceClaim` gives this process the device claim.
- `MkPioGrant` takes a transitional device's legacy I/O BAR, or `MkMmioMap`
  maps its register window (legacy MMIO, or the modern structures).
- Completions are polled: `setup::pci` turns the legacy INTx line off and no
  interrupt is bound, so the manifest holds no `Irq`.
- `MkDmaMap` allocates the virtqueue and entropy buffers. The driver never
  unmaps them itself: `MkDeviceRelease` takes them back once bus mastering
  is off.
- `MkIpcRecv` and `MkIpcSend` serve `driver.virtio_rng` on
  `service:4200:driver.virtio_rng`.

The kernel owns scheduling, address-space isolation, capability checks, and
grant revocation. It does not provide user-facing entropy policy from inside
the kernel.

No capsule may send to `driver.virtio_rng`: the kernel holds the endpoint to an
empty list (`src/services/registry/held.rs`), so only the kernel's own sends
reach it. The kernel mirror's client (`src/hardware/virtio_rng_capsule`) has
`fill_random` and `healthcheck`, but at this commit nothing in the kernel calls
them: the kernel's entropy code reads its own in-kernel virtio-rng driver
(`src/drivers/virtio_rng`), which drives the same PCI function outside the
broker.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_FILL_RANDOM` | fill caller buffer from virtio entropy | status plus bytes |
| `OP_HEALTHCHECK` | server liveness | status word |

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Dma`
and `Pio` (`CAPSULE_REQUIRED_CAPS = 0x1B8018`); the driver polls, so no `Irq`. It has no filesystem, network,
graphics, admin, debug, or raw kernel-memory authority.

```text
allowed:   one virtio RNG claim, MMIO or PIO registers, DMA queue, IPC
forbidden: persistent storage, crypto policy, admin control, packet IO
```

## Privacy and persistence

Entropy bytes are sensitive and short-lived. The capsule does not persist
samples, write logs, expose device state to unrelated capsules, or keep a
long-term entropy pool. The DMA buffer is emptied before each request and
again once its bytes are copied out, and the reply buffer once the reply is
sent, so no output stays in the capsule after it is delivered. DMA memory is
revoked on exit.

## Runtime lifecycle

The capsule claims the virtio RNG, sets Interrupt Disable, maps its registers,
resets the device, allocates queue and entropy DMA, turns bus mastering on,
initializes the virtqueue, performs a sanity fill (exit 3 if it fails, exit 4
if every byte is zero), and serves IPC. Teardown resets the device (status 0,
read back by the clock) and then releases the claim; the broker turns bus
mastering off before it tears down the DMA and register grants.

The modern transport is taken whenever the function presents the virtio 1.0
structures it maps, common and notify (virtio-rng has no device
configuration), a transitional 0x1005 beside its legacy I/O BAR included
(`nonos_virtio::modern_caps`): only the modern transport can take
ACCESS_PLATFORM, which is accepted whenever it is offered. A modern-only
0x1044 missing either is refused by name. The legacy registers are left for
a transitional function that does not present them; there the
device's QUEUE_NUM is taken as offered, or refused when the two-page ring
region cannot hold it (above 227 entries). Whether the claim is confined to
an IOMMU domain is the kernel's to enforce at claim time; the capsule does
not guess.

## Failure model

Setup failure aborts the attempt and releases the claim, which takes every
grant with it. Bring-up is tried a bounded number of times with a sleep
between tries (`nonos_libc::bring_up`); the capsule exits 2 when no device is
present and 6 when one is present but never comes up. Fill failure returns an
error and never falls back to a fake software source. Request lengths are
bounded by `MAX_FILL_BYTES`.

One request is in flight at most. A request the device has not answered
within `FILL_MS` (500 ms, by the uptime clock) is answered `E_IO`, and the
queue stalls with it still posted: its buffer is never read and nothing is
posted over it, so a late completion cannot pass for a later request's. The
server then resets the device, releases the claim and brings the device up
again on fresh grants (`server::recover`), or exits 6 if it does not come
back. A used element naming a descriptor that was never posted, or a used
index that moves by more than the request in flight, is handled the same
way.

## Current implemented surface

- Discovers and claims the virtio RNG device.
- Legacy and virtio 1.0 (modern) PCI transports, chosen per function.
- Maps the register BAR by kind (MMIO or PIO) and binds no interrupt.
- Allocates DMA for virtqueue and entropy buffers.
- Initializes the queue and performs a sanity fill.
- Serves fill requests by submitting descriptors and waiting for used-ring
  completion.
- Fails closed if the hardware path cannot be established.

## Wire format

Requests use the `NORD` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte status word. Fill replies return
bounded entropy bytes. `MAX_FILL_BYTES` is 4096.

## State ownership

The capsule owns the virtqueue, entropy DMA buffer, register grant, and device
claim. The entropy service owns pool policy. The crypto capsule owns
cryptographic use of entropy.

## Operating rules

- Never fabricate entropy.
- Never persist samples.
- Bound every fill request.
- Fail closed if broker setup or device completion fails.

## Release target

The finished virtio-rng capsule is a signed entropy-source service with
startup health checks, refill handling, QEMU validation, hardware-equivalent
virtio proof, and strict delivery to the kernel.
It provides source bytes only and never becomes the system CSPRNG or key
generator.

## Release evidence

Release requires QEMU fill validation, entropy-service handoff proof, request-bound
tests, teardown DMA revocation proof, and no fallback path that fabricates
entropy.

## Release checklist

- Signed manifest and kernel mirror present.
- QEMU fill validation passes.
- The kernel consumes the source through IPC (nothing calls the client yet).
- Bounds tests reject oversized requests.
- Teardown proof shows DMA/register/device claim revocation.

## Explicit non-goals today

No entropy mixing, no software fallback RNG, no persistent health telemetry,
no key generation, and no crypto API live in this capsule. Nor do continuous
health tests: the SP 800-90B repetition-count and adaptive-proportion tests
are run on its bytes by the kernel's entropy accumulator (K9, plan/56), which
mixes them in and never credits the capsule alone. The capsule refuses only
an all-zero startup fill.

## Verification

- Build: `make -B nonos-mk-driver-virtio-rng`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: the capsule must stay free of kernel driver imports and
  direct hardware access.
- Documentation check: this README is required by CI and describes authority,
  privacy, current surface, and non-goals.
- Handbook: [drivers](../../docs/handbook/drivers.md) and
  [adding a driver](../../docs/handbook/extending/driver.md), which walks
  through this capsule.

## Real bring-up checklist

Proved on the host, without a device: the transport choice, the capability
parse, feature negotiation, queue programming and the doorbell arithmetic
(`userland/virtio_transport_proofs`), and the legacy handshake and queue
size, the request ring, the fill's deadline, a late completion that is never
served, and the reset-before-release orders (`userland/virtio_rng_proofs`). Only a boot shows the rest. Under QEMU q35,
confirm both ways:

- Without an IOMMU (`-device virtio-rng-pci`, transitional 0x1005): the
  driver binds modern, the sanity fill passes (the capsule does not exit 3 or
  4) and fill requests of different sizes return fresh, different bytes.
- A slow backend (`-object rng-random,id=r,filename=/dev/urandom -device
  virtio-rng-pci,rng=r,max-bytes=1,period=60000`): a fill times out, the
  `driver.virtio_rng: request stalled; resetting the device` line appears,
  and the device comes back.
- With the IOMMU lane (`-device intel-iommu` with `QEMU_IOMMU_OPTS`, and the
  device with `QEMU_IOMMU_VIRTIO`, i.e. `iommu_platform=on,disable-legacy=on`;
  see mk/10-qemu.mk) (modern-only 0x1044): the same, with VERSION_1 and
  ACCESS_PLATFORM negotiated, the device writing entropy through the IOMMU into
  the DMA grant, and no DMA fault in the VT-d log.
- A device that cannot come up: at most seven attempts, one
  `driver.virtio_rng: device present, bring-up failed` line, exit 6. No
  virtio-rng at all: exit 2 at once.
