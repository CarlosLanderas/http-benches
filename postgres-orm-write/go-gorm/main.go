package main

import (
	"log"
	"net/http"
	"strconv"
	"time"

	"github.com/beevik/guid"
	"github.com/jinzhu/gorm"
	_ "github.com/jinzhu/gorm/dialects/postgres"
)

var db *gorm.DB

type User struct {
	gorm.Model
	Email     string `gorm:"type:varchar(50);unique;not null"`
	IsEnabled bool   `gorm:"default:false`
}

func add_user(w http.ResponseWriter, r *http.Request) {

	user := User{
		Email:     guid.NewString(),
		IsEnabled: true,
	}

	if err := db.Save(&user).Error; err != nil {
		panic(err)
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(strconv.FormatUint(uint64(user.ID), 10)))
}

func main() {

	var err error

	db, err = gorm.Open("postgres", "user=admin password=example sslmode=disable")
	db.DB().SetMaxOpenConns(90)
	db.DB().SetMaxIdleConns(5)
	db.DB().SetConnMaxLifetime(time.Minute)

	defer db.Close()

	if err != nil {
		panic(err)
	}

	db.AutoMigrate(&User{})

	http.HandleFunc("/user", add_user)
	log.Fatal(http.ListenAndServe(":8080", nil))

}
