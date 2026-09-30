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
# The llama.cpp build is in QwenLlama.mk, the qwenchat guest in QwenChat.mk.
ifeq ($(NONOS_QWEN),1)
LLAMA_CPP_REV := 00af63567a3ac073919be039db8492f229de5ffc
LLAMA_CPP_URL := https://github.com/ggml-org/llama.cpp
LLAMA_CPP_DIR := $(TARGET_DIR)/llama.cpp
QWEN_CPU ?= x86_64_v3
ZIG ?= zig
QWEN_ZIG := $(abspath $(LLAMA_CPP_DIR))/zig-$(QWEN_CPU)
QWEN_BUILD := $(LLAMA_CPP_DIR)/build-$(QWEN_CPU)
QWEN_LIBS := $(addprefix $(QWEN_BUILD)/,src/libllama.a ggml/src/libggml.a ggml/src/libggml-cpu.a ggml/src/libggml-base.a)

include $(LINUX_GUESTS_DIR)/QwenLlama.mk

# Both guests share the compute threads and the memory check.
QWEN_SHARED := $(addprefix $(LINUX_GUESTS_DIR)/cpp/,qwenpool.cpp qwenmem.cpp qwenmem_file.cpp)
QWEN_SHARED_HDR := $(addprefix $(LINUX_GUESTS_DIR)/cpp/,qwenpool.h qwenmem.h qwenmem_file.h)
QWEN_SRC := $(addprefix $(LINUX_GUESTS_DIR)/cpp/,qwencheck.cpp qwencheck_run.cpp qwencheck_report.cpp) \
	$(QWEN_SHARED)
$(LINUX_GUESTS_C)/qwencheck: $(QWEN_SRC) $(LINUX_GUESTS_DIR)/cpp/qwencheck.h $(QWEN_SHARED_HDR) $(QWEN_LIBS) \
		$(QWEN_LIBC)
	@mkdir -p $(@D)
	@# Linked once with its symbol table, kept beside it as the map a stuck
	@# guest's instruction pointer is read against; the store gets a copy
	@# stripped of it, the same code at the same addresses.
	@$(QWEN_ZIG)/c++ -static -O2 -g0 -mcpu=$(QWEN_CPU) -std=c++17 \
		-I$(LLAMA_CPP_DIR)/include -I$(LLAMA_CPP_DIR)/ggml/include \
		$(QWEN_SRC) $(QWEN_LIBC) -o $@.full $(QWEN_LIBS) -lpthread
	@strip -o $@ $@.full
$(eval $(call LINUX_GUEST,qwencheck,5100,5101,$(LINUX_GUESTS_C)/qwencheck))

include $(LINUX_GUESTS_DIR)/QwenChat.mk
LINUX_GUEST_STORE_ENTRIES += --entry /linux/etc/qwen-prompt.txt=$(LINUX_GUESTS_DIR)/etc/qwen-prompt.txt
endif
