/*
 * The character devices, as Go's os package reaches them: /dev/null stats as
 * a character device and reads end of file, /dev/zero reads zeros, and a
 * write to /dev/full fails with ENOSPC.
 */
package main

import (
	"errors"
	"os"
	"syscall"
)

func deviceParts() {
	st, err := os.Stat("/dev/null")
	part(err == nil && st.Mode()&os.ModeCharDevice != 0, "/dev/null stats as a character device")
	null, err := os.ReadFile("/dev/null")
	part(err == nil && len(null) == 0, "/dev/null reads end of file")
	z, err := os.Open("/dev/zero")
	buf := make([]byte, 4096)
	n := 0
	if err == nil {
		n, err = z.Read(buf)
		z.Close()
	}
	zeros := n == len(buf)
	for _, b := range buf {
		zeros = zeros && b == 0
	}
	part(err == nil && zeros, "/dev/zero reads 4096 zeros")
	err = os.WriteFile("/dev/full", []byte("x"), 0)
	part(errors.Is(err, syscall.ENOSPC), "a write to /dev/full fails with ENOSPC")
}
