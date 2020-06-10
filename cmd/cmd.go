package cmd

import "github.com/jessevdk/go-flags"

type Options struct {
	File    string `default:"C:\\Windows\\System32\\drivers\\etc\\hosts" short:"f" long:"file" required:"false"`
	Host    string `required:"true" short:"p" long:"host"`
	Ip      string `required:"true" short:"i" long:"ip"`
	command string
}

func NewOptions() (*Options, error) {
	options := &Options{}
	parser := flags.NewNamedParser("host", flags.Default)

	_, err := parser.AddCommand("add", "Adds new entry", "Adds new entry to `host` file", options)

	if err != nil {
		return nil, err
	}

	_, err = parser.AddCommand("remove", "Remove single entry", "Removes single host entry in `host` file by dns name", options)

	if err != nil {
		return nil, err
	}

	_, err = parser.AddCommand("list", "List all", "Lists all lines from host file", options)

	if err != nil {
		return nil, err
	}

	_, err = parser.Parse()

	if err != nil {
		return nil, err
	}
	options.command = parser.Active.Name

	return options, nil
}

func (o *Options) Command() string {
	return o.command
}
