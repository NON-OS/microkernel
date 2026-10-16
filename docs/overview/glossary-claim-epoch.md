# Glossary: Claim epoch

Part of [Glossary](glossary.md).

## Claim epoch

The number `MkDeviceClaim` returns, taken from one counter that starts at 1 and
grows with every claim. The calls that map MMIO, bind an interrupt, take a DMA
buffer or a port grant, or read or write PCI configuration on that device must
pass it back, and a call with an old one fails with ESTALE, -116. Explained in
[Broker ABI](../abi/broker.md). Code: `src/hardware/broker/claim/state.rs`.

## Correlation token

The nonzero number the kernel gives each `MkIpcCall` from a counter. Only a
reply that carries the same number is delivered to the caller; a plain send
carries 0, so it cannot pass as a reply. Explained in
[IPC](../kernel/ipc.md#blocking-waking-and-timeouts). Code:
`src/syscall/microkernel/ipc/call/sys_ipc_call.rs`.

## Data volume

The encrypted volume the disk plan places at sector 262,144 or above, which
holds imported files such as the Qwen models; each 512-byte sector is sealed on
its own with ChaCha20-Poly1305. On an installed disk its key is derived from the
TPM under the label `blockfs.data.v1`; the kernel can also key it with a
passphrase, but no capsule asks for that in this release. On a live stick the
volume is held in RAM under a key drawn for that boot and is gone at power off.
Explained in [Device secrets and
keys](../security/device-secrets-and-keys.md#the-data-volume). Code:
`src/fs/blockfs_volume/open_machine.rs`, `src/fs/blockfs_volume/session.rs`.

## Development image

A profile's development twin, `<profile>-dev`: the same kernel features,
path-only attestation and the `dev-qemu` loader policy, sealed with throwaway
keys by `make dev-image` in a copy of the checkout under `target/dev/tree`. Its
gates take a capsule on its Merkle path without the STARK proof, and the seal
refuses it for a release. Explained in [The
seal](../build/seal.md#without-the-release-keys). Code: `tools/nonos-dev-image`.

## Device secret

Four field words the TPM derives as the witness of the anonymous device proof,
under a policy the release approves over PCR 9 and this machine's PCRs 0, 4 and
7. It is never stored; a firmware or loader change gives a new one, while a
kernel update the release approved keeps it. In this release only `app.prove`
holds the `DeviceSecret` capability that receives it. Explained in [Device
secrets and keys](../security/device-secrets-and-keys.md#the-device-secret).
Code: `src/security/tpm/device_secret/mod.rs`.

## Directmap

The kernel's linear, never executable mapping of the first 256 GiB of physical
memory at 0xFFFF_8000_0000_0000, PML4 slot 256. The bootloader builds it, and
the kernel reaches page tables, user frames and the handoff through it.
Explained in [Memory and paging](../kernel/memory-and-paging.md). Code:
`src/memory/layout/constants/regions.rs`.

## Disk plan

One plain sector at LBA 245,760, with the magic `NONOSDP1`, that names the data
volume's range and up to 30 files to import. The kernel checks every range in it
against the disk and against each other before it uses any; a live stick's plan
names no volume. Explained in [Storage
drivers](../drivers/storage/README.md#the-disk-layout). Code:
`src/fs/blockfs_volume/plan_types.rs`.
