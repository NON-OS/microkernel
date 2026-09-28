package main

import (
	"os"
	"strings"
)

func temp() {
	p := "temp"
	f, err := os.CreateTemp("", "goos-*.txt")
	if !check(p, err == nil && strings.HasPrefix(f.Name(), "/tmp/goos-"), "CreateTemp", err) {
		return
	}
	f.WriteString("temporary")
	f.Close()
	got, err := os.ReadFile(f.Name())
	if !check(p, err == nil && string(got) == "temporary", "ReadFile", got, err) {
		return
	}
	dir, err := os.MkdirTemp("", "goos-dir-*")
	st, _ := os.Stat(dir)
	if !check(p, err == nil && st != nil && st.IsDir() && st.Mode().Perm() == 0o700, "MkdirTemp", err) {
		return
	}
	check(p, os.Remove(f.Name()) == nil && os.Remove(dir) == nil, "Remove")
	_, err = os.Stat(f.Name())
	if !check(p, os.IsNotExist(err), "gone", err) {
		return
	}
	done(p, "CreateTemp and MkdirTemp in /tmp, read back and removed")
}
