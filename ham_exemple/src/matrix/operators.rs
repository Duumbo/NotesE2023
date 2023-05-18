// Operator.rs
// Defines number operators and hoping operators.

use crate::typedef::{Precision, Cluster};
use crate::{N_SITE, CONS_U};

#[inline(always)]
pub fn terme_pot(n: Cluster) -> Precision {
    Precision::from(((n << N_SITE) & n).count_ones() as f64) * CONS_U
}

fn c_dag_c(n: Cluster, mask: Cluster) -> Cluster {
    if ((mask) & n) == mask {
        // Can destroy
        let new_n = n ^ mask;
        let mask = mask << 1;
        if (mask & new_n) == Cluster(0) {
            return new_n ^ mask
        }
    }
    return Cluster(0)
}

fn c_c_dag(n: Cluster, mask: Cluster) -> Cluster {
    if ((mask) & n) == mask {
        // Can destroy
        let new_n = n ^ mask;
        let mask = mask >> 1;
        if (mask & new_n) == Cluster(0) {
            return new_n ^ mask
        }
    }
    return Cluster(0)
}

pub fn terme_cin(n: Cluster) -> Vec<Cluster>{
    let mut out: Vec<Cluster> = Vec::new();
    for i in 0..(N_SITE-1) {
        let mask1: Cluster = Cluster(1 << i);
        let mask2: Cluster = Cluster(1 << i+N_SITE);
        let mixed1 = c_dag_c(n, mask1);
        let mixed2 = c_dag_c(n, mask2);
        if mixed1 !=Cluster(0) {out.push(mixed1);}
        if mixed2 !=Cluster(0) {out.push(mixed2);}
    }
    for i in (0..(N_SITE-1)).rev() {
        let mask1: Cluster = Cluster(1 << i+1);
        let mask2: Cluster = Cluster(1 << i+N_SITE+1);
        let mixed1 = c_c_dag(n, mask1);
        let mixed2 = c_c_dag(n, mask2);
        if mixed1 !=Cluster(0) {out.push(mixed1);}
        if mixed2 !=Cluster(0) {out.push(mixed2);}
    }
    if out.len() == 0 {out.push(n);}
    out
}

#[test]
fn test_terme_potentiel() {
    assert_eq!(Precision::from(0.0) , terme_pot(Cluster(1)));
    assert_eq!(Precision::from(0.0), terme_pot(Cluster(3)));
    assert_eq!(Precision::from(1.0) * CONS_U, terme_pot(Cluster(5)));
    assert_eq!(Precision::from(1.0) * CONS_U, terme_pot(Cluster(11)));
    assert_eq!(Precision::from(2.0) * CONS_U, terme_pot(Cluster(15)));
}

#[test]
fn test_terme_hopping() {
    assert_eq!(vec![Cluster(1)], terme_cin(Cluster(2)));
    assert_eq!(vec![Cluster(2)], terme_cin(Cluster(1)));
    assert_eq!(vec![Cluster(3)], terme_cin(Cluster(3)));
    assert_eq!(vec![Cluster(6), Cluster(9)], terme_cin(Cluster(5)));
    assert_eq!(vec![Cluster(10), Cluster(5)], terme_cin(Cluster(6)));
    assert_eq!(vec![Cluster(10), Cluster(5)], terme_cin(Cluster(9)));
}
