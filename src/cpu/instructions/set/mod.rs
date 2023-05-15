use super::{AddressingMode, Cpu};

#[cfg(test)]
use super::RegisterAlias;

mod inx;
mod load;
mod store;
mod tax;

#[cfg(test)]
mod tests;
