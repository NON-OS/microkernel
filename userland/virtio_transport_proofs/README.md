# virtio_transport_proofs

Host proofs for the shared virtio 1.0 PCI transport, `userland/nonos_virtio`.
Each `#[path]` pulls in that crate's shipping source module by module, so the
tests run the code the virtio-net, virtio-blk, virtio-rng and virtio-gpu
capsules boot with, against synthetic config spaces, a broker in host memory
and a model device.

## What it proves

70 `#[test]` functions:

- the capability walk over QEMU's modern-only layout and over hostile ones:
  duplicates, truncated capabilities, I/O and absent BARs, pointer loops,
  pointers out of range, capabilities over the MSI-X table (`caps_tests`,
  `walk_tests`);
- the transport choice, and that the register window it maps for a
  modern-only function is never the MSI-X BAR (`select_tests`, `map_tests`);
- feature negotiation (`features_tests`);
- the bring-up order and queue programming against a model device
  (`bringup_tests`, `queue_tests`);
- the notify arithmetic and the bounds of every register access
  (`notify_tests`, `mmio_tests`).

## What it does not prove

What a real device does with these writes, and interrupt delivery through the
IOMMU's interrupt path, which is the kernel's.

## Run

```sh
cd userland/virtio_transport_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
