package main

import (
	"fmt"
	"os"
	"path"

	"github.com/cacilhas/nonogram/ui"
	"github.com/spf13/viper"
)

func readSettings() {
	viper.SetConfigName("nonogram")
	viper.SetConfigType("yaml")
	viper.AddConfigPath(path.Join(os.Getenv("HOME"), ".config"))

	if err := viper.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); !ok {
			panic(err)
		}
	}
	defaultSettings()
	viper.Set("homepage", "https://cacilhas.itch.io/nonogram")
	viper.Set("version", "2.7")
}

func defaultSettings() {
	if !viper.IsSet("size") {
		viper.Set("size", 10)
	}
	if !viper.IsSet("easy") {
		viper.Set("easy", false)
	}
	_, width := ui.GetResolution()
	if viper.GetInt("width") == 0 {
		viper.Set("width", width)
	}
	if viper.GetInt("height") == 0 {
		viper.Set("height", width)
	}
	if !viper.IsSet("fullscreen") {
		viper.Set("fullscreen", false)
	}
}

func saveSettings() {
	fmt.Println("saving settings...")
	if err := viper.WriteConfig(); err != nil {
		if err = viper.SafeWriteConfig(); err != nil {
			panic(err)
		}
	}
}
