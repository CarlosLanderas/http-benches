package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"os"

	"github.com/beevik/guid"
)

func main() {

	if _, err := os.Stat("files"); os.IsNotExist(err) {
		fmt.Println("Creatig files directory")
		os.Mkdir("files", 0700)
	}

	http.HandleFunc("/file", func(w http.ResponseWriter, r *http.Request) {

		ch := make(chan string)

		go func() {

			fileName := guid.NewString()
			contents := []byte("The content of the file is")
			contents = append(contents, fileName...)

			err := ioutil.WriteFile(fmt.Sprintf("./files/%s", fileName), contents, 0700)

			ch <- fileName

			if err != nil {
				panic(err)
			}
		}()

		fileName := <-ch

		w.WriteHeader(http.StatusOK)
		w.Header().Add("Content-Type", "text/plain")
		w.Write([]byte(fileName))

	})

	log.Fatal(http.ListenAndServe(":8080", nil))
}
