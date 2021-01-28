package main

import (
	"context"
	"os"

	"github.com/BrosSquad/hosts/cmd"
	_ "github.com/jessevdk/go-flags"
)

var Version = "dev"

func main() {
	ctx := context.Background()

	if err := cmd.Execute(ctx, Version); err != nil {
		os.Exit(1)
	}
}
