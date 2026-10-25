# iort_proofs

Host tests for the aarch64 IORT reader, mounted by `#[path]` from
`src/arch/aarch64/iort/`, with only the record types it fills
(`SmmuInfo`, `IommuMap`, `MsiController`) mounted from `src/arch/fdt/find/`.

- Whole tables read into records: a root complex behind an SMMUv3 (QEMU
  virt's shape) and root complexes sent straight to an ITS group.
- Composition through an SMMU whose mappings split the stream ids, checked
  requester id by requester id, with the SMMU's own MSI mapping left out.
- Refusals: mappings that name no node or wrap, single mappings on a root
  complex, two SMMUs or two ITS groups for one root complex, more entries or
  root complexes than the records hold, an ITS MADT does not name.
- Every truncation and every byte at seven hostile values: refused or read,
  never a panic or an overflow.

Run with `RUST_TEST_THREADS=1 cargo test --release --config
profile.release.overflow-checks=true` from this directory.
