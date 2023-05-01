use std::io;

const MAXN: usize = 20; // Max value of matrix order
// 

fn main() {
    let n: usize;
    let mut a: [[f32; MAXN+1]; MAXN] = [[0.0; MAXN+1]; MAXN];
    let mut x: [f32; MAXN] = [0.0; MAXN];

    let mut input = String::new();
    println!("Enter the order of matrix: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    n = input.trim().parse().unwrap();

    println!("\nEnter the elements of augmented matrix row-wise:\n");
    for i in 0..n {
        for j in 0..=n {
            println!("A[{}][{}] : ", i, j);
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            a[i][j] = input.trim().parse().unwrap();
        }
    }

    gauss(n, &mut a, &mut x);

    println!("\nThe solution is:");
    for i in 0..n {
        println!("x{} = {}", i+1, x[i]); // x1, x2, x3 are the required solutions
    }
}

fn gauss(n: usize, a: &mut[[f32; MAXN+1]; MAXN], x: &mut[f32]) {
    for j in 0..n {
        for i in 0..n {
            if i > j {
                let c = a[i][j] / a[j][j];
                for k in 0..=n {
                    a[i][k] -= c * a[j][k];
                }
            }
        }
    }
    x[n-1] = a[n-1][n] / a[n-1][n-1];
    for i in (0..n-1).rev() {
        let mut sum = 0.0;
        for j in i+1..n {
            sum += a[i][j] * x[j];
        }
        x[i] = (a[i][n] - sum) / a[i][i];
    }
}
