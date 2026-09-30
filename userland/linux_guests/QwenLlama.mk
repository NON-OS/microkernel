# llama.cpp at its pinned commit, the zig wrappers that build it, and the
# page-safe libc pieces every Qwen guest links. Included by QwenGuest.mk.

$(LLAMA_CPP_DIR)/.rev:
	@rm -rf $(@D) && git init -q $(@D)
	@git -C $(@D) fetch -q --depth 1 $(LLAMA_CPP_URL) $(LLAMA_CPP_REV) && git -C $(@D) checkout -q FETCH_HEAD
	@test "$$(git -C $(@D) rev-parse HEAD)" = "$(LLAMA_CPP_REV)" || { echo "llama.cpp is not $(LLAMA_CPP_REV)"; exit 1; }
	@echo $(LLAMA_CPP_REV) > $@

# One-word wrappers, since cmake takes a compiler as a single program.
$(QWEN_ZIG)/.done: $(LLAMA_CPP_DIR)/.rev
	@mkdir -p $(@D)
	@for t in cc:cc c++:c++ ar:ar ranlib:ranlib; do \
		n=$${t%%:*}; c=$${t#*:}; \
		case $$c in cc|c++) a="-target x86_64-linux-musl" ;; *) a="" ;; esac; \
		printf '#!/bin/sh\nexec %s %s %s "$$@"\n' "$(ZIG)" "$$c" "$$a" > $(@D)/$$n; chmod +x $(@D)/$$n; \
	done
	@touch $@

# One llama.cpp build per instruction set: $(1) the -mcpu level, $(2) the
# build directory.
define QWEN_CMAKE
	@cmake -S $(LLAMA_CPP_DIR) -B $(2) -DCMAKE_BUILD_TYPE=Release \
		-DCMAKE_C_COMPILER=$(QWEN_ZIG)/cc -DCMAKE_CXX_COMPILER=$(QWEN_ZIG)/c++ \
		-DCMAKE_AR=$(QWEN_ZIG)/ar -DCMAKE_RANLIB=$(QWEN_ZIG)/ranlib \
		-DCMAKE_C_FLAGS="-mcpu=$(1) -g0" -DCMAKE_CXX_FLAGS="-mcpu=$(1) -g0" \
		-DBUILD_SHARED_LIBS=OFF -DGGML_STATIC=ON -DGGML_NATIVE=OFF -DGGML_OPENMP=OFF \
		-DGGML_CCACHE=OFF -DGGML_BACKEND_DL=OFF -DLLAMA_OPENSSL=OFF -DLLAMA_BUILD_COMMON=OFF \
		-DLLAMA_BUILD_TESTS=OFF -DLLAMA_BUILD_TOOLS=OFF -DLLAMA_BUILD_EXAMPLES=OFF \
		-DLLAMA_BUILD_SERVER=OFF -DLLAMA_BUILD_APP=OFF > $(2).log
	@cmake --build $(2) -j4 --target llama >> $(2).log
endef

$(QWEN_LIBS) &: $(QWEN_ZIG)/.done
	$(call QWEN_CMAKE,$(QWEN_CPU),$(QWEN_BUILD))

# zig's strnlen reads up to its bound, not the terminator, and printf asks
# for INT_MAX: a string at the end of a mapping reads the next page. Each
# guest links page-safe replacements; qwenlibc-check proves the difference.
QWEN_LIBC := $(LINUX_GUESTS_C)/qwenlibc.o
$(QWEN_LIBC): $(LINUX_GUESTS_DIR)/cpp/qwenlibc.c $(QWEN_ZIG)/.done
	@mkdir -p $(@D)
	@$(QWEN_ZIG)/cc -c -O2 -mcpu=$(QWEN_CPU) -std=c11 -fno-builtin $< -o $@

.PHONY: nonos-mk-qwenlibc-check
nonos-mk-qwenlibc-check: $(QWEN_LIBC)
	@$(QWEN_ZIG)/cc -static -O2 -mcpu=$(QWEN_CPU) -fno-builtin \
		$(LINUX_GUESTS_DIR)/cpp/qwenlibc_check.c $(QWEN_LIBC) -o $(TARGET_DIR)/qwenlibc-check
	@$(TARGET_DIR)/qwenlibc-check
