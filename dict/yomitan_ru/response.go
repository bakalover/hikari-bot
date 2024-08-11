package yomitanru

import "errors"

type YoResponse struct {
	Entries []DictEntry
}

func (r *YoResponse) HasEntries() bool {
	return len(r.Entries) > 0
}

func (r *YoResponse) RelevantKana() (string, error) {
	if len(r.Entries) < 1 {
		return "", errors.New("no word =(")
	}
	return r.Entries[0].Kanas[0], nil
}

func (r *YoResponse) RelevantWord() (string, error) {
	if len(r.Entries) < 1 {
		return "", errors.New("no word =(")
	}
	return r.Entries[0].Word, nil
}

func (r *YoResponse) Kanas() []string {
	readings := []string{}
	for _, entry := range r.Entries {
		readings = append(readings, entry.Kanas...)
	}
	return readings
}

func (r *YoResponse) Words() []string {
	words := []string{}
	for _, entry := range r.Entries {
		words = append(words, entry.Word)
	}
	return words
}

func (r *YoResponse) RelevantSpeechParts() ([]string, error) {
	if len(r.Entries) < 1 {
		return nil, errors.New("cannot determine speech parts =(")
	}
	return r.Entries[0].SpeechParts, nil
}

func (r *YoResponse) RelevantDefinition() (string, error) {
	if len(r.Entries) < 1 {
		return "", errors.New("no word =(")
	}
	return r.Entries[0].Meanings[0], nil
}
