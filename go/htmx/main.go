package main

import (
	"html/template"
	"net/http"
)

type Todo struct {
	Title string
	Done  bool
}

func getTodos(w http.ResponseWriter, r *http.Request) {
	tmpl := template.Must(template.ParseFiles("templates/todos.html"))
	todos := []Todo{
		{Title: "demo1", Done: true},
		{Title: "demo2", Done: false},
	}
	tmpl.Execute(w, todos)
}

func root(w http.ResponseWriter, r *http.Request) {
	tmpl := template.Must(template.ParseFiles("templates/index.html"))
	tmpl.Execute(w, nil)
}

func main() {
	http.HandleFunc("/", root)
	http.HandleFunc("/todos", getTodos)
	http.ListenAndServe(":3000", nil)
}
