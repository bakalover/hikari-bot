package yomitanru

import (
	"bakalover/hikari-bot/dict"
	"encoding/json"
	"fmt"
	"io"
	"os"

	"gorm.io/gorm"
)

type Yomitan struct {
	DbConn *gorm.DB
}

const (
	DictPath = "./dict"
)

func NewYomitan(DbConn *gorm.DB) *Yomitan {
	InitDB(DbConn)
	InitDict(DictPath)
	return &Yomitan{DbConn}
}

func (yomitan *Yomitan) Search(key string) (dict.Response, error) {
	res := []DictEntry{}
	yomitan.DbConn.Model(&DictEntry{}).Where("word = ?", key).Find(&res)
	return &YoResponse{res}, nil
}

func (yomitan *Yomitan) NounRepr() string {
	return "n"
}

func (yomitan *Yomitan) Repr() string {
	return "Yomitan"
}

func InitDict(dictPath string) {
	items, _ := os.ReadDir(dictPath)
	for _, item := range items {
		elements := ReadJson(item.Name())
		for el := range elements.(map[string]interface{}) {
			fmt.Println(el)
		}
	}
}

func ReadJson(filename string) interface{} {
	jsonFile, err := os.Open(filename)
	if err != nil {
		fmt.Println(err)
	}
	defer jsonFile.Close()

	byteValue, _ := io.ReadAll(jsonFile)

	var result map[string]interface{}
	json.Unmarshal([]byte(byteValue), &result)
	return result
}
