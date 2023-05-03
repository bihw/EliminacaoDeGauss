# Eliminação De Gauss
Implementação da Eliminação de Gauss em __C__, __Rust__ e __Golang__ para o segundo trabalho da disciplina de Conceitos de Linguagens de Programação na UFPel.


## 🛠️ Compilando e executando:
Os programas recebem por linha de comando a matriz de entrada em um arquivo texto seguido pelo tamanho N de uma matriz NxN+1 (matriz aumentada). A matriz deve:
* Estar no formato: 

![7](https://user-images.githubusercontent.com/76601652/235682119-3fcdb58b-7da4-4058-b486-225d6d9e020a.PNG)
* Ser a matriz aumentada do sistema (tamanho NxN+1), equivalente a matriz Ab desse exemplo:

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

### ✔️ Saída
A solução do sistema é escrita na tela, junto com o tempo de execução

![ree](https://user-images.githubusercontent.com/76601652/235684084-eca04f76-b4df-4175-8fb5-b1fdc9593c08.PNG) 

* Não esqueça de alterar o valor de MAXN no início de cada código conforme o necessário (e suportado) para o tamanho da sua matriz.

#### Observação
Por motivos de segurança, em Rust, o tamanho máximo de uma matriz que pode ser alocada na pilha da thread principal depende do tamanho dessa pilha, que varia de acordo com a arquitetura do sistema (32 ou 64 bits) e com a versão do sistema operacional que o programa está sendo executado. Por exemplo, no Windows 10 de 64 bits, o tamanho padrão é de 1 MB, enquanto no Linux de 64 bits é de 8 MB. Ou seja, para que esse programa fuincione sem estourar a pilha da thread main temos um tamanho máximo para a matriz de entrada dependendo do seu SO: 
* No win10-64bit: o tamanho máximo é 353x354 (altere o valor de MAXN para 353);
* e no linux-64bit: é 1015x1016 (altere o valor de MAXN para 1015) (não foram feitos testes em Linux).
Uma solução para isso seria mover a matriz para o heap, por exemplo, usando a coleção _Vec_. Por exemplo:
* Você pode criar uma matriz usando ```Vec``` criando um ```Vec``` de ```Vecs```. Aqui está um exemplo de como você pode criar uma matriz de ```f64```s 405x406:
```
let mut ab: Vec<Vec<f64>> = vec![vec![0.0; 406]; 405];
```
Isso criará uma matriz de 405 linhas e 406 colunas, onde cada elemento é inicializado com o valor 0.0.
Nós não fizemos isso para deixar da maneira mais parecida com a nossa versão do programa em C.

## ✒️ Autores: 
* Bianca Waskow https://github.com/bihw <br>
* Rafael Siqueira https://github.com/Volrizz
