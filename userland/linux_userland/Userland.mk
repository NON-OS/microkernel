# The Linux userland a production image carries in its store, beside the
# personality's built-in BusyBox: programs built from pinned upstream sources
# by tools/nonos-linux-userland-build, each signed under the one Linux
# userland publisher the key ceremony makes and enrolled under the policy
# root like a capsule, so the personality proves it before any page of it
# runs. A guest holds no capabilities; its endpoints are declared, never
# registered.

LINUX_USERLAND_DIR := userland/linux_userland
LINUX_USERLAND_OUT := $(TARGET_DIR)/linux-userland
LINUX_USERLAND_TOOLS := tools/nonos-linux-userland-build tools/nonos-zig
LINUX_USERLAND_RECIPE = tools/linux-userland/$(1).sh
LINUX_USERLAND_TERMINAL := tools/linux-userland/lib/terminal.sh

$(LINUX_USERLAND_OUT)/lua: $(call LINUX_USERLAND_RECIPE,lua)
$(LINUX_USERLAND_OUT)/zstd: $(call LINUX_USERLAND_RECIPE,zstd)
$(LINUX_USERLAND_OUT)/lua $(LINUX_USERLAND_OUT)/zstd: $(LINUX_USERLAND_TOOLS)
	@echo "Building Linux userland $(@F) from pinned source..."
	@tools/nonos-linux-userland-build $(@F) $(@D)

# The SQLite shell, with ncurses and readline for line editing.
$(LINUX_USERLAND_OUT)/sqlite3: $(LINUX_USERLAND_TOOLS) $(call LINUX_USERLAND_RECIPE,sqlite3) $(LINUX_USERLAND_TERMINAL)
	@echo "Building Linux userland sqlite3 from pinned source..."
	@tools/nonos-linux-userland-build sqlite3 $(@D)

# QuickJS, the interpreter with its REPL compiled in.
$(LINUX_USERLAND_OUT)/qjs: $(LINUX_USERLAND_TOOLS) $(call LINUX_USERLAND_RECIPE,qjs)
	@echo "Building Linux userland qjs from pinned source..."
	@tools/nonos-linux-userland-build qjs $(@D)

# jq, with the oniguruma its release bundles built in.
$(LINUX_USERLAND_OUT)/jq: $(LINUX_USERLAND_TOOLS) $(call LINUX_USERLAND_RECIPE,jq)
	@echo "Building Linux userland jq from pinned source..."
	@tools/nonos-linux-userland-build jq $(@D)

# Tcl's shell, and the script library it reads, compiled in as /usr/lib/tcl8.6.
$(LINUX_USERLAND_OUT)/tclsh: $(LINUX_USERLAND_TOOLS) $(call LINUX_USERLAND_RECIPE,tclsh)
	@echo "Building Linux userland tclsh from pinned source..."
	@tools/nonos-linux-userland-build tclsh $(@D)
LINUX_USERLAND_TCL := init.tcl tclIndex auto.tcl package.tcl tm.tcl history.tcl parray.tcl word.tcl \
	clock.tcl msgcat/msgcat.tcl msgcat/pkgIndex.tcl
$(addprefix $(LINUX_USERLAND_OUT)/tcl8.6/,$(LINUX_USERLAND_TCL)): $(LINUX_USERLAND_OUT)/tclsh
	@test -f $@

# mruby, built by its own minirake from the commit its 3.4.0 tag names.
$(LINUX_USERLAND_OUT)/mruby: $(LINUX_USERLAND_TOOLS) $(call LINUX_USERLAND_RECIPE,mruby) tools/linux-userland/lib/git.sh
	@echo "Building Linux userland mruby from pinned source..."
	@tools/nonos-linux-userland-build mruby $(@D)

# Rust programs from crates.io, each built against the lock it was published
# with (tools/linux-userland/lib/crate.sh). Installed later, not carried.
LINUX_USERLAND_CRATE := tools/linux-userland/lib/crate.sh
$(LINUX_USERLAND_OUT)/rg: $(call LINUX_USERLAND_RECIPE,rg) tools/nix/locks/ripgrep-15.2.0.Cargo.lock
$(LINUX_USERLAND_OUT)/fd: $(call LINUX_USERLAND_RECIPE,fd) tools/nix/locks/fd-find-10.5.0.Cargo.lock
$(LINUX_USERLAND_OUT)/rg $(LINUX_USERLAND_OUT)/fd: $(LINUX_USERLAND_TOOLS) $(LINUX_USERLAND_CRATE)
	@echo "Building Linux userland $(@F) from its pinned crate..."
	@tools/nonos-linux-userland-build $(@F) $(@D)

# gojq, with no cgo, from Go modules pinned file by file.
$(LINUX_USERLAND_OUT)/gojq: $(LINUX_USERLAND_TOOLS) $(call LINUX_USERLAND_RECIPE,gojq)
	@echo "Building Linux userland gojq from pinned modules..."
	@tools/nonos-linux-userland-build gojq $(@D)

# Perl, static, and the trimmed library it reads, compiled in as
# /usr/lib/perl5, carried in the store.
$(LINUX_USERLAND_OUT)/perl: $(LINUX_USERLAND_TOOLS) $(call LINUX_USERLAND_RECIPE,perl) $(LINUX_USERLAND_DIR)/perl-lib.txt
	@echo "Building Linux userland perl from pinned source..."
	@tools/nonos-linux-userland-build perl $(@D)

# GNU nano on ncurses, GNU make, and the OpenSSL command line tool from the
# release Python's ssl module is built on. Installed later, not carried.
$(LINUX_USERLAND_OUT)/nano: $(call LINUX_USERLAND_RECIPE,nano) $(LINUX_USERLAND_TERMINAL)
$(LINUX_USERLAND_OUT)/make: $(call LINUX_USERLAND_RECIPE,make)
$(LINUX_USERLAND_OUT)/openssl: $(call LINUX_USERLAND_RECIPE,openssl)
$(LINUX_USERLAND_OUT)/nano $(LINUX_USERLAND_OUT)/make $(LINUX_USERLAND_OUT)/openssl: $(LINUX_USERLAND_TOOLS)
	@echo "Building Linux userland $(@F) from pinned source..."
	@tools/nonos-linux-userland-build $(@F) $(@D)

# One build makes the auditing tool and the config and word list it reads.
$(LINUX_USERLAND_OUT)/john: $(LINUX_USERLAND_TOOLS) $(call LINUX_USERLAND_RECIPE,john)
	@echo "Building Linux userland john from pinned source..."
	@tools/nonos-linux-userland-build john $(@D)
$(LINUX_USERLAND_OUT)/john.conf $(LINUX_USERLAND_OUT)/password.lst: $(LINUX_USERLAND_OUT)/john
	@test -f $@

# One build makes the Qwen conversation for each instruction set the
# personality chooses between: x86-64-v3, v2, and plain x86-64 for QEMU's
# software CPU.
QWENCHAT_SRC := $(wildcard userland/linux_guests/cpp/qwen*.cpp userland/linux_guests/cpp/qwen*.h \
	userland/linux_guests/cpp/qwenlibc.c)
$(LINUX_USERLAND_OUT)/qwenchat: $(LINUX_USERLAND_TOOLS) $(call LINUX_USERLAND_RECIPE,qwenchat) tools/nonos-cmake $(QWENCHAT_SRC)
	@echo "Building Linux userland qwenchat from pinned llama.cpp..."
	@tools/nonos-linux-userland-build qwenchat $(@D)
$(LINUX_USERLAND_OUT)/qwenchat-x86_64_v2 $(LINUX_USERLAND_OUT)/qwenchat-x86_64: $(LINUX_USERLAND_OUT)/qwenchat
	@test -f $@

# One build makes the interpreter and its standard library together.
$(LINUX_USERLAND_OUT)/python3: $(LINUX_USERLAND_TOOLS) $(call LINUX_USERLAND_RECIPE,python) $(LINUX_USERLAND_TERMINAL) tools/nonos-python-stdlib-zip
	@echo "Building Linux userland CPython from pinned source..."
	@tools/nonos-linux-userland-build python $(@D)
$(LINUX_USERLAND_OUT)/python312.zip $(LINUX_USERLAND_OUT)/cacert.pem: $(LINUX_USERLAND_OUT)/python3
	@test -f $@

# The capsule declaration every program gets: name, service port, reply
# port, ELF. The program is enrolled and signed under the Linux userland
# publisher like a capsule, so the personality proves it before it runs.
define LINUX_USERLAND_CAPSULE
CAPSULE_SLUG             := linux-userland-$(1)
CAPSULE_HANDLE           := linux.userland.$(1)
CAPSULE_DIR              := $(LINUX_USERLAND_DIR)
CAPSULE_BIN_NAME         := userland_$(1)
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_NAMESPACE        := systems.nonos.linux.userland.$(1)
CAPSULE_TARGET           := x86_64-unknown-linux-musl
CAPSULE_SERVICE_ENDPOINT := service:$(2):linux.userland.$(1)
CAPSULE_REPLY_ENDPOINT   := reply:$(3):endpoint.linux.userland.$(1).reply
CAPSULE_REQUIRED_CAPS    := 0x0
CAPSULE_PREBUILT_BIN     := $(4)
CAPSULE_KEY_PUB_PREFIX   := $(NONOS_BAKED_TRUST_DIR)/keys/linux_userland_publisher
CAPSULE_KEY_SEED_PREFIX  := .keys/linux_userland_publisher
CAPSULE_MK_FILE          := $(LINUX_USERLAND_DIR)/Userland.mk
CAPSULE_METADATA         := NØNOS Linux userland $(1)
include nonos-mk/capsule.mk
endef

# The program and the three files that prove it, at its path in the Linux
# tree, as `path=file` pairs.
linux_userland_files = /linux$(2)=$$(linux-userland-$(1)_BIN) \
	/linux$(2).nonos_id_cert.bin=$$(linux-userland-$(1)_CERT) \
	/linux$(2).manifest.bin=$$(linux-userland-$(1)_MANIFEST) \
	/linux$(2).zk_trailer.bin=$$(linux-userland-$(1)_ATTESTATION)

# A program every image carries in its store: name, service port, reply
# port, ELF, its path in the Linux tree.
define LINUX_USERLAND_GUEST
$(call LINUX_USERLAND_CAPSULE,$(1),$(2),$(3),$(4))
LINUX_USERLAND_STORE_DEPS += $$(linux-userland-$(1)_ARTIFACTS)
LINUX_USERLAND_STORE_ENTRIES += $(addprefix --entry ,$(call linux_userland_files,$(1),$(5)))
endef

# A program the image does not carry, installed later from the Marketplace's
# Linux tab: the same arguments. It is enrolled and signed exactly as a
# stored one, so the image's policy root already proves it, and its files,
# with any data a caller adds to LINUX_USERLAND_PACKAGE_<name>, make the
# package the seal publishes (tools/nix/store.json, "packages").
define LINUX_USERLAND_PACKAGE
$(call LINUX_USERLAND_CAPSULE,$(1),$(2),$(3),$(4))
LINUX_USERLAND_PACKAGES += $(1)
LINUX_USERLAND_PACKAGE_DEPS += $$(linux-userland-$(1)_ARTIFACTS)
LINUX_USERLAND_PACKAGE_$(1) += $(call linux_userland_files,$(1),$(5))
endef

$(eval $(call LINUX_USERLAND_GUEST,python3,5150,5151,$(LINUX_USERLAND_OUT)/python3,/usr/bin/python3))
$(eval $(call LINUX_USERLAND_GUEST,lua,5152,5153,$(LINUX_USERLAND_OUT)/lua,/usr/bin/lua))
$(eval $(call LINUX_USERLAND_GUEST,zstd,5154,5155,$(LINUX_USERLAND_OUT)/zstd,/usr/bin/zstd))
$(eval $(call LINUX_USERLAND_GUEST,john,5162,5163,$(LINUX_USERLAND_OUT)/john,/usr/bin/john))
$(eval $(call LINUX_USERLAND_GUEST,sqlite3,5164,5165,$(LINUX_USERLAND_OUT)/sqlite3,/usr/bin/sqlite3))
$(eval $(call LINUX_USERLAND_GUEST,qjs,5166,5167,$(LINUX_USERLAND_OUT)/qjs,/usr/bin/qjs))
$(eval $(call LINUX_USERLAND_GUEST,jq,5168,5169,$(LINUX_USERLAND_OUT)/jq,/usr/bin/jq))
# Installed later from the Marketplace's Linux tab, not carried.
$(eval $(call LINUX_USERLAND_GUEST,tclsh,5170,5171,$(LINUX_USERLAND_OUT)/tclsh,/usr/bin/tclsh))
$(eval $(call LINUX_USERLAND_GUEST,mruby,5172,5173,$(LINUX_USERLAND_OUT)/mruby,/usr/bin/mruby))
$(eval $(call LINUX_USERLAND_GUEST,rg,5174,5175,$(LINUX_USERLAND_OUT)/rg,/usr/bin/rg))
$(eval $(call LINUX_USERLAND_GUEST,fd,5176,5177,$(LINUX_USERLAND_OUT)/fd,/usr/bin/fd))
$(eval $(call LINUX_USERLAND_GUEST,gojq,5178,5179,$(LINUX_USERLAND_OUT)/gojq,/usr/bin/gojq))
$(eval $(call LINUX_USERLAND_GUEST,perl,5180,5181,$(LINUX_USERLAND_OUT)/perl,/usr/bin/perl))
# Perl's library is data in the store, the files perl-lib.txt lists, which
# the recipe writes; nothing in it runs until perl, proved, reads it.
LINUX_USERLAND_PERL_LIB := $(shell grep -v '^\#' $(LINUX_USERLAND_DIR)/perl-lib.txt)
$(addprefix $(LINUX_USERLAND_OUT)/perl5/,$(LINUX_USERLAND_PERL_LIB)): $(LINUX_USERLAND_OUT)/perl
	@test -f $@
LINUX_USERLAND_STORE_DEPS += $(addprefix $(LINUX_USERLAND_OUT)/perl5/,$(LINUX_USERLAND_PERL_LIB))
LINUX_USERLAND_STORE_ENTRIES += $(foreach f,$(LINUX_USERLAND_PERL_LIB),\
	--entry /linux/usr/lib/perl5/$(f)=$(LINUX_USERLAND_OUT)/perl5/$(f))
$(eval $(call LINUX_USERLAND_GUEST,nano,5182,5183,$(LINUX_USERLAND_OUT)/nano,/usr/bin/nano))
$(eval $(call LINUX_USERLAND_GUEST,make,5184,5185,$(LINUX_USERLAND_OUT)/make,/usr/bin/make))
$(eval $(call LINUX_USERLAND_GUEST,openssl,5186,5187,$(LINUX_USERLAND_OUT)/openssl,/usr/bin/openssl))
# At the paths the personality runs a Qwen tier from (install/chat_build.rs).
$(eval $(call LINUX_USERLAND_GUEST,qwenchat,5156,5157,$(LINUX_USERLAND_OUT)/qwenchat,/bin/qwenchat))
$(eval $(call LINUX_USERLAND_GUEST,qwenchatv2,5158,5159,$(LINUX_USERLAND_OUT)/qwenchat-x86_64_v2,/bin/qwenchat-x86_64_v2))
$(eval $(call LINUX_USERLAND_GUEST,qwenchatv1,5160,5161,$(LINUX_USERLAND_OUT)/qwenchat-x86_64,/bin/qwenchat-x86_64))

# Data the programs read and never run, so nothing proves it: the standard
# library the interpreter finds at <prefix>/lib/python312.zip, and the
# lib-dynload directory its getpath looks for beside it, which holds a note.
# cacert.pem is the trust store OpenSSL loads from /etc/ssl/cert.pem, so
# ssl.create_default_context() verifies a TLS peer with no SSL_CERT_FILE set.
LINUX_USERLAND_STORE_DEPS += $(LINUX_USERLAND_OUT)/python312.zip $(LINUX_USERLAND_DIR)/lib-dynload.txt \
	$(LINUX_USERLAND_OUT)/cacert.pem
LINUX_USERLAND_STORE_ENTRIES += \
	--entry /linux/usr/lib/python312.zip=$(LINUX_USERLAND_OUT)/python312.zip \
	--entry /linux/usr/lib/python3.12/lib-dynload/README=$(LINUX_USERLAND_DIR)/lib-dynload.txt \
	--entry /linux/etc/ssl/cert.pem=$(LINUX_USERLAND_OUT)/cacert.pem

# John's config and bundled word list, under the system-wide path its binary
# was built to read. Data: nothing here runs, so nothing proves it. The
# incremental-mode charset files are large and left to a later install.
LINUX_USERLAND_STORE_DEPS += $(LINUX_USERLAND_OUT)/john.conf $(LINUX_USERLAND_OUT)/password.lst
LINUX_USERLAND_STORE_ENTRIES += \
	--entry /linux/usr/share/john/john.conf=$(LINUX_USERLAND_OUT)/john.conf \
	--entry /linux/usr/share/john/password.lst=$(LINUX_USERLAND_OUT)/password.lst

# Tcl's script library, where tclsh was built to find it: init.tcl, which it
# sources as it starts, the scripts its autoloader finds through tclIndex, and
# clock with the msgcat package it loads. Data: tclsh proves itself before
# it reads one.
LINUX_USERLAND_STORE_DEPS += $(addprefix $(LINUX_USERLAND_OUT)/tcl8.6/,$(LINUX_USERLAND_TCL))
LINUX_USERLAND_STORE_ENTRIES += $(foreach f,$(LINUX_USERLAND_TCL),\
	--entry /linux/usr/lib/tcl8.6/$(f)=$(LINUX_USERLAND_OUT)/tcl8.6/$(f))

# Who the guest is. Every guest runs as uid 0, and a program that asks for its
# home by uid (John, through getpwuid, before HOME) needs the answer here:
# /root, one of the family's private directories. Without it John keeps
# ~/.john under the working directory, the read-only shared tree, and stops.
LINUX_USERLAND_STORE_DEPS += $(LINUX_USERLAND_DIR)/passwd
LINUX_USERLAND_STORE_ENTRIES += --entry /linux/etc/passwd=$(LINUX_USERLAND_DIR)/passwd

# The tour: scripts the programs above run, and what to type. Data, so
# nothing proves them; python3 proves itself before it reads one.
LINUX_USERLAND_TOUR := $(addprefix $(LINUX_USERLAND_DIR)/tour/,README mandelbrot.py)
LINUX_USERLAND_STORE_DEPS += $(LINUX_USERLAND_TOUR)
LINUX_USERLAND_STORE_ENTRIES += $(foreach f,$(LINUX_USERLAND_TOUR),\
	--entry /linux/usr/share/nonos/tour/$(notdir $(f))=$(f))

# The other names the programs answer to, in the table the personality
# follows: one `path target` pair a line, committed in nonos-links, which the
# flake reads too. BusyBox's programs need none: the personality runs them
# from its built-in copy by name.
LINUX_USERLAND_STORE_DEPS += $(LINUX_USERLAND_DIR)/nonos-links
LINUX_USERLAND_STORE_ENTRIES += --entry /linux/etc/nonos-links=$(LINUX_USERLAND_DIR)/nonos-links

# The guest test image packs its own Linux tree and link table into the same
# store budget, so it carries these programs enrolled but not stored.
ifeq ($(NONOS_LINUX_GUESTS),1)
LINUX_USERLAND_STORE_DEPS :=
LINUX_USERLAND_STORE_ENTRIES :=
endif
