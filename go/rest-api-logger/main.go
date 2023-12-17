package main

import (
	"gin-api/Config"
)

func main() {

	app := Config.CreateServer()
	Config.SetupRouting(app)

	err := app.Run()
	if err != nil {
		return
	}
}
