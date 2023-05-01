use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::env;
use std::fs::File;
const MAXN: usize = 20; // Max value of matrix order
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
    for i in 0..n {
        for j in 0..(n + 1) {
            println!("Ab[{}][{}]: {:.2}", i + 1, j + 1, ab[i][j]);
        }
    }
    gauss(n, &mut ab, &mut x);
    println!("\nThe solution is:");
    for i in 0..n {
        println!("x{} = {:.2}", i+1, x[i]);
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
            sum += ab[i][j] * x[j];
        }
        x[i] = (ab[i][n] - sum) / ab[i][i];
    }
}
fn read_matrix(n: usize, arg: &str, ab: &mut[[f64; MAXN+1]; MAXN]){
    let filename = arg;
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|num| num.parse().unwrap());
        for j in 0..n+1 {
            ab[i][j] = nums.next().unwrap();
        }
    }
}