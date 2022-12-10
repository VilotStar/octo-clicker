#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::Octo;
pub mod fakerinput;
#[derive(Debug)]
pub enum mdata {
    Enabled(bool),
    HiValue(f64),
    LowValue(f64),
    Linear(bool)
}