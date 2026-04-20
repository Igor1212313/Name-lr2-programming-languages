package main

import (
 "bufio"
 "fmt"
 "os"
 "strconv"
 "strings"
)

func isPrime(n int) bool {
 if n < 2 {
  return false
 }
 if n == 2 {
  return true
 }
 if n%2 == 0 {
  return false
 }
 for i := 3; i*i <= n; i += 2 {
  if n%i == 0 {
   return false
  }
 }
 return true
}

func nearestPrimeDistance(n int) int {
 if isPrime(n) {
  return 0
 }
 d := 1
 for {
  if isPrime(n-d) || isPrime(n+d) {
   return d
  }
  d++
 }
}

func main() {
 var N int
 data, err := os.ReadFile("input.txt")
 if err == nil {
  text := strings.TrimSpace(string(data))
  N, _ = strconv.Atoi(text)
 } else {
  fmt.Print("Введите N: ")
  in := bufio.NewReader(os.Stdin)
  fmt.Fscan(in, &N)
 }

 result := nearestPrimeDistance(N)

 fmt.Println("Результат:", result)

 file, _ := os.Create("output.txt")
 defer file.Close()
 fmt.Fprintln(file, result)
}
