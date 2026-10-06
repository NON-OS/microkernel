# NONOS documentation

This page is the way into the NONOS documentation: what to read first for what you want to do, and one line on every page.

NONOS is an operating system for x86_64 computers, built on a capability microkernel in Rust, in which drivers, system services and apps run in ring 3 as signed [capsules](overview/glossary.md#capsule) that hold only the capabilities their manifests grant.

Every page names, in its last line, the commit of the source tree it was checked against. When a page and the code disagree, the code is what runs.
