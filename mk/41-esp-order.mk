# The ESP is packed after every other goal on the same command line.
#
# MAKEFLAGS carries -j (mk/00-config.mk), so goals named together build in
# parallel. In `make nonos-mk-smp-prod <store stamp> nonos-mk-esp` the pack
# recipe copied the kernel of the previous run, its staged-kernel check
# passed because the ELF had not been relinked yet, and the kernel linked
# three minutes later was never packed. The ESP kernel then carried the
# capsule policy root of the previous enrolment while the store's trailers
# were proved under the new one, so every program from the store failed its
# attestation.
#
# Order-only, so a goal named beside nonos-mk-esp never makes the pack out of
# date; it only has to finish first. The recipe of nonos-mk-esp re-drives the
# sign, enrol and embed chain in sub-makes, which then see the final ELF.
# Goals that pack through nonos-mk-esp are left out, since as its prerequisites
# they would form a cycle. A sub-make that names nonos-mk-esp alone, as
# nonos_kernel_and_esp does, has nothing else in MAKECMDGOALS and is unchanged.
NONOS_ESP_GOALS := nonos-mk-esp nonos-mk-usb-img nonos-mk-iso
nonos-mk-esp: | $(filter-out $(NONOS_ESP_GOALS),$(MAKECMDGOALS))
