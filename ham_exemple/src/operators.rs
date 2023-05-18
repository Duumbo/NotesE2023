// Operator.rs
// Defines number operators and hoping operators.

use crate::{N_SITE, CONS_U};

#[inline(always)]
pub fn terme_pot(n: u32) -> u32 {
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

pub fn terme_cin(n: u32) -> Vec<u32>{
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

#[test]
fn test_terme_potentiel() {
    assert_eq!(0, terme_pot(1));
    assert_eq!(0, terme_pot(3));
    assert_eq!(1 * CONS_U, terme_pot(5));
    assert_eq!(1 * CONS_U, terme_pot(11));
    assert_eq!(2 * CONS_U, terme_pot(15));
}

#[test]
fn test_terme_hopping() {
    assert_eq!(vec![1], terme_cin(2));
    assert_eq!(vec![2], terme_cin(1));
    assert_eq!(vec![3], terme_cin(3));
    assert_eq!(vec![6, 9], terme_cin(5));
    assert_eq!(vec![10, 5], terme_cin(6));
    assert_eq!(vec![10, 5], terme_cin(9));
}
