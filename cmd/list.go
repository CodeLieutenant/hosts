package cmd

import (
	"fmt"
	"io"
	"os"

	"github.com/BrosSquad/hosts/v2"

	"github.com/spf13/cobra"
)

func listCommand(output io.Writer, hostPath string) *cobra.Command {
	return &cobra.Command{
		Use:     "list",
		Long:    "Lists all hosts from the file",
		Aliases: []string{"l", "p", "print"},
		Example: "hosts list",
		RunE:    list(output, hostPath),
	}
}

func list(output io.Writer, hostPath string) func(*cobra.Command, []string) error {
	return func(cmd *cobra.Command, args []string) error {
		file, err := os.OpenFile(hostPath, os.O_RDONLY, 0644)
		if err != nil {
			cmd.PrintErrf("Error while opening while (%s): %v\n", hostPath, err)
			return err
		}

		defer file.Close()

		p := hosts.Parser{
			File: file,
		}

		err = p.List(func(host, ip string, isComment bool) error {
			fmt.Fprintf(output, "Host: %s, IP: %s\n", host, ip)
			return nil
		})

		if err != nil {
			cmd.PrintErrf("Error while printing the hosts: %v\n", err)
		}

		return nil
	}

}
