# qwenchat, the conversation guest, for x86_64_v3 and x86_64_v2. Included by
# QwenGuest.mk after llama.cpp and the libc pieces are defined.

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
# The same conversation for x86-64 CPUs without AVX2, FMA, F16C or BMI2,
# which would stop the v3 build on an invalid opcode: x86_64_v2 (SSE4.2,
# POPCNT) runs a 0.5B model within a few percent of v3's decode speed. The
# personality starts it when CPUID lacks any of those; its own llama.cpp
# build and its own copy of the libc replacements, built for v2 as well.
QWEN_CPU2 := x86_64_v2
QWEN_BUILD2 := $(LLAMA_CPP_DIR)/build-$(QWEN_CPU2)
QWEN_LIBS2 := $(addprefix $(QWEN_BUILD2)/,src/libllama.a ggml/src/libggml.a ggml/src/libggml-cpu.a \
	ggml/src/libggml-base.a)
$(QWEN_LIBS2) &: $(QWEN_ZIG)/.done
	$(call QWEN_CMAKE,$(QWEN_CPU2),$(QWEN_BUILD2))
QWEN_LIBC2 := $(LINUX_GUESTS_C)/qwenlibc-v2.o
$(QWEN_LIBC2): $(LINUX_GUESTS_DIR)/cpp/qwenlibc.c $(QWEN_ZIG)/.done
	@mkdir -p $(@D)
	@$(QWEN_ZIG)/cc -c -O2 -mcpu=$(QWEN_CPU2) -std=c11 -fno-builtin $< -o $@
$(LINUX_GUESTS_C)/qwenchat-v2: $(QWENCHAT_SRC) $(QWENCHAT_HDR) $(QWEN_LIBS2) $(QWEN_LIBC2)
	@mkdir -p $(@D)
	@$(QWEN_ZIG)/c++ -static -O2 -g0 -mcpu=$(QWEN_CPU2) -std=c++17 \
		-I$(LLAMA_CPP_DIR)/include -I$(LLAMA_CPP_DIR)/ggml/include \
		$(QWENCHAT_SRC) $(QWEN_LIBC2) -o $@.full $(QWEN_LIBS2) -lpthread
	@strip -o $@ $@.full
$(eval $(call LINUX_GUEST,qwenchatv2,5104,5105,$(LINUX_GUESTS_C)/qwenchat-v2,/bin/qwenchat-x86_64_v2))
