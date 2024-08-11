package yomitanru

import (
	"gorm.io/gorm"
)

type DictEntry struct {
	ID          uint `gorm:"primaryKey;autoIncrement"`
	Word        string
	Kanas       []string
	Meanings    []string
	SpeechParts []string
}

func InitDB(db *gorm.DB) {
	if db.Migrator().HasTable(&DictEntry{}) {
		db.Migrator().DropTable(&DictEntry{})
	}
	db.AutoMigrate(&DictEntry{})
}

func Insert(db *gorm.DB, el interface{}) {
	entry := el.(DictEntry)
	db.Model(&DictEntry{}).Create(&entry)
}
