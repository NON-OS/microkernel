/*
 * qwenwl: a Linux key code as the character qwenchat takes. The display
 * server sends evdev codes and xkb modifiers, as Wayland does, with the US
 * keymap (userland/linux_userland/keymap-us.xkb); this is that layout's
 * printable keys, and Enter, Backspace and Escape as qwenchat's controls.
 */
#include "qwenwl_keys.h"

/* xkb's core modifiers: Shift, Lock (Caps), Control, Mod1 (Alt). */
static const uint32_t SHIFT = 1, CAPS = 2, CTRL = 4, ALT = 8;

/* evdev 2 to 53, unshifted then shifted; a space is no character. */
static const char PLAIN[] = "1234567890-=  qwertyuiop[]  asdfghjkl;'` \\zxcvbnm,./";
static const char SHIFTED[] = "!@#$%^&*()_+  QWERTYUIOP{}  ASDFGHJKL:\"~ |ZXCVBNM<>?";

uint32_t wl_char(uint32_t code, uint32_t depressed, uint32_t locked) {
    switch (code) {
    case 1: return 0x1B;          /* Escape */
    case 14: return 0x08;         /* Backspace */
    case 28: case 96: return 0x0D; /* Enter, keypad Enter */
    case 57: return ' ';
    }
    if (depressed & (CTRL | ALT)) return 0;
    if (code < 2 || code > 53) return 0;
    const char plain = PLAIN[code - 2];
    if (plain == ' ') return 0;
    bool shift = depressed & SHIFT;
    /* Caps Lock shifts letters only, and Shift undoes it. */
    if (plain >= 'a' && plain <= 'z' && (locked & CAPS)) shift = !shift;
    return (uint32_t)(unsigned char)(shift ? SHIFTED[code - 2] : plain);
}
