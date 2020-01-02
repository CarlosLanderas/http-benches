package main

import (
	"log"
	"net/http"
	"strconv"

	"github.com/beevik/guid"
	"github.com/jinzhu/gorm"
	_ "github.com/jinzhu/gorm/dialects/postgres"
)

var db *gorm.DB

type Users struct {
	db *gorm.DB
}

type User struct {
	gorm.Model
	Email     string `gorm:"type:varchar(50);unique;not null"`
	IsEnabled bool   `gorm:"default:false`
}

func (users *Users) add_user(w http.ResponseWriter, r *http.Request) {

	user := User{
		Email:     guid.NewString(),
		IsEnabled: true,
	}

	if err := users.db.Save(&user).Error; err != nil {
		panic(err)
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(strconv.FormatUint(uint64(user.ID), 10)))
}

func main() {

	db, err := gorm.Open("postgres", "user=admin password=example sslmode=disable")

	defer db.Close()

	if err != nil {
		panic(err)
	}

	users := Users{db: db.AutoMigrate(&User{})}

	http.HandleFunc("/user", users.add_user)
	log.Fatal(http.ListenAndServe(":8080", nil))

}
