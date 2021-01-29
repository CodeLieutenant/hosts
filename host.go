package hosts

import (
	"bufio"
	"fmt"
	"io"
)

type (
	// ReadWriteName - Essencial methods for working with hosts File
	// os.File implements all of these methods
	ReadWriteName interface {
		io.ReadWriter
		io.Seeker
		Name() string
		Truncate(int64) error
	}

	// Interface - Parser interface, should be used as parameter to methods and functions
	Interface interface {
		Remove(tmp ReadWriteName, host string) error
		List(handle HandleHost) error
		Add(host, ip string) error
	}

	// Parser - Actual hosts file parser (reader and writer)
	Parser struct {
		File ReadWriteName
	}


	HandleHost func(host, ip string, isComment bool) error
)

// Remove - removes a all instances of host from the file
// Eg.
// 127.0.0.1 hello.test
// 192.168.0.1 hello.test
// Both of the instances will be removed from the file
func (p Parser) Remove(tmp ReadWriteName, host string) error {
	handleFunc := func(h, ip string, isComment bool) error {
		if isComment {
			_, err := tmp.Write([]byte(fmt.Sprintf("%s\n", h)))
			if err != nil {
				return err
			}
			return nil
		}

		if host != h {
			_, err := tmp.Write([]byte(fmt.Sprintf("%s\t%s\n", ip, h)))
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

	if _, err := p.File.Seek(0, io.SeekStart); err != nil {
		return err
	}

	if _, err := tmp.Seek(0, io.SeekStart); err != nil {
		return err
	}

	if _, err := io.Copy(p.File, tmp); err != nil {
		return err
	}

	return nil
}

// List lists a hosts in the file
func (p Parser) List(handle HandleHost) error {
	err := iterate(bufio.NewScanner(p.File), handle, false)
	if err != nil {
		return err
	}

	return nil
}

// Add appends at the end of the file a new entry
// Format of the entry: <ip>\t<host>\n (tab separeted)
// This method always and new line character (without carrage return \r) Unix Style
func (p Parser) Add(host, ip string) error {
	_, err := p.File.Seek(0, io.SeekEnd)

	if err != nil {
		return err
	}

	_, err = p.File.Write([]byte(fmt.Sprintf("%s\t%s\n", ip, host)))

	if err != nil {
		return err
	}

	return nil
}
