use super::{Cpu, CrementMode, RegisterAlias};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// INY - Increment the Y Register.
    /// Adds one to the Y register setting the zero and negative flags as appropriate.
    pub fn iny(&mut self) {
        self.crement_register(&RegisterAlias::Y, &CrementMode::Increment);
    }
}

#[cfg(test)]
mod tests {
    use super::{test_templates::immediate, CrementMode, RegisterAlias};

    const REGISTER_ALIAS: RegisterAlias = RegisterAlias::Y;
    const CREMENT_MODE: CrementMode = CrementMode::Increment;

    #[test]
    fn overflow() {
        immediate::wrapping(0xC8, &REGISTER_ALIAS, &CREMENT_MODE);
    }

    #[test]
    fn increment() {
        immediate::crement(0xC8, &REGISTER_ALIAS, &CREMENT_MODE);
    }
}
