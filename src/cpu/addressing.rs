/// The addressing modes supported by the CPU
///
/// Explanation of each addressing mode is copied from
/// [obelisk.me.uk](http://www.obelisk.me.uk/6502/addressing.html#IMM)
#[derive(Debug)]
pub enum Addressing {
    /// # Absolute Addressing
    /// Instructions using absolute addressing contain a full 16 bit address to identify the target location.
    ///
    /// # Example
    ///
    /// ```norun
    /// JMP $1234       ;Jump to location $1234
    /// JSR WIBBLE      ;Call subroutine WIBBLE
    /// ```
    Absolute,
    /// # X Indexed Addressing
    /// Instructions using X indexed absolute addressing contain a full 16-bit address and add the
    /// contents of the X register. If X contains $13 and an instruction provides the address
    /// `$1000`, then `$1013` will be used.
    ///
    /// # Example
    /// ```norun
    /// STA $3000,X     ;Store accumulator between $3000 and $30FF
    /// ROR CRC,X       ;Rotate right one bit
    /// ```
    AbsoluteX,
    /// Y Indexed Addressing
    /// The Y register indexed absolute addressing mode is the same as the previous mode only with
    /// the contents of the Y register added to the 16 bit address from the instruction.
    ///
    /// # Example
    /// ```norun
    /// AND $4000,Y     ;Perform a logical AND with a byte of memory
    /// STA MEM,Y       ;Store accumulator in memory
    /// ```
    AbsoluteY,
    /// # Immediate Addressing
    /// Immediate addressing allows the programmer to directly specify an 8 bit constant within the
    /// instruction. It is indicated by a '#' symbol followed by an numeric expression.
    ///
    /// # Example
    /// ```norun
    /// LDA #10         ;Load 10 ($0A) into the accumulator
    /// LDX #LO LABEL   ;Load the LSB of a 16 bit address into X
    /// LDY #HI LABEL   ;Load the MSB of a 16 bit address into Y
    /// ```
    Immediate,
    /// # Indirect Addressing
    /// JMP is the only 6502 instruction to support indirection. The instruction contains a 16 bit
    /// address which identifies the location of the least significant byte of another 16 bit
    /// memory address which is the real target of the instruction.
    ///
    /// # Example
    /// If location `$0120` contains `$FC` and location `$0121` contains `$BA` then the instruction
    /// `JMP ($0120)` will cause the next instruction execution to occur at `$BAFC` (e.g.  the
    /// contents of `$0120` and `$0121`).
    ///
    /// ```norun
    /// JMP ($FFFC)     ;Force a power on reset
    /// JMP (TARGET)    ;Jump via a labelled memory area
    /// ```
    Indirect,
    /// # Pre-Indexed Indirect Addressing
    /// Indexed indirect addressing is normally used in conjunction with a table of address held on
    /// zero page. The address of the table is taken from the instruction and the X register added
    /// to it (with zero page wrap around) to give the location of the least significant byte of
    /// the target address.
    ///
    /// # Example
    /// ```norun
    /// LDA ($40,X)     ;Load a byte indirectly from memory
    /// STA (MEM,X)     ;Store accumulator indirectly into memory
    /// ```
    IndirectX,
    /// # Post-Indexed Indirect Addressing
    /// Indirect indirect addressing is the most common indirection mode used on the 6502. In
    /// instruction contains the zero page location of the least significant byte of 16 bit
    /// address. The Y register is dynamically added to this value to generated the actual target
    /// address for operation.
    ///
    /// # Example
    /// ```norun
    /// LDA ($40),Y     ;Load a byte indirectly from memory
    /// STA (DST),Y     ;Store accumulator indirectly into memory
    /// ```
    IndirectY,
    /// # Relative addressing
    /// Relative addressing mode is used by branch instructions (e.g. `BEQ`, `BNE`, etc.) which
    /// contain a signed 8 bit relative offset (e.g. `-128` to `+127`) which is added to program
    /// counter if the condition is true. As the program counter itself is incremented during
    /// instruction execution by two the effective address range for the target instruction must be
    /// with `-126` to `+129` bytes of the branch.
    ///
    /// # Example
    /// ```norun
    /// BEQ LABEL       ;Branch if zero flag set to LABEL
    /// BNE *+4         ;Skip over the following 2 byte instruction
    /// ```
    Relative,
    /// # Zero-Page Addressing
    /// An instruction using zero page addressing mode has only an 8 bit address operand. This
    /// limits it to addressing only the first 256 bytes of memory (e.g. `$0000` to `$00FF`) where
    /// the most significant byte of the address is always zero. In zero page mode only the least
    /// significant byte of the address is held in the instruction making it shorter by one byte
    /// (important for space saving) and one less memory fetch during execution (important for
    /// speed).
    ///
    /// An assembler will automatically select zero page addressing mode if the operand evaluates
    /// to a zero page address and the instruction supports the mode (not all do).
    ///
    /// # Example
    /// ```norun
    /// LDA $00         ;Load accumulator from $00
    /// ASL ANSWER      ;Shift labelled location ANSWER left
    /// ```
    ZeroPage,
    /// # Zero-Page X Indexed Addressing
    /// The address to be accessed by an instruction using indexed zero page addressing is
    /// calculated by taking the 8 bit zero page address from the instruction and adding the
    /// current value of the X register to it. For example if the X register contains `$0F` and the
    /// instruction `LDA $80,X` is executed then the accumulator will be loaded from $008F (e.g.
    /// `$80 + $0F => $8F`).
    ///
    /// The address calculation wraps around if the sum of the base address and the register exceed
    /// `$FF`. If we repeat the last example but with `$FF` in the X register then the accumulator
    /// will be loaded from `$007F` (e.g. `$80 + $FF => $7F`) and not `$017F`.
    ///
    /// # Example
    /// ```norun
    /// STY $10,X       ;Save the Y register at location on zero page
    /// AND TEMP,X      ;Logical AND accumulator with a zero page value
    /// ```
    ZeroPageX,
    /// # Zero-Page Y Indexed Addressing
    /// The address to be accessed by an instruction using indexed zero page addressing is
    /// calculated by taking the 8 bit zero page address from the instruction and adding the
    /// current value of the Y register to it. This mode can only be used with the LDX and STX
    /// instructions.
    ///
    /// # Example
    /// ```norun
    /// LDX $10,Y       ;Load the X register from a location on zero page
    /// STX TEMP,Y      ;Store the X register in a location on zero page
    /// ```
    ZeroPageY,
}
