#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    IGL,
    LOAD, // load memory
    ADD,  // add to addresses and output to 1 address
    SUB,  // add but subtract
    DIV,  // divide 2 addresses, result goes to address given, remainder goes to vm remainder.
    JMP,  // jump to the position in the registers value.
    EQ,   // checks the equality of 2 addresses and changes the equality bool.
    NEQ,  // EQ but not
    JEQ,  // jumps if eq bool.
    GTQ,  // greater than or equal to and outputs to the equality bool
    LTQ,  // GTQ but less.
    LT,   // Less than
    GT,   // Greater than
    MUL,  // ADD but multiply.
    JMPB, // jump back
    JMPF, // jump forwards
    ALOC, // ALOCate memory in the heap.
    SPWN, // Spawn a strand
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::LOAD,
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::MUL,
            4 => Opcode::DIV,
            5 => Opcode::HLT,
            6 => Opcode::JMPF,
            7 => Opcode::JMP,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::JEQ,
            12 => Opcode::GTQ,
            13 => Opcode::LTQ,
            14 => Opcode::GT,
            15 => Opcode::LT,
            16 => Opcode::ALOC,
            _ => Opcode::IGL,
        }
    }
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
