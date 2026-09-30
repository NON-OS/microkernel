# qwenchat, the conversation guest, for x86_64_v3, x86_64_v2 and x86_64.
# Included by QwenGuest.mk after llama.cpp and the libc pieces are defined.

# The conversation itself: the terminal, or with -ui window a window on the
# desktop through the personality's display server. Same map-then-strip link.
QWENCHAT_SRC := $(wildcard $(LINUX_GUESTS_DIR)/cpp/qwenchat*.cpp $(LINUX_GUESTS_DIR)/cpp/qwenwl_*.cpp \
	$(LINUX_GUESTS_DIR)/cpp/qwenui_*.cpp) $(QWEN_SHARED)
QWENCHAT_HDR := $(wildcard $(LINUX_GUESTS_DIR)/cpp/qwenchat.h $(LINUX_GUESTS_DIR)/cpp/qwenwl*.h \
	$(LINUX_GUESTS_DIR)/cpp/qwenui.h $(LINUX_GUESTS_DIR)/cpp/qwenglyphs*.h) $(QWEN_SHARED_HDR)
$(LINUX_GUESTS_C)/qwenchat: $(QWENCHAT_SRC) $(QWENCHAT_HDR) $(QWEN_LIBS) $(QWEN_LIBC)
	@mkdir -p $(@D)
	@$(QWEN_ZIG)/c++ -static -O2 -g0 -mcpu=$(QWEN_CPU) -std=c++17 \
		-I$(LLAMA_CPP_DIR)/include -I$(LLAMA_CPP_DIR)/ggml/include \
		$(QWENCHAT_SRC) $(QWEN_LIBC) -o $@.full $(QWEN_LIBS) -lpthread
	@strip -o $@ $@.full
$(eval $(call LINUX_GUEST,qwenchat,5102,5103,$(LINUX_GUESTS_C)/qwenchat))
# The same conversation built for another instruction set: $(1) the -mcpu
# level, $(2) the guest and binary name, $(3) and $(4) its ports, $(5) the
# path the personality asks the store for. Each has its own llama.cpp build
# and its own copy of the libc replacements, built for that level as well.
define QWEN_VARIANT
QWEN_BUILD_$(2) := $(LLAMA_CPP_DIR)/build-$(1)
QWEN_LIBS_$(2) := $$(addprefix $$(QWEN_BUILD_$(2))/,src/libllama.a ggml/src/libggml.a \
	ggml/src/libggml-cpu.a ggml/src/libggml-base.a)
$$(QWEN_LIBS_$(2)) &: $(QWEN_ZIG)/.done
	$$(call QWEN_CMAKE,$(1),$$(QWEN_BUILD_$(2)))
$(LINUX_GUESTS_C)/qwenlibc-$(2).o: $(LINUX_GUESTS_DIR)/cpp/qwenlibc.c $(QWEN_ZIG)/.done
	@mkdir -p $$(@D)
	@$(QWEN_ZIG)/cc -c -O2 -mcpu=$(1) -std=c11 -fno-builtin $$< -o $$@
$(LINUX_GUESTS_C)/$(2): $(QWENCHAT_SRC) $(QWENCHAT_HDR) $$(QWEN_LIBS_$(2)) $(LINUX_GUESTS_C)/qwenlibc-$(2).o
	@mkdir -p $$(@D)
	@$(QWEN_ZIG)/c++ -static -O2 -g0 -mcpu=$(1) -std=c++17 \
		-I$(LLAMA_CPP_DIR)/include -I$(LLAMA_CPP_DIR)/ggml/include \
		$(QWENCHAT_SRC) $(LINUX_GUESTS_C)/qwenlibc-$(2).o -o $$@.full $$(QWEN_LIBS_$(2)) -lpthread
	@strip -o $$@ $$@.full
$$(eval $$(call LINUX_GUEST,$(2),$(3),$(4),$(LINUX_GUESTS_C)/$(2),$(5)))
endef

# For x86-64 CPUs without AVX2, FMA, F16C or BMI2, which would stop the v3
# build on an invalid opcode: x86_64_v2 (SSE4.2, POPCNT) runs a 0.5B model
# within a few percent of v3's decode speed on such a chip.
$(eval $(call QWEN_VARIANT,x86_64_v2,qwenchatv2,5104,5105,/bin/qwenchat-x86_64_v2))
# For QEMU's software CPU (TCG), which emulates SSE4 and AVX2 code far more
# slowly than plain x86-64 code. The personality starts it only when CPUID's
# hypervisor leaf says TCG, and only when the store holds it.
$(eval $(call QWEN_VARIANT,x86_64,qwenchatv1,5106,5107,/bin/qwenchat-x86_64))
