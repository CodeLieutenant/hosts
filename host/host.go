package host

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

type HandleHost func(host, ip string, isComment bool) error

type ReadWriteNameCloser interface {
	io.ReadWriteCloser
	io.Seeker
	Name() string
	Truncate(int64) error
}
type WriteNameCloser interface {
	io.WriteCloser
	Name() string
}

type Parser interface {
	io.Closer
	Remove(tmp ReadWriteNameCloser, host string) error
	List(handle HandleHost) error
	Add(host, ip string) error
}

type parser struct {
	file ReadWriteNameCloser
}

func NewParser(r ReadWriteNameCloser) Parser {
	return parser{file: r}
}

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
			if c == ' ' || c == '	' {
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

func (p parser) Remove(tmp ReadWriteNameCloser, host string) error {

	handleFunc := func(h, ip string, hasComment bool) error {
		if hasComment {
			_, err := tmp.Write([]byte(fmt.Sprintf("%s\n", h)))
			if err != nil {
				return err
			}
			return nil
		}

		if host != h {
			_, err := tmp.Write([]byte(fmt.Sprintf("%s	%s\n", ip, h)))
			if err != nil {
				return err
			}
		}

		return nil
	}

	err := iterate(bufio.NewScanner(p.file), handleFunc, true)

	if err != nil {
		return err
	}

	if err := p.file.Truncate(0); err != nil {
		return err
	}

	if _, err := p.file.Seek(0, os.SEEK_SET); err != nil {
		return err
	}

	tmp.Seek(0, os.SEEK_SET)

	if _, err := io.Copy(p.file, tmp); err != nil {
		return err
	}

	return nil
}

func (p parser) List(handle HandleHost) error {
	err := iterate(bufio.NewScanner(p.file), handle, false)

	if err != nil {
		return err
	}

	return nil
}

func (p parser) Add(host, ip string) error {
	_, err := p.file.Write([]byte(fmt.Sprintf("%s	%s\n", ip, host)))

	if err != nil {
		return err
	}

	return nil
}

func (p parser) Close() error {
	return p.file.Close()
}
