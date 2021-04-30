package cmd

import (
	"io/ioutil"
	"os"

	"github.com/BrosSquad/hosts/v2"
	"github.com/spf13/cobra"
)

func removeCommand(hostPath string) *cobra.Command {
	c := &cobra.Command{
		Use:     "remove",
		Example: "host remove hello.test",
		Short:   "Remove host",
		Long:    "Remove host from `Hosts` file (If there are multiple same hosts, it removes them all)",
		Aliases: []string{"r", "erase"},
		RunE:    remove(hostPath),
		Args:    cobra.ExactArgs(1),
	}

	return c
}

func remove(hostPath string) func(cmd *cobra.Command, args []string) error {
	return func(cmd *cobra.Command, args []string) error {
		tmp, err := ioutil.TempFile("", "hosts_copy")
		if err != nil {
			cmd.PrintErrln("Error while creating a temporary copy file for hosts")
			return err
		}

		defer tmp.Close()

		file, err := os.OpenFile(hostPath, os.O_RDWR, 0644)
		if err != nil {
			cmd.PrintErrf("Error while opening while (%s): %v\n", hostPath, err)
			return err
		}

		defer file.Close()

		p := hosts.Parser{
			File: file,
		}

		if err := p.Remove(tmp, args[0]); err != nil {
			cmd.PrintErrf("Error while removing host %s from the file %s: %v\n", args[0], hostPath, err)
			return err
		}

		cmd.Printf("Host removed from file: %s\n", args[0])
		return nil

	}

}
