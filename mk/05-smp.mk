# Every kernel the nonos-mk lanes build starts every CPU the firmware lists,
# as every profile in nonos.toml does (smp = true), and every QEMU lane boots
# QEMU_SMP CPUs (10-qemu.mk). NONOS_SMP=0 builds the one-CPU kernel, for a
# lane that has to show that path; nothing ships so. The check is
# nonos-ci/check-smp-default.py.

NONOS_SMP ?= 1
NONOS_SMP_FEATURE := $(if $(filter 0,$(NONOS_SMP)),,$(_boot_comma)nonos-smp)
