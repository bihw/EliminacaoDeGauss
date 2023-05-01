#include <stdio.h>
#include <time.h>
#include <stdlib.h>
#define MAXN 20  /* Max value of matrix order */
int N; 
float Ab[MAXN][MAXN], X[MAXN];
void gauss();
void read_matrix(int N, char* argv);
int main(int argc, char** argv) { // >.\c.exe matrix.txt N
    clock_t t_start, t_end;
    double execution_time;
    t_start = clock();
    int i, j; 
    if (argc != 3) {
        printf("Incorrect number of arguments\n");
        exit(EXIT_FAILURE);
    }
    N = atoi(argv[2]);
    read_matrix(N, argv[1]);
    for(i=1; i<=N; i++) {
        for(j=1; j<=(N+1); j++) {
            printf("Ab[%d][%d]: %.2f\n", i,j,Ab[i][j]);
        }
    }
    gauss();
    printf("\nThe solution is: ");
    for(i=1; i<=N; i++) {
        printf("\nx%d = %f\t",i,X[i]); /* x1, x2, x3 are the required solutions*/
    }
    t_end = clock();
    execution_time = (((double)(t_end - t_start)) / CLOCKS_PER_SEC)*1000.0;
    printf("\nTempo de execucao: %.4fms\n", execution_time);
    return(0);
}
void gauss(){
    int i, j, k;
    float c, sum=0.0;
    for(j=1; j<=N; j++) { /* loop for the generation of upper triangular matrix*/
        for(i=1; i<=N; i++) {
            if(i>j) {
                c=Ab[i][j]/Ab[j][j];
                for(k=1; k<=N+1; k++)
                {
                    Ab[i][k]=Ab[i][k]-c*Ab[j][k];
                }
            }
        }
    }
    X[N]=Ab[N][N+1]/Ab[N][N];    
    for(i=N-1; i>=1; i--) { /* this loop is for backward substitution*/
        sum=0;
        for(j=i+1; j<=N; j++) {
            sum=sum+Ab[i][j]*X[j];
        }
        X[i]=(Ab[i][N+1]-sum)/Ab[i][i];
    }
}
void read_matrix(int N, char* argv){
    FILE *fp;
    int i, j;
    fp = fopen(argv, "rt");
    if (fp == NULL) {
        printf("Cannot open file %s\n", argv);
        exit(EXIT_FAILURE);
    }
    for (i=1; i<=N; i++) { // read matrix elements
        for (j=1; j<=(N+1); j++) {
            fscanf(fp, "%f", &Ab[i][j]); 
        }
    }
    fclose(fp);
}