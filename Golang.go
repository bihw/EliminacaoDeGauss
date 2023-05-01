package main

import "fmt"

const MAXN int = 20 // Max value of matrix order
var N int
var A [MAXN][MAXN]float32
var X [MAXN]float32

func main() {
	var i, j int

	fmt.Printf("\nEnter the order of matrix: ")
	fmt.Scan(&N)

	fmt.Printf("\nEnter the elements of augmented matrix row-wise:\n\n")
	for i = 0; i < N; i++ {
		for j = 0; j <= N; j++ {
			fmt.Printf("A[%d][%d] : ", i+1, j+1)
			fmt.Scan(&A[i][j])
		}
	}

	gauss()

	fmt.Printf("\nThe solution is: \n")
	for i = 0; i < N; i++ {
		fmt.Printf("\nx%d=%f\t", i+1, X[i]) // x1, x2, x3 are the required solutions
	}
}

func gauss() {
	var c, sum float32

	for j := 0; j < N; j++ { // loop for the generation of upper triangular matrix
		for i := 0; i < N; i++ {
			if i > j {
				c = A[i][j] / A[j][j]
				for k := 0; k <= N; k++ {
					A[i][k] = A[i][k] - c*A[j][k]
				}
			}
		}
	}
	X[N-1] = A[N-1][N] / A[N-1][N-1]

	// this loop is for backward substitution
	for i := N - 2; i >= 0; i-- {
		sum = 0
		for j := i + 1; j < N; j++ {
			sum = sum + A[i][j]*X[j]
		}
		X[i] = (A[i][N] - sum) / A[i][i]
	}
}
