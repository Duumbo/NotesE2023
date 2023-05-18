pub const N_SITE: u32 = 5;
pub const CONS_T: u32 = 1;
pub const CONS_U: u32 = 1;

mod operators;

use std::time::Instant;

fn main() {
    // Générer la banque d'états
    // 0 .. 2^N
    let now = Instant::now();
    let mut skip: Vec<u32> = Vec::new();
    let mut sub_space: Vec<Vec<u32>> = Vec::new();
    sub_space.push(Vec::new());
    let mut subspace_len: usize = 0;

    // Iter on states
    for n_curr in 0 as u32..(1<<2*N_SITE){
        if skip.contains(&n_curr) {
            continue;
        }
        // Calcul du terme diagonal (premier élément de la matrice.)
        sub_space[subspace_len].push(
            operators::terme_pot(n_curr)
        );

        // Check si le sub space est de plus grande dimension que 1
        let mut to_skip = vec![n_curr];
        to_skip.append(&mut operators::terme_cin(n_curr));
        let mut guessed_dim = 1;
        for elem in to_skip.iter() {
            if *elem != n_curr {
                // Compte le nombre d'éléments différents.
                guessed_dim += 1;
            }
        }

        // Si la dimension du sous-espace est plus grande qu'un, il faut trouver
        // tout le sous-espace
        if guessed_dim > 1 {
            let mut to_skip_len = to_skip.len();
            let mut i = 1;
            while i < to_skip_len {
                let state = to_skip[i];

                // Get the diag
                sub_space[subspace_len].push(
                    operators::terme_pot(state)
                );

                // Get other elements
                let t = operators::terme_cin(state);
                for m in (0..i).rev() {
                    if t.contains(&to_skip[m]) {
                        // Push cons_t
                        sub_space[subspace_len].push(CONS_T);
                    }
                    else {
                        sub_space[subspace_len].push(0);
                    }
                }
                // Check for new values
                for m in t.into_iter() {
                    if ! to_skip.contains(&m) {
                        to_skip.push(m);
                        to_skip_len += 1;
                    }
                }
                i += 1

            }
        }

        // Change de bloc
        sub_space.push(Vec::new());
        subspace_len += 1;
        // Update skip list
        // Let's clean up values that we don't need
        let cond = |skip_passed: &u32| skip_passed > &n_curr;
        skip.retain(cond);
        to_skip.retain(cond);
        // Skip next time
        skip.append(&mut to_skip);
    }

    println!("Time taken: {:.2?}", now.elapsed());
    println!("Taille du système: {}", N_SITE);
    println!("Nombres de blocs: {}", sub_space.len());
    // Taille du bloc maximum
    let mut max = 0;
    for n in sub_space.iter() {
        if n.len() > max { max = n.len();}
    }
    println!("Taille du bloc maximal: {}", max);
}

