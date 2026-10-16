# NONOS documentation

This page is the way into the NONOS documentation: what to read first for what
you want to do, and one line on every page.

NONOS is an operating system for x86_64 computers built on a capability
microkernel in Rust. Drivers, system services and apps run in ring 3 as signed
[capsules](overview/glossary-boot-root-record.md#capsule) that hold only the
[capabilities](overview/glossary-boot-root-record.md#capability) their
[manifests](overview/glossary-linux-personality.md#manifest) grant. It runs on
every core: the kernel starts every CPU the firmware enables and schedules
processes on all of them.

Every page names, in its last line, the commit of the source tree it was checked
against. When a page and the code disagree, the code is what runs. A command
followed by "Not tested in this release." was not run against this release; what
the page says it does is read from the code.

## Parts

- [Where to start](README-where-to-start.md)
- [Every page](README-a-security-reviewer.md)
- [What NONOS is](README-what-nonos-is.md)
- [Using NONOS](README-using-nonos.md)
- [Kernel](README-kernel.md)
- [Drivers](README-drivers.md)
- [Userland](README-userland.md)
- [See also](README-architectures.md)
