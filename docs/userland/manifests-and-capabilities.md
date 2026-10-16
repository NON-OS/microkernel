# Manifests and capabilities

A [capsule](../overview/glossary-boot-root-record.md#capsule) states what it is
and what it may do in its `Capsule.mk`; the build turns that into a signed
binary [manifest](../overview/glossary-linux-personality.md#manifest), and the
kernel fixes the capsule's [capability
word](../overview/glossary-boot-root-record.md#capability-word) from it at
spawn.

## Parts

- [Where a manifest comes from](manifests-and-capabilities-where-a-manifest.md)
- [A real example](manifests-and-capabilities-a-real-example.md)
- [The binary format](manifests-and-capabilities-the-binary-format.md)
- [What the kernel checks; Capability
  bits](manifests-and-capabilities-what-the-kernel.md)
- [How the word is fixed](manifests-and-capabilities-how-the-word.md)
- [How the word is fixed, continued; See
  also](manifests-and-capabilities-how-the-word-2.md)
