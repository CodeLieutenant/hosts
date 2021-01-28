package cmd

import (
	"fmt"
	"net"
	"os"

	"github.com/BrosSquad/hosts"

	"github.com/spf13/cobra"
)

func appendCommand() *cobra.Command {
	c := &cobra.Command{
		Use:     "append",
		Short:   "Add new host",
		Long:    "Append new hosts to the end of the `Hosts` file in format <host> <ip>",
		Example: "append hello.test | append hello.test 127.0.0.1",
		Aliases: []string{"add", "a"},
		RunE:    append,
		Args: func(cmd *cobra.Command, args []string) error {
			if len(args) == 0 {
				return fmt.Errorf("Hosts program needs at least an Host")
			}

			if len(args) == 2 {
				ip := net.ParseIP(args[1])

				if ip == nil {
					return fmt.Errorf("IP %s is not valid address", ip.String())
				}
			}

			if len(args) > 2 {
				return fmt.Errorf("Hosts program accepts only 2 arguments. Host and IP.")
			}

			return nil
		},
		ValidArgs: []string{"host", "ip"},
	}

	return c
}

func append(cmd *cobra.Command, args []string) error {
	var host, ip string
	file, err := os.OpenFile(filePath, os.O_RDWR|os.O_APPEND, 0644)
	if err != nil {
		cmd.PrintErrf("Error while opening while (%s): %v\n", filePath, err)
		return err
	}

	defer file.Close()

	p := hosts.Parser{
		File: file,
	}

	ip = Localhost
	host = args[0]

	if len(args) == 2 {
		ip = args[1]
	}

	if err := p.Add(host, ip); err != nil {
		cmd.PrintErrf("Error while appending new host to file %s: %v\n", filePath, err)
		return err
	}

	cmd.Printf("Success! New host has been added to host file: %s %s\n", host, ip)

	return nil
}
