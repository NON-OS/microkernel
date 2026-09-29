# Qwen, the model the personality pins, run as a guest and checked against
# the host. Included by Guests.mk.
#
# llama.cpp is fetched at one commit and checked to be that commit, then
# built static against musl with zig's C++ toolchain (ZIG: a zig binary,
# or `python3 -m ziglang`). qwencheck reads the model from /models with
# read(), never a mapping, decodes greedily, and exits 0 only when the
# token ids equal the host's; its numbers go to /dev/nonos-metrics.
# QWEN_CPU picks the instruction set, and each has its own host
# reference: the float sums differ between x86_64_v3 and x86_64.
# The lane is opt-in, NONOS_QWEN=1, since it fetches and needs zig.
ifeq ($(NONOS_QWEN),1)
LLAMA_CPP_REV := 00af63567a3ac073919be039db8492f229de5ffc
LLAMA_CPP_URL := https://github.com/ggml-org/llama.cpp
LLAMA_CPP_DIR := $(TARGET_DIR)/llama.cpp
QWEN_CPU ?= x86_64_v3
ZIG ?= zig
QWEN_ZIG := $(abspath $(LLAMA_CPP_DIR))/zig-$(QWEN_CPU)
QWEN_BUILD := $(LLAMA_CPP_DIR)/build-$(QWEN_CPU)
QWEN_LIBS := $(addprefix $(QWEN_BUILD)/,src/libllama.a ggml/src/libggml.a ggml/src/libggml-cpu.a ggml/src/libggml-base.a)

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

$(QWEN_LIBS) &: $(QWEN_ZIG)/.done
	@cmake -S $(LLAMA_CPP_DIR) -B $(QWEN_BUILD) -DCMAKE_BUILD_TYPE=Release \
		-DCMAKE_C_COMPILER=$(QWEN_ZIG)/cc -DCMAKE_CXX_COMPILER=$(QWEN_ZIG)/c++ \
		-DCMAKE_AR=$(QWEN_ZIG)/ar -DCMAKE_RANLIB=$(QWEN_ZIG)/ranlib \
		-DCMAKE_C_FLAGS="-mcpu=$(QWEN_CPU) -g0" -DCMAKE_CXX_FLAGS="-mcpu=$(QWEN_CPU) -g0" \
		-DBUILD_SHARED_LIBS=OFF -DGGML_STATIC=ON -DGGML_NATIVE=OFF -DGGML_OPENMP=OFF \
		-DGGML_CCACHE=OFF -DGGML_BACKEND_DL=OFF -DLLAMA_OPENSSL=OFF -DLLAMA_BUILD_COMMON=OFF \
		-DLLAMA_BUILD_TESTS=OFF -DLLAMA_BUILD_TOOLS=OFF -DLLAMA_BUILD_EXAMPLES=OFF \
		-DLLAMA_BUILD_SERVER=OFF -DLLAMA_BUILD_APP=OFF > $(QWEN_BUILD).log
	@cmake --build $(QWEN_BUILD) -j4 --target llama >> $(QWEN_BUILD).log

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

QWEN_SRC := $(addprefix $(LINUX_GUESTS_DIR)/cpp/,qwencheck.cpp qwencheck_run.cpp qwencheck_report.cpp)
$(LINUX_GUESTS_C)/qwencheck: $(QWEN_SRC) $(LINUX_GUESTS_DIR)/cpp/qwencheck.h $(QWEN_LIBS) $(QWEN_LIBC)
	@mkdir -p $(@D)
	@# Linked once with its symbol table, kept beside it as the map a stuck
	@# guest's instruction pointer is read against; the store gets a copy
	@# stripped of it, the same code at the same addresses.
	@$(QWEN_ZIG)/c++ -static -O2 -g0 -mcpu=$(QWEN_CPU) -std=c++17 \
		-I$(LLAMA_CPP_DIR)/include -I$(LLAMA_CPP_DIR)/ggml/include \
		$(QWEN_SRC) $(QWEN_LIBC) -o $@.full $(QWEN_LIBS) -lpthread
	@strip -o $@ $@.full
$(eval $(call LINUX_GUEST,qwencheck,5100,5101,$(LINUX_GUESTS_C)/qwencheck))

# The conversation itself: the terminal, or with -ui window a window on the
# desktop through the personality's display server. Same map-then-strip link.
QWENCHAT_SRC := $(wildcard $(LINUX_GUESTS_DIR)/cpp/qwenchat*.cpp $(LINUX_GUESTS_DIR)/cpp/qwenwl_*.cpp \
	$(LINUX_GUESTS_DIR)/cpp/qwenui_*.cpp)
QWENCHAT_HDR := $(wildcard $(LINUX_GUESTS_DIR)/cpp/qwenchat.h $(LINUX_GUESTS_DIR)/cpp/qwenwl*.h \
	$(LINUX_GUESTS_DIR)/cpp/qwenui.h $(LINUX_GUESTS_DIR)/cpp/qwenglyphs.h)
$(LINUX_GUESTS_C)/qwenchat: $(QWENCHAT_SRC) $(QWENCHAT_HDR) $(QWEN_LIBS) $(QWEN_LIBC)
	@mkdir -p $(@D)
	@$(QWEN_ZIG)/c++ -static -O2 -g0 -mcpu=$(QWEN_CPU) -std=c++17 \
		-I$(LLAMA_CPP_DIR)/include -I$(LLAMA_CPP_DIR)/ggml/include \
		$(QWENCHAT_SRC) $(QWEN_LIBC) -o $@.full $(QWEN_LIBS) -lpthread
	@strip -o $@ $@.full
$(eval $(call LINUX_GUEST,qwenchat,5102,5103,$(LINUX_GUESTS_C)/qwenchat))
LINUX_GUEST_STORE_ENTRIES += --entry /linux/etc/qwen-prompt.txt=$(LINUX_GUESTS_DIR)/etc/qwen-prompt.txt
endif
