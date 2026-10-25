# The riscv64 kernel's build and boot rules, beside the other kernels' in
# mk/20-build.mk.

# The riscv64 kernel, for rustc's own RV64GC bare-metal target. It boots
# under QEMU virt with OpenSBI as far as the VM layer, which is not ported
# yet, and stops there by name; nonos-mk-riscv-boot-check holds it to that.
RISCV_KERNEL := $(TARGET_DIR)/riscv64gc-unknown-none-elf/release/nonos-kernel
.PHONY: nonos-mk-riscv nonos-mk-riscv-check nonos-mk-riscv-boot-check
nonos-mk-riscv: | $(SIGNING_KEY)
	@echo "Building kernel (riscv64, microkernel-core + nonos-arch-preview)..."
	@NONOS_USER_TARGET=riscv64-nonos-user \
		NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --target riscv64gc-unknown-none-elf \
		-Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem \
		--no-default-features --features microkernel-core$(_boot_comma)nonos-arch-preview

nonos-mk-riscv-boot-check: nonos-mk-riscv
	@$(NONOS_PYTHON) scripts/check_riscv64_boot.py --kernel $(RISCV_KERNEL)

nonos-mk-riscv-check: | $(SIGNING_KEY)
	@echo "Type-checking kernel (riscv64, microkernel-core + nonos-arch-preview)..."
	@NONOS_USER_TARGET=riscv64-nonos-user \
		NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) check --release --target riscv64gc-unknown-none-elf \
		-Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem \
		--no-default-features --features microkernel-core$(_boot_comma)nonos-arch-preview
