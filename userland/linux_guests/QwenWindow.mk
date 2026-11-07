# The host checks of qwenchat's window (cpp/qwenwl_*.cpp), each over a
# socketpair standing in for the display. Included by QwenChat.mk.

# The window takes the size the display server configures, full screen and
# back (cpp/qwenresize_check.cpp).
QWENWL_SRC := $(addprefix $(LINUX_GUESTS_DIR)/cpp/,qwenwl_loop.cpp qwenwl_keys.cpp qwenwl_keymap.cpp \
	qwenwl_open.cpp qwenwl_resize.cpp qwenwl_wire.cpp qwenwl_event.cpp)
.PHONY: nonos-mk-qwenresize-check
nonos-mk-qwenresize-check:
	@$(CXX) -std=c++17 -Wall -Wextra $(LINUX_GUESTS_DIR)/cpp/qwenresize_check.cpp $(QWENWL_SRC) \
		-o $(TARGET_DIR)/qwenresize-check
	@$(TARGET_DIR)/qwenresize-check

# qwenchat's window presents only the rows that changed: the input box for
# a key, the header for a status tick, else the whole frame
# (cpp/qwenshow_check.cpp).
.PHONY: nonos-mk-qwenshow-check
nonos-mk-qwenshow-check:
	@$(CXX) -std=c++17 -Wall -Wextra $(LINUX_GUESTS_DIR)/cpp/qwenshow_check.cpp $(QWENWL_SRC) \
		$(addprefix $(LINUX_GUESTS_DIR)/cpp/,qwenui_draw.cpp qwenui_text.cpp qwenui_show.cpp) \
		-o $(TARGET_DIR)/qwenshow-check
	@$(TARGET_DIR)/qwenshow-check

# qwenchat's window types what the server's Linux key codes and xkb modifiers
# mean on the US layout, and repeats a held key at repeat_info's rate
# (cpp/qwenkeys_check.cpp).
.PHONY: nonos-mk-qwenkeys-check
nonos-mk-qwenkeys-check:
	@$(CXX) -std=c++17 -Wall -Wextra $(LINUX_GUESTS_DIR)/cpp/qwenkeys_check.cpp $(QWENWL_SRC) \
		-o $(TARGET_DIR)/qwenkeys-check
	@$(TARGET_DIR)/qwenkeys-check
