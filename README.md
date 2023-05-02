# Eliminação De Gauss
Implementação da Eliminação de Gauss em __C__, __Rust__ e __Golang__ para o segundo trabalho da disciplina de Conceitos de Linguagens de Programação na UFPel.


## 🛠️ Compilando e executando:
Os programas recebem por linha de comando a matriz de entrada em um arquivo texto seguido pelo tamanho N de uma matriz NxN+1. A matriz deve:
* Estar no formato: 

![7](https://user-images.githubusercontent.com/76601652/235682119-3fcdb58b-7da4-4058-b486-225d6d9e020a.PNG)
* Ter o tamanho NxN+1, equivalente a matriz Ab desse exemplo:

<img src="https://user-images.githubusercontent.com/76601652/235670730-cc2ee712-386e-4d25-895d-354c366a8801.PNG" width="601" height="325">

### Exemplo em C:
```
gcc C.c -o c
.\c matrices/matrix_7.txt 7
```
Onde o arquivo texto "matrix_7.txt" contém uma matriz de entrada com tamanho 7x8


### Exemplo em Rust:
```
rustc Rust.rs
.\Rust matrices/matrix_35.txt 35
```
Onde o arquivo texto "matrix_35.txt" contém uma matriz de entrada com tamanho 35x36


### Exemplo em Golang:
```
go run Golang.go matrices/matrix_90.txt 90
```
Onde o arquivo texto "matrix_90.txt" contém uma matriz de entrada com tamanho 90x91

### Saída
A solução do sistema é escrita na tela, junto com o tempo de execução
![ree](https://user-images.githubusercontent.com/76601652/235684084-eca04f76-b4df-4175-8fb5-b1fdc9593c08.PNG)

## ✒️ Autores: 
* Bianca Waskow https://github.com/bihw <br>
* Rafael Siqueira https://github.com/Volrizz
