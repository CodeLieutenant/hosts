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
	Name() string
}
type WriteNameCloser interface {
	io.WriteCloser
	Name() string
}

type Parser interface {
	Remove(tmp WriteNameCloser, host string) error
	List(handle HandleHost) error
	Add(host, ip string) error
}

type parser struct {
	file ReadWriteNameCloser
}

func NewParser(r ReadWriteNameCloser) Parser {
	return &parser{file: r}
}

func iterate(scanner *bufio.Scanner, handle HandleHost, includeComments bool) error {
	builders := [2]strings.Builder{}
	for scanner.Scan() {
		line := strings.Trim(scanner.Text(), "\n \t")
		index := 0

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

func (p *parser) Remove(tmp WriteNameCloser, host string) error {

	handleFunc := func(h, ip string, isComment bool) error {
		if isComment {
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

	if err := p.file.Close(); err != nil {
		return err
	}

	if err := tmp.Close(); err != nil {
		return err
	}

	if err := os.Rename(tmp.Name(), p.file.Name()); err != nil {
		return err
	}

	return nil
}

func (p *parser) List(handle HandleHost) error {
	err := iterate(bufio.NewScanner(p.file), handle, false)

	if err != nil {
		return err
	}

	return nil
}

func (p *parser) Add(host, ip string) error {
	_, err := p.file.Write([]byte(fmt.Sprintf("%s	%s\n", ip, host)))

	if err != nil {
		return err
	}

	return nil
}
