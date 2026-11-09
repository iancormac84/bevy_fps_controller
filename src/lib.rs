#[cfg(any(feature = "avian-f32", feature = "avian-f64"))]
mod controller_avian;
#[cfg(feature = "rapier")]
mod controller_rapier;

pub mod controller {
    #[cfg(any(feature = "avian-f32", feature = "avian-f64"))]
    pub use crate::controller_avian::*;
    #[cfg(feature = "rapier")]
    pub use crate::controller_rapier::*;
}
