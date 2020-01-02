package main

import (
	"encoding/json"
	"github.com/google/uuid"
	"log"
	"math/rand"
	"net/http"
)

var letter = []rune("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")

type User struct {
	Id    string `json:"id"`
	Email string `json:"email"`
}

func user(w http.ResponseWriter, r *http.Request) {
	id := uuid.New().String()
	email := RandomString(20)

	json, _ := json.Marshal(&User{
		Id:    id,
		Email: email,
	})

	w.Header().Add("Content-Type", "application/json")
	w.Write(json)
}

func main() {
	http.HandleFunc("/user", user)
	log.Fatal(http.ListenAndServe(":8080", nil))
}

func RandomString(n int) string {

	b := make([]rune, n)
	for i := range b {
		b[i] = letter[rand.Intn(len(letter))]
	}
	return string(b)
}
