pub(crate) struct Flags {
    pub(crate) carry: bool,
    pub(crate) zero: bool,
    pub(crate) interrupt_disable: bool,
    pub(crate) break_command: bool,
    pub(crate) overflow: bool,
    pub(crate) negative: bool,
}

impl Default for Flags {
    fn default() -> Flags {
        Flags {
            carry: false,
            zero: false,
            interrupt_disable: false,
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

    pub fn set_zero_from_byte(&mut self, byte: u8) {
        self.zero = byte == 0;
    }

    pub fn set_negative_from_byte(&mut self, byte: u8) {
        self.negative = byte >> 7 & 1 == 1;
    }

    pub fn set_carry(&mut self, carry: bool) {
        self.carry = carry;
    }

    pub fn set_overflow(&mut self, overflow: bool) {
        self.overflow = overflow;
    }
}

impl From<u8> for Flags {
    fn from(flags: u8) -> Self {
        Flags {
            carry: flags & 1 == 1,
            zero: (flags >> 1) & 1 == 1,
            interrupt_disable: (flags >> 2) & 1 == 1,
            break_command: (flags >> 4) & 1 == 1,
            overflow: (flags >> 6) & 1 == 1,
            negative: (flags >> 7) & 1 == 1,
        }
    }
}

impl From<Flags> for u8 {
    fn from(flags: Flags) -> u8 {
        let mut result = 0;
        if flags.carry {
            result |= 1;
        }
        if flags.zero {
            result |= 1 << 1;
        }
        if flags.interrupt_disable {
            result |= 1 << 2;
        }
        if flags.break_command {
            result |= 1 << 4;
        }
        if flags.overflow {
            result |= 1 << 6;
        }
        if flags.negative {
            result |= 1 << 7;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn flags_from_u8() {
        let flags = Flags::from(0b1111_0000);

        assert_eq!(flags.negative, true);
        assert_eq!(flags.overflow, true);
        assert_eq!(flags.break_command, true);
        assert_eq!(flags.interrupt_disable, false);
        assert_eq!(flags.zero, false);
        assert_eq!(flags.carry, false);

        let flags = Flags::from(0b0000_1111);
        assert_eq!(flags.negative, false);
        assert_eq!(flags.overflow, false);
        assert_eq!(flags.break_command, false);
        assert_eq!(flags.interrupt_disable, true);
        assert_eq!(flags.zero, true);
        assert_eq!(flags.carry, true);
    }

    #[test]
    fn flags_into_u8() {
        let flags = Flags {
            carry: true,
            zero: true,
            interrupt_disable: true,
            break_command: false,
            overflow: false,
            negative: false,
        };

        assert_eq!(u8::from(flags), 0b0000_0111);

        let flags = Flags {
            carry: false,
            zero: false,
            interrupt_disable: false,
            break_command: true,
            overflow: true,
            negative: true,
        };

        assert_eq!(u8::from(flags), 0b1101_0000);
    }
}
