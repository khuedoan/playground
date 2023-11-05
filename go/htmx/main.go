package main

import (
	"database/sql"
	"html/template"
	"log"
	"net/http"

	_ "github.com/mattn/go-sqlite3"
)

type TodoStatus int

const (
	TodoOpen       TodoStatus = 0
	TodoInProgress TodoStatus = 1
	TodoDone       TodoStatus = 2
)

type Todo struct {
	Id     int
	Title  string
	Status TodoStatus
}

func index(w http.ResponseWriter, r *http.Request) {
	db, err := sql.Open("sqlite3", "./data/sqlite.db")
	if err != nil {
		log.Fatal(err)
	}

	rows, err := db.Query("SELECT * FROM todos")
	if err != nil {
		log.Fatal(err)
	}
	defer rows.Close()

	todos := []Todo{}
	for rows.Next() {
		todo := Todo{}
		rows.Scan(&todo.Id, &todo.Title, &todo.Status)
		todos = append(todos, todo)
	}

	tmpl := template.Must(template.ParseFiles("templates/index.html"))
	tmpl.Execute(w, todos)
}

func main() {
	http.HandleFunc("/", index)
	http.ListenAndServe(":3000", nil)
}
