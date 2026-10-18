/* qwenwl: the keyboard's events, the US layout, and the repeat of a held key. */
#pragma once
#include "qwenwl.h"

/* A Linux key code under xkb modifiers as qwenchat's character, 0 for none. */
uint32_t wl_char(uint32_t code, uint32_t depressed, uint32_t locked);
long long wl_now_ms();
void wl_key(Wl &w, const std::vector<uint8_t> &body, std::vector<Key> &keys, long long now);
void wl_mods(Wl &w, const std::vector<uint8_t> &body);
void wl_repeat_info(Wl &w, const std::vector<uint8_t> &body);
/* The held key's repeats due by `now`. */
void wl_repeat(Wl &w, std::vector<Key> &keys, long long now);
