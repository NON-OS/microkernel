# Broker API

The calls a [driver capsule](../overview/glossary.md#driver-capsule) makes to
find a device, claim it, map its registers, get DMA memory, take its interrupts
and let it go, with what the kernel checks at each step.

## Parts

- [Where the calls live](broker-api-where-the-calls.md)
- [Capabilities; The device record](broker-api-capabilities.md)
- [List and claim](broker-api-list-and-claim.md)
- [Configuration space; Registers](broker-api-configuration-space.md)
- [DMA memory](broker-api-dma-memory.md)
- [Interrupts; Release](broker-api-interrupts.md)
- [Errors; Known gaps; See also](broker-api-errors.md)
