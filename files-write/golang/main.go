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

		fileName := guid.NewString()
		contents := fmt.Sprintf("%s %s","The content of the file is", fileName)

		err := ioutil.WriteFile(fmt.Sprintf("./files/%s", fileName), []byte(contents), 0700)

		if err != nil {
			panic(err)
		}

		w.WriteHeader(http.StatusOK)
		w.Header().Add("Content-Type", "text/plain")
		w.Write([]byte(fileName))

	})

	log.Fatal(http.ListenAndServe(":8080", nil))
}
