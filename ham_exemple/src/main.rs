pub const N_SITE: u32 = 8;
pub const CONS_T: f64 = 1.0;
pub const CONS_U: f64 = 1.0;

mod matrix;

use std::time::Instant;
use lapack::dspevd;

fn main() {
    // Générer la banque d'états
    // 0 .. 2^N
    let now = Instant::now();
    let mut matrix = matrix::gen_matrix_blocs();


    println!("Time taken: {:.2?}", now.elapsed());
    println!("Taille du système: {}", N_SITE);
    println!("Nombres de blocs: {}", matrix.len());
    // Taille du bloc maximum
    let mut max = 0;
    for n in matrix.iter() {
        if n.len() > max { max = n.len();}
    }
    println!("Taille du bloc maximal: {}", max);
    println!("Let's go eigenvalues!");
    matrix.pop();
    let now = Instant::now();
    for mut bloc in matrix.into_iter() {
        let order = (<f64>::sqrt((8 * bloc.len() + 1) as f64) - 1.0) / 2.0;
        let mut eigenvalues: Vec<f64> = vec![0.0; order as usize];
        let mut eigenvectors: Vec<f64> = Vec::with_capacity(1);
        let mut work: Vec<f64> = Vec::with_capacity(2 * order as usize);
        let mut iwork: Vec<i32> = Vec::with_capacity(2 * order as usize);
        let mut info: i32 = 0;
        unsafe {
            dspevd(
                b"N"[0],
                b"U"[0],
                order as i32,
                &mut bloc,
                &mut eigenvalues,
                &mut eigenvectors,
                order as i32,
                &mut work,
                2 * order as i32,
                &mut iwork,
                1,
                &mut info
            );
        }
    }
    println!("Time taken eigen: {:.2?}", now.elapsed());
}

