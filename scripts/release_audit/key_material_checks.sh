#!/bin/sh
set -eu

fail=0
hits=target/release-audit/private-key-pem-hits.txt

if git ls-files | grep -E '(^|/)(signing_key|private|secret|seed).*\.(bin|key|pem)$' >/dev/null 2>&1; then
  git ls-files | grep -E '(^|/)(signing_key|private|secret|seed).*\.(bin|key|pem)$'
  fail=1
  printf '%s\n' "FAIL: tracked file names look like private key or seed material"
else
  printf '%s\n' "PASS: no tracked private-key/seed filenames matched the release denylist"
fi

if git ls-files -z | xargs -0 grep -I -n -E -- '-----BEGIN (RSA |EC |OPENSSH |DSA |)PRIVATE KEY-----' > "$hits" 2>/dev/null; then
  fail=1
  printf '%s\n' "FAIL: tracked source contains PEM private key material; see private-key-pem-hits.txt"
else
  printf '%s\n' "PASS: no tracked PEM private keys found"
fi

# DER keys carry no PEM armour, so they are held by name. The devnet's two
# throwaway TLS keys are the only ones the tree may track: public on purpose
# (tools/devnet/tls/README.md), and their root is trusted by no release
# (scripts/check_devnet_feature.py, tools/nix/release_names.nix).
allowed='^tools/devnet/tls/(root|leaf)-key\.throwaway\.der$'
if git ls-files | grep -E -- '(^|[/_-])key[^/]*\.der$' | grep -v -E "$allowed" >/dev/null 2>&1; then
  git ls-files | grep -E -- '(^|[/_-])key[^/]*\.der$' | grep -v -E "$allowed"
  fail=1
  printf '%s\n' "FAIL: tracked DER key files outside the devnet's two throwaway keys"
else
  printf '%s\n' "PASS: the only tracked DER keys are the devnet's throwaway pair:"
  git ls-files | grep -E "$allowed" || true
fi

exit "$fail"
