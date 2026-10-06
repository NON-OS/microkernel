# Update

How to move an installed NONOS to a newer release, what carries over, and what the TPM changes on the way.

## There is no in-place update in 0.9.2

NONOS 0.9.2 has no update service and no A/B system slots. The installer writes a whole disk from the image that is running, and the Marketplace installs programs into the store, never the system itself (`userland/capsule_install/src/install/mod.rs`, `userland/capsule_installer/README.md`). The boot slots that `boot_slots_proofs` tests (`MkBootSlots`) feed the anonymous device proof, and are not an update mechanism (`userland/boot_slots_proofs/README.md`).

To update, you install the newer release over the old one.
