const N_SITE: u32 = 2;
const CONS_T: u32 = 1;
const CONS_U: u32 = 1;

#[inline(always)]
fn terme_pot(n: u32) -> u32 {
    ((n << N_SITE) & n).count_ones() * CONS_U
}

fn c_dag_c(n: u32, mask: u32) -> u32 {
    if ((mask) & n) == mask {
        // Can destroy
        let new_n = n ^ mask;
        let mask = mask << 1;
        if (mask & new_n) == 0 {
            return new_n ^ mask
        }
    }
    return 0
}

fn c_c_dag(n: u32, mask: u32) -> u32 {
    if ((mask) & n) == mask {
        // Can destroy
        let new_n = n ^ mask;
        let mask = mask >> 1;
        if (mask & new_n) == 0 {
            return new_n ^ mask
        }
    }
    return 0
}

fn terme_cin(n: u32) -> Vec<u32>{
    let mut out: Vec<u32> = Vec::new();
    for i in 0..(N_SITE-1) {
        let mask1: u32 = 1 << i;
        let mask2: u32 = 1 << i+N_SITE;
        let mixed1 = c_dag_c(n, mask1);
        let mixed2 = c_dag_c(n, mask2);
        if mixed1 !=0 {out.push(mixed1);}
        if mixed2 !=0 {out.push(mixed2);}
    }
    for i in (0..(N_SITE-1)).rev() {
        let mask1: u32 = 1 << i+1;
        let mask2: u32 = 1 << i+N_SITE+1;
        let mixed1 = c_c_dag(n, mask1);
        let mixed2 = c_c_dag(n, mask2);
        if mixed1 !=0 {out.push(mixed1);}
        if mixed2 !=0 {out.push(mixed2);}
    }
    if out.len() == 0 {out.push(n);}
    out
}

fn main() {
    // Générer la banque d'états
    // 0 .. 2^N
    let mut skip: Vec<u32> = Vec::new();
    let mut sub_space: Vec<Vec<u32>> = Vec::new();
    sub_space.push(Vec::new());
    let mut subspace_len: usize = 0;

    // Iter on banque
    for n_curr in 0 as u32..(2<<N_SITE){
        println!("Iter: {}", n_curr);
        if skip.contains(&n_curr) {
            println!("Skipped");
            continue;
        }
        // Compte le nombre de U à mettre sur cette diagonale
        sub_space[subspace_len].push(terme_pot(n_curr));

        // Termes hors diag
        let mut bank: Vec<u32> = Vec::new();
        let mut map: Vec<(u32, Vec<u32>)> = Vec::new();
        bank.push(n_curr);
        if n_curr != 0{
            let mut to_skip = terme_cin(n_curr);
            let mut b: bool = true;
            for elem in to_skip.iter() {
                b = b & bank.contains(&elem);
            }
            map.push((n_curr, to_skip.clone()));
            while ! b {
                let mut tmp = terme_cin(n);
                map.push(())
            }
        }

        // Change de bloc
        sub_space.push(Vec::new());
        subspace_len += 1;
        println!("{:?}", sub_space);
    }

}
