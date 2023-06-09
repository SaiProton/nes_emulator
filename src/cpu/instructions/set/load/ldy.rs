use super::{AddressingMode, Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// LDY - Load Y Register.
    /// Loads a byte of memory into the Y register setting the zero and negative flags as
    /// appropriate.
    pub fn ldy(&mut self, addr_mode: &AddressingMode) {
        self.load(addr_mode, &RegisterAlias::Y);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_templates::{absolute, immediate, zero},
        RegisterAlias,
    };

    const REGISTER_ALIAS: RegisterAlias = RegisterAlias::Y;

    #[test]
    fn imm_load_data() {
        immediate::load_data(0xA0, &REGISTER_ALIAS);
    }

    #[test]
    fn imm_zero_flag() {
        immediate::flag_check(0xA0);
    }

    #[test]
    fn zero_from_memory() {
        zero::from_memory(0xA4, &REGISTER_ALIAS);
    }

    #[test]
    fn zero_x_from_memory() {
        zero::x_from_memory(0xB4, &REGISTER_ALIAS);
    }

    #[test]
    fn abs_from_memory() {
        absolute::from_memory(0xAC, &REGISTER_ALIAS);
    }

    #[test]
    fn abs_x_from_memory() {
        absolute::x_from_memory(0xBC, &REGISTER_ALIAS);
    }
}
