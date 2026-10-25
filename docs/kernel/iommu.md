# IOMMU

What the NONOS kernel does with DMA remapping hardware, the
[IOMMU](../overview/glossary-hardware-broker.md#iommu), on Intel VT-d and AMD-Vi machines: which
devices it confines, when remapping is in service, and what happens when it is
not.

## Parts

- [What the kernel does; Which IOMMU](iommu-what-the-kernel.md)
- [When VT-d comes into service](iommu-when-vt-d.md)
- [Per capsule domains; When remapping is not in
  service](iommu-per-capsule-domains.md)
- [The posture line; AMD-Vi; Trying it in QEMU](iommu-the-posture-line.md)
- [Tests; See also](iommu-tests.md)
