// uma adaptação desses dois códigos: 
// https://www.codewithc.com/c-program-for-gauss-elimination-method/
// e https://github.com/gmendonca/gaussian-elimination-pthreads-openmp/blob/master/gauss.c 

#include <stdio.h>

#define MAXN 20  /* Max value of matrix order */
int N; 
float A[MAXN][MAXN], X[MAXN];

void gauss();

int main()
{
    int i, j;    
    
    printf("\nEnter the order of matrix: ");
    scanf("%d",&N);
    
    printf("\nEnter the elements of augmented matrix row-wise:\n\n");
    for(i=1; i<=N; i++)
    {
        for(j=1; j<=(N+1); j++)
        {
            printf("A[%d][%d] : ", i,j);
            scanf("%f",&A[i][j]);
        }
    }

    gauss();

    printf("\nThe solution is: \n");
    for(i=1; i<=N; i++)
    {
        printf("\nx%d=%f\t",i,X[i]); /* x1, x2, x3 are the required solutions*/
    }

    return(0);
}

void gauss(){
    int i, j, k;
    float c, sum=0.0;

    for(j=1; j<=N; j++) /* loop for the generation of upper triangular matrix*/
    {
        for(i=1; i<=N; i++)
        {
            if(i>j)
            {
                c=A[i][j]/A[j][j];
                for(k=1; k<=N+1; k++)
                {
                    A[i][k]=A[i][k]-c*A[j][k];
                }
            }
        }
    }
    X[N]=A[N][N+1]/A[N][N];
    
    for(i=N-1; i>=1; i--) /* this loop is for backward substitution*/
    {
        sum=0;
        for(j=i+1; j<=N; j++)
        {
            sum=sum+A[i][j]*X[j];
        }
        X[i]=(A[i][N+1]-sum)/A[i][i];
    }
}