package hosts

import (
	"bufio"
	"strings"
)


func iterate(scanner *bufio.Scanner, handle HandleHost, includeComments bool) error {
	builders := [2]strings.Builder{}
	for i := 0; scanner.Scan(); i++ {
		line := strings.TrimSpace(scanner.Text())
		index := 0

		// Removing first 3 bytes (BOM Bytes)
		if i == 0 && line[0] == 239 && line[1] == 187 && line[2] == 191 {
			line = line[3:]
		}

		if strings.HasPrefix(line, "#") || len(line) == 0 {
			if includeComments {
				err := handle(line, "", true)
				if err != nil {
					return err
				}
			}
			continue
		}

		for _, c := range line {
			if c == ' ' || c == '\t' {
				if index != 1 {
					index++
				}
				continue
			}
			builders[index].WriteRune(c)

		}

		h := builders[1].String()
		ip := builders[0].String()

		if err := handle(h, ip, false); err != nil {
			return err
		}

		for i := 0; i < len(builders); i++ {
			builders[i].Reset()
		}
	}
	return nil
}
