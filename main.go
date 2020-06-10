package main

import (
	"bufio"
	"flag"
	"fmt"
	"io/ioutil"
	"log"
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

func main() {

	args := os.Args[1:]

	if len(args) < 1 {
		log.Fatal("Action is not supplied")
	}

	action := args[0]

	file := flag.String("file", "C:\\Windows\\System32\\drivers\\etc\\hosts", "Host file")
	ip := flag.String("ip", "127.0.0.1", "ip address")
	host := flag.String("host", "", "Host")

	flag.Parse()

	switch strings.ToLower(action) {
	case "add":
		if *host == "" || *ip == "" {
			log.Fatal("Provide host and ip flags")
		}

		if err := appendHost(*file, *host, *ip); err != nil {
			log.Fatal(err)
		}
		fmt.Printf("New host added to file: %s %s\n", *host, *ip)

	case "remove":
		if *host == "" {
			log.Fatal("Host is not supplied")
		}
		if err := removeHost(*file, *host); err != nil {
			log.Fatal(err)
		}
		fmt.Printf("Host removed from file: %s %s\n", *host, *ip)

	case "list":
		if err := listHosts(*file); err != nil {
			log.Fatal(err)
		}
	default:
		log.Fatal("Action needs to ADD|REMOVE|LIST")
	}
}
