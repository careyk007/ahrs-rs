//! A collection of AHRS algorithms ported to Rust.

#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "ahrs"]

extern crate nalgebra as na;
extern crate alga;

pub use ahrs::{
    Ahrs
};
pub use madgwick::{
    Madgwick
};
pub use mahony::{
    Mahony
};

mod ahrs;
mod madgwick;
mod mahony;
