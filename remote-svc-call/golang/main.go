package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"strconv"
	"sync"
)

var url = "https://pokeapi.co/api/v2/ability"

type Ability struct {
	Id           int    `json:"id"`
	Name         string `json:"name"`
	IsMainSeries bool   `json:"is_main_series"`
}

type Api struct {
	client  http.Client
	counter int
	mux     sync.Mutex
}

func (api *Api) GetCounter() int {
	api.mux.Lock()
	defer api.mux.Unlock()
	current := api.counter
	api.counter++
	return current
}

func (api *Api) GetAbility(index int) (*Ability, error) {

	res, err := api.client.Get(url + "/" + strconv.Itoa(index))
	defer res.Body.Close()

	if err != nil {
		return nil, err
	}

	bytes, err := ioutil.ReadAll(res.Body)

	if err != nil {
		return nil, err
	}

	ability := Ability{}
	err = json.Unmarshal(bytes, &ability)

	if err != nil {
		return nil, err
	}

	return &ability, nil
}

func (api *Api) Call(w http.ResponseWriter, r *http.Request) {

	counter := api.GetCounter()

	fmt.Println("Executing request to " + strconv.Itoa(counter))
	ability, err := api.GetAbility(counter)

	if err != nil {
		panic(err)
	}

	body, _ := json.Marshal(ability)

	w.WriteHeader(http.StatusOK)
	w.Header().Add("Content-Type", "application/json")
	w.Write(body)
}

func main() {

	api := Api{client: http.Client{}, counter: 1}

	http.HandleFunc("/call", api.Call)

	log.Fatal(http.ListenAndServe(":8080", nil))
}
