package main

import (
	"fmt"
	"github.com/jessevdk/go-flags"
	_ "github.com/jessevdk/go-flags"
	"github.com/malusev998/hosts/host"
	"io/ioutil"
	"os"
)

type listOptions struct {
	File string `default:"C:\\Windows\\System32\\drivers\\etc\\host" short:"f" long:"file" required:"false"`
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
		fmt.Println(err)
		os.Exit(1)
	}
}

func main() {
	addOpt := &addOptions{}
	removeOpt := &removeOptions{}
	listOpt := &listOptions{}

	parser := flags.NewNamedParser("host", flags.Default)

	_, err := parser.AddCommand("add", "Adds new entry", "Adds new entry to `host` file", addOpt)
	handleError(err)
	_, err = parser.AddCommand("remove", "Remove single entry", "Removes single host entry in `host` file by dns name", removeOpt)
	handleError(err)
	_, err = parser.AddCommand("list", "List all", "Lists all lines from host file", listOpt)
	handleError(err)

	_, err = parser.Parse()
	handleError(err)

	switch parser.Active.Name {
	case "add":
		file, err := os.OpenFile(addOpt.File, os.O_APPEND, 0777)
		handleError(err)
		defer file.Close()
		p := host.NewParser(file)
		handleError(p.Add(addOpt.Host, addOpt.Ip))
		fmt.Printf("New host added to file: %s %s\n", addOpt.Host, addOpt.Ip)
	case "remove":
		tmp, err := ioutil.TempFile("", "hosts_copy")
		handleError(err)
		file, err := os.OpenFile(removeOpt.File, os.O_RDONLY, 0777)
		handleError(err)
		p := host.NewParser(file)
		handleError(p.Remove(tmp, removeOpt.Host))
		fmt.Printf("Host removed from file: %s\n", removeOpt.Host)
	case "list":
		file, err := os.OpenFile(listOpt.File, os.O_RDONLY, 0777)
		handleError(err)
		defer file.Close()
		p := host.NewParser(file)
		handleError(p.List(func(host, ip string, isComment bool) error {
			fmt.Printf("Host: %s, IP: %s\n", host, ip)
			return nil
		}))
	}
}
