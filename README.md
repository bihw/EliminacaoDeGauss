# Eliminação De Gauss
Implementação da Eliminação de Gauss em __C__, __Rust__ e __Golang__ para o segundo trabalho da disciplina de Conceitos de Linguagens de Programação na UFPel.


## 🛠️ Compilando e executando:
Os programas recebem por linha de comando a matriz de entrada em um arquivo texto seguido pelo tamanho N de uma matriz NxN+1. A matriz deve:
* Estar no formato: 

![6](https://user-images.githubusercontent.com/76601652/235670231-b0e4994c-876b-4610-a684-a55931489198.PNG)
* Ter o tamanho NxN+1, equivalente a matriz Ab desse exemplo:

<img src="https://user-images.githubusercontent.com/76601652/235670730-cc2ee712-386e-4d25-895d-354c366a8801.PNG" width="651" height="375">

### Exemplo em C:
```
gcc C.c -o c.exe
.\c.exe matrix_6.txt 6
```
Onde o arquivo texto "matrix_6.txt" contém uma matriz de entrada com tamanho 6x7


### Exemplo em Rust:
```
rustc Rust.rs
.\Rust.exe matrix_7.txt 7
```
Onde o arquivo texto "matrix_7.txt" contém uma matriz de entrada com tamanho 7x8


### Exemplo em Golang:
```
go run Golang.go matrix_8.txt 8
```
Onde o arquivo texto "matrix_8.txt" contém uma matriz de entrada com tamanho 8x9

## ✒️ Autores: 
* Bianca Waskow https://github.com/bihw <br>
* Rafael Siqueira https://github.com/Volrizz
