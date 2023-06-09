use super::{AddressingMode, Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// CPX - Compare X Register.
    /// This instruction compares the contents of the X register with another memory held value and
    /// sets the zero and carry flags as appropriate.
    pub fn cpx(&mut self, addr_mode: &AddressingMode) {
        self.compare(&RegisterAlias::X, addr_mode);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_templates::{absolute, immediate, zero},
        RegisterAlias,
    };

    const REGISTER_ALIAS: RegisterAlias = RegisterAlias::X;

    #[test]
    fn imm_compare() {
        immediate::compare(0xE0, &REGISTER_ALIAS);
    }

    #[test]
    fn zero_compare() {
        zero::compare(0xE4, &REGISTER_ALIAS);
    }

    #[test]
    fn abs_compare() {
        absolute::compare(0xEC, &REGISTER_ALIAS);
    }
}
