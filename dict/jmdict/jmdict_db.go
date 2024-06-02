package jmdict

import (
	"strings"

	"gorm.io/gorm"
)

type Term struct {
	ID            uint   `gorm:"primaryKey"`
	Word          string `gorm:"type:varchar(255)"`
	Reading       string `gorm:"type:varchar(255)"`
	PartsOfSpeech string `gorm:"type:varchar(255)"`
	Meanings      string `gorm:"type:text"`
}

func saveToDatabase(entries []JMdictEntry, db *gorm.DB) error {
	for _, entry := range entries {
		term := Term{
			Word:          entry.Word,
			Reading:       entry.Reading,
			PartsOfSpeech: strings.Join(entry.PartsOfSpeech, ","),
			Meanings:      strings.Join(entry.Meanings, ";"),
		}
		if err := db.Create(&term).Error; err != nil {
			return err
		}
	}
	return nil
}

func InitJMdictTable(db *gorm.DB) error {
	if err := db.AutoMigrate(&Term{}); err != nil {
		return err
	}
	return nil
}
