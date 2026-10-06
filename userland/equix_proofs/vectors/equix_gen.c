/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0-or-later
 *
 * Writes equix.expect from the fork's own HashX and Equi-X, so the expected
 * values in these proofs come from the reference and not from the port.
 * Run from a checkout of anyone-protocol/ator-protocol (generated at commit
 * 024363a6518018f0a4f8b959b1472d63752222dc), with E=src/ext/equix:
 *
 *   gcc -O2 -DHASHX_SIZE=8 -I$E/include -I$E/hashx/include -I$E/hashx/src \
 *     -I$E/src equix_gen.c $E/src/{context,equix,solver}.c \
 *     $E/hashx/src/{blake2,compiler,compiler_a64,compiler_x86,context}.c \
 *     $E/hashx/src/{hashx,program,program_exec,siphash,siphash_rng}.c \
 *     $E/hashx/src/virtual_memory.c -o equix_gen && ./equix_gen > equix.expect
 *
 * HASHX_SIZE=8 is what the fork's configure.ac sets. The interpreter is
 * forced, which is the only HashX the port has; the compiled one computes
 * the same function.
 *
 * The "pow" lines follow hs_pow.c's hs_pow_solve exactly, except that the
 * first nonce is given instead of drawn at random.
 */
#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <equix.h>
#include <hashx.h>
#include "blake2.h"

static void hex(const void *p, size_t n)
{
  const unsigned char *b = p;
  for (size_t i = 0; i < n; i++)
    printf("%02x", b[i]);
}

static void unhex(const char *s, uint8_t *o, size_t n)
{
  for (size_t i = 0; i < n; i++) {
    unsigned v;
    sscanf(s + 2 * i, "%2x", &v);
    o[i] = (uint8_t)v;
  }
}

static void b2(const char *name, size_t out_len, const char *salt,
               const void *in, size_t len)
{
  blake2b_param P;
  blake2b_state st;
  uint8_t o[64];
  memset(&P, 0, sizeof P);
  P.digest_length = (uint8_t)out_len;
  P.fanout = 1;
  P.depth = 1;
  if (salt)
    memcpy(P.salt, salt, strlen(salt));
  hashx_blake2b_init_param(&st, &P);
  hashx_blake2b_update(&st, in, len);
  hashx_blake2b_final(&st, o, out_len);
  printf("b2 %s %zu %s ", name, out_len, salt ? salt : "-");
  hex(o, out_len);
  printf("\n");
}

static uint32_t effort_hash(const uint8_t *challenge, const uint8_t *sol)
{
  blake2b_param P;
  blake2b_state st;
  uint8_t h[4];
  memset(&P, 0, sizeof P);
  P.digest_length = 4;
  P.fanout = 1;
  P.depth = 1;
  hashx_blake2b_init_param(&st, &P);
  hashx_blake2b_update(&st, challenge, 100);
  hashx_blake2b_update(&st, sol, 16);
  hashx_blake2b_final(&st, h, 4);
  return (uint32_t)h[0] << 24 | (uint32_t)h[1] << 16 | (uint32_t)h[2] << 8 | h[3];
}

int main(void)
{
  uint8_t ramp[300];
  for (int i = 0; i < 300; i++)
    ramp[i] = (uint8_t)i;
  b2("abc", 64, NULL, "abc", 3);
  b2("abc", 4, NULL, "abc", 3);
  b2("empty", 64, "HashX v1", "", 0);
  b2("ramp128", 64, NULL, ramp, 128);
  b2("ramp129", 64, NULL, ramp, 129);
  b2("ramp300", 64, NULL, ramp, 300);
  b2("ramp300", 64, "HashX v1", ramp, 300);

  hashx_ctx *h = hashx_alloc(HASHX_TYPE_INTERPRETED);
  /* The first two seeds are the upstream suite's, terminator included. */
  const char *seeds[] = { "This is a test", "Lorem ipsum dolor sit amet", "",
                          "Tor hs intro v1" };
  const size_t lens[] = { 15, 27, 0, 16 };
  const uint64_t ctrs[] = { 0, 1, 123456, 987654321123456789ULL,
                            0xffffffffffffffffULL };
  for (int s = 0; s < 4; s++) {
    hashx_make(h, seeds[s], lens[s]);
    for (int c = 0; c < 5; c++) {
      unsigned char o[HASHX_SIZE];
      hashx_exec(h, ctrs[c], o);
      printf("hashx ");
      if (lens[s])
        hex(seeds[s], lens[s]);
      else
        printf("-");
      printf(" %llu ", (unsigned long long)ctrs[c]);
      hex(o, HASHX_SIZE);
      printf("\n");
    }
  }
  const char *bad[] = { "\xf8\x05\x00\x00", "\xf9\x05\x00\x00",
                        "\x5d\x93\x02\x00", "\x5e\x93\x02\x00" };
  for (int i = 0; i < 4; i++) {
    printf("seed ");
    hex(bad[i], 4);
    printf(" %s\n", hashx_make(h, bad[i], 4) == HASHX_OK ? "ok" : "fail");
  }

  equix_ctx *e = equix_alloc(EQUIX_CTX_SOLVE);
  for (uint32_t n = 0; n < 32; n++) {
    unsigned char ch[4] = { n, n >> 8, n >> 16, n >> 24 };
    equix_solutions_buffer b;
    printf("solve ");
    hex(ch, 4);
    if (equix_solve(e, ch, 4, &b) != EQUIX_OK) {
      printf(" fail\n");
      continue;
    }
    printf(" %u", b.count);
    for (unsigned k = 0; k < b.count; k++) {
      printf(" ");
      for (int j = 0; j < 8; j++) {
        uint8_t le[2] = { (uint8_t)b.sols[k].idx[j], (uint8_t)(b.sols[k].idx[j] >> 8) };
        hex(le, 2);
      }
    }
    printf("\n");
  }

  static const struct { const char *seed, *id, *nonce; uint32_t effort; } pow[] = {
    { "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
      "1111111111111111111111111111111111111111111111111111111111111111",
      "55555555555555555555555555555555", 1 },
    { "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
      "1111111111111111111111111111111111111111111111111111111111111111",
      "00000000000000000000000000000000", 8 },
    { "86fb0acf4932cda44dbb451282f415479462dd10cb97ff5e7e8e2a53c3767a7f",
      "bfd298428562e530c52bdb36d81a0e293ef4a0e94d787f0f8c0c611f4f9e78ed",
      "ff000000000000000000000000000000", 50 },
    { "86fb0acf4932cda44dbb451282f415479462dd10cb97ff5e7e8e2a53c3767a7f",
      "bfd298428562e530c52bdb36d81a0e293ef4a0e94d787f0f8c0c611f4f9e78ed",
      "ffffffffffffffffffffffffffffffff", 3 },
    { "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
      "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210",
      "01010101010101010101010101010101", 200 },
  };
  for (unsigned k = 0; k < sizeof pow / sizeof pow[0]; k++) {
    uint8_t ch[100], nonce[16];
    memcpy(ch, "Tor hs intro v1\0", 16);
    unhex(pow[k].id, ch + 16, 32);
    unhex(pow[k].seed, ch + 48, 32);
    unhex(pow[k].nonce, nonce, 16);
    memcpy(ch + 80, nonce, 16);
    ch[96] = pow[k].effort >> 24;
    ch[97] = pow[k].effort >> 16;
    ch[98] = pow[k].effort >> 8;
    ch[99] = pow[k].effort;
    unsigned tries = 0;
    for (;;) {
      equix_solutions_buffer b;
      int done = 0;
      tries++;
      if (equix_solve(e, ch, 100, &b) == EQUIX_OK) {
        for (unsigned i = 0; i < b.count && !done; i++) {
          uint8_t s[16];
          for (int j = 0; j < 8; j++) {
            s[2 * j] = (uint8_t)b.sols[i].idx[j];
            s[2 * j + 1] = (uint8_t)(b.sols[i].idx[j] >> 8);
          }
          if ((uint64_t)effort_hash(ch, s) * pow[k].effort <= 0xffffffffULL) {
            printf("pow %s %s %s %u ", pow[k].seed, pow[k].id, pow[k].nonce, pow[k].effort);
            hex(ch + 80, 16);
            printf(" ");
            hex(s, 16);
            printf(" %u\n", tries);
            done = 1;
          }
        }
      }
      if (done)
        break;
      for (int i = 0; i < 16; i++) {
        uint8_t prev = nonce[i];
        if (++nonce[i] > prev)
          break;
      }
      memcpy(ch + 80, nonce, 16);
    }
  }
  return 0;
}
