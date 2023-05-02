use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::env;
use std::fs::File;
const MAXN: usize = 200; // Max value of A matrix order
fn main() {
    let t_start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let n: usize = args[2].parse().unwrap();
    let mut ab: [[f64; MAXN+1]; MAXN] = [[0.0; MAXN+1]; MAXN];
    let mut x: [f64; MAXN] = [0.0; MAXN];
    if args.len() != 3 {
        println!("Incorrect number of arguments");
        std::process::exit(1);
    }
    read_matrix(n, &args[1], &mut ab);
    /*println!("Your Ab matrix is: ");
    for i in 0..n {
        for j in 0..(n + 1) {
            print!("{:.2}  ", ab[i][j]);
        }
        println!("");
    }*/
    gauss(n, &mut ab, &mut x);
    println!("\nThe solution is:");
    for i in 0..n { 
        println!("x{} = {:.2}", i+1, x[i]); /* x1, x2, x3 are the required solutions*/
    }
    let t_end = Instant::now();
    let execution_time = t_end.duration_since(t_start);
    println!("Tempo de execucao: {:#?}", execution_time);
}
fn gauss(n: usize, ab: &mut[[f64; MAXN+1]; MAXN], x: &mut [f64; MAXN]) {
    for j in 0..n { /* loop for the generation of upper triangular matrix*/
        for i in 0..n {
            if i > j {
                let c = ab[i][j] / ab[j][j];
                for k in 0..(n + 1) {
                    ab[i][k] = ab[i][k] - c * ab[j][k];
                }
            }
        }
    }
    x[n-1] = ab[n-1][n] / ab[n-1][n-1];
    for i in (0..n-1).rev() { // this loop is for backward substitution
        let mut sum = 0.0;
        for j in i+1..n {
            sum = sum + ab[i][j] * x[j];
        }
        x[i] = (ab[i][n] - sum) / ab[i][i];
    }
}
fn read_matrix(n: usize, arg: &str, ab: &mut[[f64; MAXN+1]; MAXN]){
    let file = File::open(arg).unwrap();
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() { // read matrix elements
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|num| num.parse().unwrap());
        for j in 0..n+1 {
            ab[i][j] = nums.next().unwrap();
        }
    }
}