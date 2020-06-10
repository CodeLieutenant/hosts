package main

import (
	"fmt"
	_ "github.com/jessevdk/go-flags"
	"github.com/malusev998/hosts/cmd"
	"github.com/malusev998/hosts/host"
	"io/ioutil"
	"os"
)

func handleError(err error) {
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

func main() {
	options, err := cmd.NewOptions()
	handleError(err)

	switch options.Command() {
	case "add":
		file, err := os.OpenFile(options.File, os.O_APPEND, 0777)
		handleError(err)
		defer file.Close()
		p := host.NewParser(file)
		handleError(p.Add(options.Host, options.Ip))
		fmt.Printf("New host added to file: %s %s\n", options.Host, options.Ip)
	case "remove":
		tmp, err := ioutil.TempFile("", "hosts_copy")
		handleError(err)
		file, err := os.OpenFile(options.File, os.O_RDONLY, 0777)
		handleError(err)
		p := host.NewParser(file)
		handleError(p.Remove(tmp, options.Host))
		fmt.Printf("Host removed from file: %s\n", options.Host)
	case "list":
		file, err := os.OpenFile(options.File, os.O_RDONLY, 0777)
		handleError(err)
		defer file.Close()
		p := host.NewParser(file)
		handleError(p.List(func(host, ip string, isComment bool) error {
			fmt.Printf("Host: %s, IP: %s\n", host, ip)
			return nil
		}))
	}
}
