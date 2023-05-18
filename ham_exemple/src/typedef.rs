#![allow(unused_imports)]
use num::complex::Complex;
use derive_more::{Add, From, Mul, Into, Shl, BitAnd, BitXor, Shr};

// Élément de base d'un état (u32 ou u128) supported
#[cfg(feature = "uint32")]
type TypeDeSite = u32;
#[cfg(feature = "uint128")]
type TypeDeSite = u128;

// Précision des éléments de matrice (f32, f64, Complex<f64>, Complex<f32>)
#[cfg(feature = "coefs_real")]
#[cfg(feature = "double_precision")]
pub type TypePrecision = f64;
#[cfg(feature = "coefs_real")]
#[cfg(feature = "single_precision")]
type TypePrécision = f32;
#[cfg(feature = "double_precision")]
#[cfg(feature = "coefs_complex")]
type TypePrécision = Complex<f64>;
#[cfg(feature = "coefs_complex")]
#[cfg(feature = "single_precision")]
type TypePrécision = Complex<f32>;

#[derive(Debug, Copy, Clone, PartialEq, From, Add, Mul, Into)]
#[mul(forward)]
//#[from(types(f64))]
pub struct Precision (pub TypePrecision);

//impl From<u32> for Precision {
//    fn from(num: u32) -> Precision {
//        Precision::from(num as f64)
//    }
//}
//

// Structure de données du cluster
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    From,
    Add,
    Mul,
    Into,
    Shl,
    BitAnd,
    BitXor,
    Shr,
    PartialOrd
)]
#[mul(forward)]
pub struct Cluster (pub TypeDeSite);

impl Cluster {
    pub fn count_ones(self) -> u32 {
        <TypeDeSite>::count_ones(self.into())
    }
}

