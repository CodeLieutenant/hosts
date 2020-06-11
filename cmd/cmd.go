package cmd

import "github.com/jessevdk/go-flags"

type ListOptions struct {
	File string `default:"C:\\Windows\\System32\\drivers\\etc\\hosts" short:"f" long:"file" required:"false"`
}

type RemoveOptions struct {
	ListOptions
	Host string `required:"true" short:"p" long:"host"`
}
type AddOptions struct {
	RemoveOptions
	Ip string `required:"true" short:"i" long:"ip"`
}

type Options struct {
	AddOptions    *AddOptions
	RemoveOptions *RemoveOptions
	ListOptions   *ListOptions
	command       string
}

func NewOptions() (*Options, error) {
	addOptions := &AddOptions{}
	listOptions := &ListOptions{}
	removeOptions := &RemoveOptions{}
	options := &Options{
		AddOptions:    addOptions,
		RemoveOptions: removeOptions,
		ListOptions:   listOptions,
	}
	parser := flags.NewNamedParser("host", flags.HelpFlag|flags.PassDoubleDash)

	_, err := parser.AddCommand("add", "Adds new entry", "Adds new entry to `host` file", addOptions)

	if err != nil {
		return nil, err
	}

	_, err = parser.AddCommand("remove", "Remove single entry", "Removes single host entry in `host` file by dns name", removeOptions)

	if err != nil {
		return nil, err
	}

	_, err = parser.AddCommand("list", "List all", "Lists all lines from host file", listOptions)

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
