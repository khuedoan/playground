package main

import (
    "fmt"
)

var (
    name string = "Khue Doan"
    age int = 22
)

func main() {
    var floatAge float32 = float32(age)
    fmt.Printf("Name: %v, Age: %v", name, floatAge)
}
