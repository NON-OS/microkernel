# gojq 0.12.19, the pure Go implementation of jq, built with no cgo, so it
# links nothing but Go's own runtime.
#   OUT/gojq
# Go modules come from proxy.golang.org, which publishes no sha256: each
# module zip and go.mod is pinned by the sha256 of the bytes it served. gojq's
# zip is the one whose h1 hash sum.golang.org records,
# h1:ttXA0XCLEMoaLOz5lSeFOZ6u6Q3QxmG46vfgI4O0DEs=, and go holds every
# dependency to the h1 hash in gojq's go.sum before it builds, offline.
# The make lane runs this inside the flake shell, which provides go and unzip.
# Fail clearly up front rather than deep in the module unpack on a host missing
# them.
for t in go unzip; do
	command -v "$t" >/dev/null 2>&1 || { echo "gojq.sh: $t not on PATH (the flake shell provides it)" >&2; exit 1; }
done
modules="$work/gomod/cache/download"
# put MODULE VERSION PIN: place a pinned zip in the module cache, with the
# go.mod inside it beside it, as the module proxy protocol lays them out. The
# cache go fills from it is left writable (-modcacherw), so the work
# directory can be removed.
put() {
	d="$modules/$1/@v"
	mkdir -p "$d"
	cp "$(fetch "$3")" "$d/$2.zip"
	unzip -p "$d/$2.zip" "$1@$2/go.mod" >"$d/$2.mod"
}
put github.com/itchyny/go-yaml v0.0.0-20251001235044-fca9a0999f15 go-yaml
put github.com/itchyny/timefmt-go v0.1.8 timefmt-go
put github.com/mattn/go-isatty v0.0.20 go-isatty
put github.com/mattn/go-runewidth v0.0.19 go-runewidth
put github.com/clipperhouse/stringish v0.1.1 stringish
put github.com/clipperhouse/uax29/v2 v2.3.0 uax29
put golang.org/x/sys v0.38.0 x-sys
# Modules in the graph whose code gojq never compiles: their go.mod alone.
mkdir -p "$modules/github.com/google/go-cmp/@v"
cp "$(fetch go-cmp.mod)" "$modules/github.com/google/go-cmp/@v/v0.7.0.mod"
cp "$(fetch x-sys-0.6.mod)" "$modules/golang.org/x/sys/@v/v0.6.0.mod"
(cd "$work" && unzip -q "$(fetch gojq)")
g="$work/github.com/itchyny/gojq@v0.12.19"
# nixpkgs patches Go's standard library to look for tzdata, mailcap and
# iana-etc at their store paths, and each path it compiles in is in the
# program: gojq carried tzdata's. They name files no guest has, and a store
# path differs from one build machine to another, a Mac's most of all, so
# gojq is built against a copy of the toolchain with Go's own paths back.
goroot="$work/goroot"
cp -R "$(go env GOROOT)/." "$goroot"
chmod -R u+w "$goroot"
store='/nix/store/[0-9a-z]\{32\}-[^/"]*'
upstream() {
	sed -e "$2" "$goroot/src/$1" >"$goroot/src/$1.new" && mv "$goroot/src/$1.new" "$goroot/src/$1"
}
upstream time/zoneinfo_unix.go "\\|^\t\"$store/share/zoneinfo/\",\$|d"
upstream mime/type_unix.go "\\|^\t\"$store/etc/mime.types\",\$|d"
upstream net/lookup_unix.go "s|\"$store/etc/protocols\"|\"/etc/protocols\"|"
upstream net/port_unix.go "s|\"$store/etc/services\"|\"/etc/services\"|"
! grep -n '/nix/store/' "$goroot/src/time/zoneinfo_unix.go" "$goroot/src/mime/type_unix.go" \
	"$goroot/src/net/lookup_unix.go" "$goroot/src/net/port_unix.go" >&2 ||
	{ echo "nonos-linux-userland-build: gojq: Go's library still names the store" >&3; exit 1; }
chmod -R u+w "$work/github.com"
(cd "$g" &&
	GOPATH="$work/gopath" GOMODCACHE="$work/gomod" GOCACHE="$work/gocache" GOENV=off \
	GOFLAGS="-mod=mod -modcacherw" GOPROXY=off GOSUMDB=off GOTOOLCHAIN=local GOWORK=off \
	CGO_ENABLED=0 GOOS=linux GOARCH=amd64 GOAMD64=v1 \
		GOROOT="$goroot" "$goroot/bin/go" build -trimpath -buildvcs=false -ldflags="-s -w -buildid=" -o "$out/gojq" ./cmd/gojq >&2)
