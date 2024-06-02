package main

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"os"
	"strings"
)

// Define the struct
type JMdictEntry struct {
	Word          string   `json:"word"`
	Reading       string   `json:"reading"`
	PartsOfSpeech []string `json:"parts_of_speech"`
	Meanings      []string `json:"meanings"`
}

// Function to filter entries by part of speech
func filterByPartOfSpeech(entries []JMdictEntry, pos string) []JMdictEntry {
	var filteredEntries []JMdictEntry
	for _, entry := range entries {
		for _, entryPos := range entry.PartsOfSpeech {
			if entryPos == pos {
				filteredEntries = append(filteredEntries, entry)
				break
			}
		}
	}
	return filteredEntries
}

// Function to parse the JSON data
func parseTermsJSON(data []byte) ([]JMdictEntry, error) {
	var rawEntries [][]interface{}
	var termBankEntries []JMdictEntry

	if err := json.Unmarshal(data, &rawEntries); err != nil {
		return nil, err
	}

	for _, entry := range rawEntries {
		var meanings []string
		for _, meaning := range entry[5].([]interface{}) {
			meanings = append(meanings, meaning.(string))
		}

		// Split the part of speech string by spaces to get individual POS values
		pos := strings.Split(entry[2].(string), " ")

		term := JMdictEntry{
			Word:          entry[0].(string),
			Reading:       entry[1].(string),
			PartsOfSpeech: pos,
			Meanings:      meanings,
		}
		termBankEntries = append(termBankEntries, term)
	}

	return termBankEntries, nil
}

func main() {
	// Example JSON data
	filePath := "term_bank_1.json"

	file, err := os.Open(filePath)
	if err != nil {
		log.Fatalf("Error opening file: %v", err)
	}
	defer file.Close()

	// Read the file contents
	data, err := io.ReadAll(file)
	if err != nil {
		log.Fatalf("Error reading file: %v", err)
	}

	// Parse the JSON data
	entries, err := parseTermsJSON(data)
	if err != nil {
		log.Fatalf("Error parsing JSON: %v", err)
	}
	// Filter entries by part of speech
	filteredEntries := filterByPartOfSpeech(entries, "n")

	// Print the parsed entries
	for _, entry := range filteredEntries {
		fmt.Printf("%+v\n", entry)
	}
}
