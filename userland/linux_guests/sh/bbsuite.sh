# bbsuite: busybox runs the applets of bbsuite-body.sh on NONOS, and its
# output, less the lines that start with @ (they name a time or a pid), must
# equal what the same busybox printed on the host (bbsuite.expect).
# usage: sh /etc/bbsuite.sh
out=/tmp/bbsuite.out
sh /etc/bbsuite-body.sh > "$out" 2>&1
grep -v '^@' "$out" > /tmp/bbsuite.got
grep -v '^@' /etc/bbsuite.expect > /tmp/bbsuite.want
lines=$(wc -l < /tmp/bbsuite.want)
sections=$(grep -c '^== ' /tmp/bbsuite.want)
got=$(sha256sum < /tmp/bbsuite.got | cut -c1-16)
want=$(sha256sum < /tmp/bbsuite.want | cut -c1-16)
if cmp -s /tmp/bbsuite.got /tmp/bbsuite.want; then
	echo "[C] bbsuite PASS: $lines lines in $sections sections equal the host's, sha256 $got"
	exit 0
fi
# What differs, then everything this run printed, for a diff on the host.
differ=$(diff -U0 /tmp/bbsuite.want /tmp/bbsuite.got | grep -v '^---\|^+++' | grep -c '^[-+]')
sed 's/^/[C] bbsuite got: /' /tmp/bbsuite.got
echo "[C] bbsuite FAIL: $differ lines differ of $lines, sha256 $got, host $want"
exit 1
