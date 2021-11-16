package main

import (
	"fmt"
	"os"
	"path"

	"github.com/spf13/viper"
)

func readSettings() {
	viper.SetConfigName("nonogram")
	viper.SetConfigType("yaml")
	viper.AddConfigPath(path.Join(os.Getenv("HOME"), ".config"))

	if err := viper.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); ok {
			defaultSettings()
		} else {
			panic(err)
		}
	}
}

func defaultSettings() {
	viper.Set("size", 10)
	viper.Set("easy", false)
}

func saveSettings() {
	fmt.Println("saving settings...")
	if err := viper.WriteConfig(); err != nil {
		if err = viper.SafeWriteConfig(); err != nil {
			panic(err)
		}
	}
}
