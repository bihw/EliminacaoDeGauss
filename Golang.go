package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

const MAXN int = 20 // Max value of matrix order
var N int
var Ab [MAXN][MAXN + 1]float64
var X [MAXN]float64

func main() {
	t_start := time.Now()
	var i, j int
	if len(os.Args) != 3 {
		fmt.Println("Incorrect number of arguments")
		os.Exit(1)
	}
	N, _ = strconv.Atoi(os.Args[2])
	readMatrix(N, os.Args[1])
	for i = 0; i < N; i++ {
		for j = 0; j <= N; j++ {
			fmt.Printf("Ab[%d][%d]: %.2f\n", i+1, j+1, Ab[i][j])
		}
	}
	gauss()
	fmt.Printf("\nThe solution is: ")
	for i = 0; i < N; i++ {
		fmt.Printf("\nx%d=%f\t", i+1, X[i]) // x1, x2, x3 are the required solutions
	}
	t_end := time.Now()
	execution_time := t_end.Sub(t_start)
	fmt.Printf("\nTempo de execução: %v\n", execution_time)
}
func gauss() {
	var c, sum float64
	for j := 0; j < N; j++ { // loop for the generation of upper triangular matrix
		for i := 0; i < N; i++ {
			if i > j {
				c = Ab[i][j] / Ab[j][j]
				for k := 0; k <= N; k++ {
					Ab[i][k] = Ab[i][k] - c*Ab[j][k]
				}
			}
		}
	}
	X[N-1] = Ab[N-1][N] / Ab[N-1][N-1]
	for i := N - 2; i >= 0; i-- { // this loop is for backward substitution
		sum = 0
		for j := i + 1; j < N; j++ {
			sum = sum + Ab[i][j]*X[j]
		}
		X[i] = (Ab[i][N] - sum) / Ab[i][i]
	}
}
func readMatrix(N int, filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	for i := 0; i < N; i++ {
		line := ""
		if scanner.Scan() {
			line = scanner.Text()
		} else {
			fmt.Println("Error reading file")
			os.Exit(1)
		}
		tokens := strings.Fields(line)
		if len(tokens) != N+1 {
			fmt.Println("Invalid line in file:", line)
			os.Exit(1)
		}
		for j := 0; j <= N; j++ {
			value, err := strconv.ParseFloat(tokens[j], 64)
			if err != nil {
				fmt.Println("Invalid value in file:", tokens[j])
				os.Exit(1)
			}
			Ab[i][j] = float64(value)
		}
	}
}
