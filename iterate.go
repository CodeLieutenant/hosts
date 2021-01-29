package hosts

import (
	"bufio"
	"strings"
)

const bufferSize = 8192

func iterate(scanner *bufio.Scanner, handle HandleHost, includeComments bool) error {
	builders := [2]strings.Builder{}

	builders[0].Grow(15)
	builders[1].Grow(20)

	buf := [bufferSize]byte{}

	scanner.Buffer(buf[:], bufferSize)
	for i := 0; scanner.Scan(); i++ {
		line := strings.TrimSpace(scanner.Text())

		if len(line) == 0 || len(line) == 3 {
			continue
		}

		if strings.HasPrefix(line, "#") {
			if includeComments {
				err := handle(line, "", true)
				if err != nil {
					return err
				}
			}

			continue
		}

		index := 0
		for _, c := range line {
			if c == ' ' || c == '\t' {
				if index != 1 {
					index++
				}
				continue
			}
			builders[index].WriteRune(c)

		}

		ip := builders[0].String()
		h := builders[1].String()

		if err := handle(h, ip, false); err != nil {
			return err
		}

		for i := 0; i < len(builders); i++ {
			builders[i].Reset()
		}
	}
	return nil
}
