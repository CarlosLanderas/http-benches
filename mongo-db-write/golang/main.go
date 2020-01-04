package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/beevik/guid"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

var books *mongo.Collection

type Book struct {
	ID   *primitive.ObjectID `json:"ID" bson:"_id,omitempty"`
	Name string              `json:"name" bson:"name"`
	Sn   string              `json:"sn" bson:"nsname"`
}

func main() {

	client, err := mongo.NewClient(options.Client().ApplyURI("mongodb://root:example@localhost:27017"))

	if err != nil {
		panic(err)
	}

	ctx, _ := context.WithTimeout(context.Background(), 10*time.Second)

	err = client.Connect(ctx)

	if err != nil {
		panic(err)
	}

	books = client.Database("main").Collection("books")

	http.HandleFunc("/books", booksHandler())

	log.Fatal(http.ListenAndServe(":8080", nil))
}

func postBook(w http.ResponseWriter, r *http.Request) {

	id := guid.NewString()
	bookName := fmt.Sprintf("%s %s", "Book", id)

	book := Book{Name: bookName, Sn: id}
	//bson := bson.M{"name": book, "sn": id}
	bson, _ := bson.Marshal(book)

	books.InsertOne(r.Context(), bson)

	w.Header().Add("Content-Type", "application/json")
	w.Write([]byte(id))
}

func getBooks(w http.ResponseWriter, r *http.Request) {

	cursor, err := books.Find(r.Context(), bson.M{})
	defer cursor.Close(r.Context())

	items := make([]Book, 0)

	for cursor.Next(context.Background()) {
		book := Book{}
		err := cursor.Decode(&book)
		if err != nil {
			panic(err)
		}

		items = append(items, book)
	}

	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
	} else {

		body, _ := json.Marshal(items)
		w.Header().Add("Content-Type", "application/json")
		w.Write(body)
	}

}

func booksHandler() http.HandlerFunc {

	return func(w http.ResponseWriter, r *http.Request) {

		if r.Method == "POST" {
			postBook(w, r)
		} else if r.Method == "GET" {
			getBooks(w, r)

		} else {
			http.NotFound(w, r)
		}
	}
}
