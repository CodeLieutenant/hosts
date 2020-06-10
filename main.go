package main

import (
	"bufio"
	"fmt"
	flag "github.com/jessevdk/go-flags"
	"io/ioutil"
	"os"
	"strings"
)

type HandleHost func(host, ip string, isComment bool) error

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

func listHosts(src string) error {
	f, err := os.OpenFile(src, os.O_RDONLY, 0777)

	if err != nil {
		return err
	}

	handleFunc := func(h, ip string, isComment bool) error {
		fmt.Printf("Host: %s, IP: %s", h, ip)
		return nil
	}

	err = iterate(bufio.NewScanner(f), handleFunc, true)

	if err != nil {
		return err
	}

	if err := f.Close(); err != nil {
		return err
	}

	return nil
}

func appendHost(src, host, ip string) error {
	f, err := os.OpenFile(src, os.O_APPEND, 0777)

	if err != nil {
		return err
	}
	_, err = f.WriteString(fmt.Sprintf("%s	%s", ip, host))

	if err != nil {
		return err
	}

	return nil
}

func removeHost(src, host string) error {
	file, err := ioutil.TempFile("", "hosts_copy")

	if err != nil {
		return err
	}

	f, err := os.OpenFile(src, os.O_RDONLY, 0777)

	if err != nil {
		return err
	}

	handleFunc := func(h, ip string, isComment bool) error {
		if isComment {
			_, err = file.WriteString(fmt.Sprintf("%s\n", h))
			if err != nil {
				return err
			}
			return nil
		}

		if host != h {
			_, err := file.WriteString(fmt.Sprintf("%s	%s\n", ip, h))
			if err != nil {
				return err
			}
		}

		return nil
	}

	err = iterate(bufio.NewScanner(f), handleFunc, true)

	if err := file.Close(); err != nil {
		return err
	}

	if err := f.Close(); err != nil {
		return err
	}

	if err := os.Rename(file.Name(), f.Name()); err != nil {
		return err
	}

	return nil
}

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

func handleError(err error) {
	if err != nil {
		fmt.Println(err.Error())
		os.Exit(1)
	}
}

func main() {
	addOpt := &addOptions{}
	removeOpt := &removeOptions{}
	listOpt := &listOptions{}

	parser := flag.NewNamedParser("Hosts Modifier", flag.Default)

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
		handleError(appendHost(addOpt.File, addOpt.Host, addOpt.Ip))
		fmt.Printf("New host added to file: %s %s\n", addOpt.Host, addOpt.Ip)
	case "remove":
		handleError(removeHost(removeOpt.File, removeOpt.Host))
		fmt.Printf("Host removed from file: %s\n", removeOpt.Host)
	case "list":
		handleError(listHosts(listOpt.File))
	}
}
