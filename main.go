package main

import (
	"bufio"
	"fmt"
	flag "github.com/jessevdk/go-flags"
	"io"
	"io/ioutil"
	"os"
	"strings"
)

type HandleHost func(host, ip string, isComment bool) error

type listOptions struct {
	File string `default:"C:\\Windows\\System32\\drivers\\etc\\hosts" short:"f" long:"file" required:"false"`
}

type removeOptions struct {
	listOptions
	Host string `required:"true" short:"p" long:"host"`
}

type addOptions struct {
	removeOptions
	Ip string `required:"true" short:"i" long:"ip"`
}

type writeNameCloser interface {
	io.WriteCloser
	Name() string
}

type readNameCloser interface {
	io.ReadCloser
	Name() string
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

func listHosts(reader io.Reader) error {
	handleFunc := func(h, ip string, isComment bool) error {
		fmt.Printf("Host: %s, IP: %s\n", h, ip)
		return nil
	}

	err := iterate(bufio.NewScanner(reader), handleFunc, false)

	if err != nil {
		return err
	}

	return nil
}

func appendHost(writer io.Writer, host, ip string) error {
	_, err := writer.Write([]byte(fmt.Sprintf("%s	%s\n", ip, host)))

	if err != nil {
		return err
	}

	return nil
}

func removeHost(rc readNameCloser, tmp writeNameCloser, host string) error {

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

	err := iterate(bufio.NewScanner(rc), handleFunc, true)

	if err != nil {
		return err
	}

	if err := rc.Close(); err != nil {
		return err
	}

	if err := tmp.Close(); err != nil {
		return err
	}

	if err := os.Rename(tmp.Name(), rc.Name()); err != nil {
		return err
	}

	return nil
}

func handleError(err error) {
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

func main() {
	addOpt := &addOptions{}
	removeOpt := &removeOptions{}
	listOpt := &listOptions{}

	parser := flag.NewNamedParser("hosts", flag.Default)

	_, err := parser.AddCommand("add", "Adds new entry", "Adds new entry to `hosts` file", addOpt)
	handleError(err)
	_, err = parser.AddCommand("remove", "Remove single entry", "Removes single host entry in `hosts` file by dns name", removeOpt)
	handleError(err)
	_, err = parser.AddCommand("list", "List all", "Lists all lines from hosts file", listOpt)
	handleError(err)

	_, err = parser.Parse()
	handleError(err)

	switch parser.Active.Name {
	case "add":
		f, err := os.OpenFile(addOpt.File, os.O_APPEND, 0777)
		handleError(err)
		defer f.Close()
		handleError(appendHost(f, addOpt.Host, addOpt.Ip))
		fmt.Printf("New host added to file: %s %s\n", addOpt.Host, addOpt.Ip)
	case "remove":
		tmp, err := ioutil.TempFile("", "hosts_copy")
		handleError(err)
		file, err := os.OpenFile(removeOpt.File, os.O_RDONLY, 0777)
		handleError(err)
		handleError(removeHost(file, tmp, removeOpt.Host))
		fmt.Printf("Host removed from file: %s\n", removeOpt.Host)
	case "list":
		f, err := os.OpenFile(listOpt.File, os.O_RDONLY, 0777)
		handleError(err)
		defer f.Close()
		handleError(listHosts(f))
	}
}
