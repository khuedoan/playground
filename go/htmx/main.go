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
	TodoOpen TodoStatus = 0
	TodoDone TodoStatus = 1
)

type Todo struct {
	Id          int
	Description string
	Status      TodoStatus
}

func index(w http.ResponseWriter, r *http.Request) {
	tmpl := template.Must(template.ParseFiles("templates/index.html"))
	tmpl.Execute(w, nil)
}

func todo(w http.ResponseWriter, r *http.Request) {
	db, err := sql.Open("sqlite3", "data/sqlite.db")
	if err != nil {
		log.Fatal(err)
	}

	switch r.Method {
	case "GET":
		rows, err := db.Query("SELECT * FROM todos")
		if err != nil {
			log.Fatal(err)
		}
		defer rows.Close()

		todos := []Todo{}
		for rows.Next() {
			todo := Todo{}
			rows.Scan(&todo.Id, &todo.Description, &todo.Status)
			todos = append(todos, todo)
		}

		tmpl := template.Must(template.ParseFiles("templates/todos.html"))
		tmpl.Execute(w, todos)
	case "POST":
		if err := r.ParseForm(); err != nil {
			log.Fatal(err)
			return
		}

		db.Exec(
			"INSERT INTO todos (description, status) VALUES(?, ?)",
			r.FormValue("description"),
			TodoOpen,
		)
		w.Header().Set("HX-Trigger", "updateTodos")
	case "PATCH":
		if err := r.ParseForm(); err != nil {
			log.Fatal(err)
			return
		}

		db.Exec(
			"UPDATE todos SET status=? WHERE id=?",
			r.FormValue("status") == "on",
			r.FormValue("id"),
		)
	case "DELETE":
		if err := r.ParseForm(); err != nil {
			log.Fatal(err)
			return
		}

		db.Exec(
			"DELETE FROM todos WHERE id=?",
			r.FormValue("id"),
		)
		w.Header().Set("HX-Trigger", "updateTodos")
	}
}

func main() {
	http.HandleFunc("/", index)
	http.HandleFunc("/todo", todo)
	http.ListenAndServe(":3000", nil)
}
