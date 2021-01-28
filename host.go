package hosts

import (
	"bufio"
	"fmt"
	"io"
)

type (
	ReadWriteName interface {
		io.ReadWriter
		io.Seeker
		Name() string
		Truncate(int64) error
	}
	Interface interface {
		Remove(tmp ReadWriteName, host string) error
		List(handle HandleHost) error
		Add(host, ip string) error
	}
	Parser struct {
		File ReadWriteName
	}


	HandleHost func(host, ip string, isComment bool) error
)

func (p Parser) Remove(tmp ReadWriteName, host string) error {
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

	if err := iterate(bufio.NewScanner(p.File), handleFunc, true); err != nil {
		return err
	}

	if err := p.File.Truncate(0); err != nil {
		return err
	}

	if _, err := p.File.Seek(0, io.SeekCurrent); err != nil {
		return err
	}

	if _, err := tmp.Seek(0, io.SeekCurrent); err != nil {
		return err
	}

	if _, err := io.Copy(p.File, tmp); err != nil {
		return err
	}

	return nil
}

func (p Parser) List(handle HandleHost) error {
	err := iterate(bufio.NewScanner(p.File), handle, false)
	if err != nil {
		return err
	}

	return nil
}

func (p Parser) Add(host, ip string) error {
	_, err := p.File.Write([]byte(fmt.Sprintf("%s	%s\n", ip, host)))
	if err != nil {
		return err
	}

	return nil
}
