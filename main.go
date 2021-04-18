package main

import (
    "fmt"
    // "time"
)

func main() {
    limit := 200000
    arr1 := make([]float64, limit)
    arr2 := make([]float64, limit)
    for i := 0;i<limit;i++ {
        arr1[i] = float64(i + 1)
        arr2[i] = float64(i + 1)
    }
    // t := time.Now()
    var dotP float64 = 0.0
    for i := 0;i<limit;i++ {
        dotP += float64(arr1[i])*float64(arr2[i])
    }
    // fmt.Println(time.Since(t))
    fmt.Printf("%f\n", dotP)
}
