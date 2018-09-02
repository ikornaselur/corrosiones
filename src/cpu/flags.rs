pub(crate) struct Flags {
    pub(crate) carry: bool,
    pub(crate) zero: bool,
    pub(crate) interrupt_disable: bool,
    pub(crate) decimal: bool,
    pub(crate) break_command: bool,
    pub(crate) overflow: bool,
    pub(crate) negative: bool,
}

impl Default for Flags {
    fn default() -> Flags {
        Flags {
            carry: false,
            zero: false,
            interrupt_disable: true,
            decimal: false,
            break_command: false,
            overflow: false,
            negative: false,
        }
    }
}

impl Flags {
    pub fn new() -> Flags {
        Flags::default()
    }

    /// Set all the flags from a single byte, useful for restoring the flags from memory
    pub fn set_from_byte(&mut self, byte: u8) {
        self.carry = byte & 0b0000_0001 != 0;
        self.zero = byte & 0b0000_0010 != 0;
        self.interrupt_disable = byte & 0b0000_0100 != 0;
        self.decimal = byte & 0b0000_1000 != 0;
        self.overflow = byte & 0b0100_0000 != 0;
        self.negative = byte & 0b1000_0000 != 0;
    }

    /// Get the flags a single byte, useful for storing the flags in memory
    pub fn as_byte(&self) -> u8 {
        let carry_byte = if self.carry { 1 } else { 0 };
        let zero_byte = if self.zero { 1 << 1 } else { 0 };
        let id_byte = if self.interrupt_disable { 1 << 2 } else { 0 };
        let decimal_byte = if self.decimal { 1 << 3 } else { 0 };
        let overflow_byte = if self.overflow { 1 << 6 } else { 0 };
        let negative_byte = if self.negative { 1 << 7 } else { 0 };

        0b0010_0000
            | carry_byte
            | zero_byte
            | id_byte
            | decimal_byte
            | overflow_byte
            | negative_byte
    }

    pub fn set_zero_from_byte(&mut self, byte: u8) {
        self.zero = byte == 0;
    }

    pub fn set_negative_from_byte(&mut self, byte: u8) {
        self.negative = byte >> 7 & 1 == 1;
    }

    pub fn set_carry(&mut self, carry: bool) {
        self.carry = carry;
    }

    pub fn set_zero(&mut self, zero: bool) {
        self.zero = zero;
    }

    pub fn set_overflow(&mut self, overflow: bool) {
        self.overflow = overflow;
    }

    pub fn set_negative(&mut self, negative: bool) {
        self.negative = negative;
    }

    pub fn set_interrupt_disable(&mut self, id: bool) {
        self.interrupt_disable = id;
    }

    pub fn set_decimal(&mut self, decimal: bool) {
        self.decimal = decimal;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn flags_into_u8() {
        let flags = Flags {
            carry: true,
            zero: true,
            interrupt_disable: true,
            decimal: false,
            break_command: false,
            overflow: false,
            negative: false,
        };

        assert_eq!(flags.as_byte(), 0b0010_0111);

        let flags = Flags {
            carry: false,
            zero: false,
            interrupt_disable: false,
            decimal: true,
            break_command: true,
            overflow: true,
            negative: true,
        };

        assert_eq!(flags.as_byte(), 0b1110_1000);
    }
}
