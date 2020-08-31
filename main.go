package main

import (
    "fmt"
    "strconv"
)

var (
    name string = "Khue Doan"
    age int = 22
)

func main() {
    var strAge string = strconv.Itoa(age)
    fmt.Printf("Name: %v, Age: %v", name, strAge)
}
