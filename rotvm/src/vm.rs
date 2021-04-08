use crate::instruction::Opcode;

pub struct VM {
    pc: usize, // the current program position, gets changed with each jump and instruction read.
    program: Vec<u8>, // stores the program
    remainder: u32, // stores the remainder for any division that occurs 
    equality: bool, // stores the output of the equality checks
    registers: [i32;32], // the cpu registers, size is known at compile time
    heap : Vec<u8>, // the heap.
    programs : Vec<(bool, usize)>, // the programs, explained more in the docs.
}

impl VM {
    pub fn new() -> VM {
        VM {
            
            program: vec![],
            pc: 0,
            remainder: 0,
            equality: false,
            registers: [0;32],
            heap: vec![],
            programs: vec![],
        }
    }
    /// Loops as long as instructions can be executed.
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    /// Executes one instruction. Meant to allow for more controlled execution of the VM
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                println!("reached load");
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                self.registers[register] = number as i32;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            }
            Opcode::ADD => {
                let register1 = self.next_8_bits() as usize;
                let register2 = self.next_8_bits() as usize;
                let reg3 = self.next_8_bits() as usize;
                self.registers[reg3] = self.registers[register1] + self.registers[register2];
            }
            Opcode::SUB => {
                let register1 = self.next_8_bits() as usize;
                let register2 = self.next_8_bits() as usize;
                let reg3 = self.next_8_bits() as usize;
                self.registers[reg3] = self.registers[register1] - self.registers[register2];
            }
            Opcode::MUL => {
                let register1 = self.next_8_bits() as usize;
                let register2 = self.next_8_bits() as usize;
                let reg3 = self.next_8_bits() as usize;
                self.registers[reg3] = self.registers[register1] * self.registers[register2];
            }
            Opcode::DIV => {
                let register1 = self.next_8_bits() as usize;
                let register2 = self.next_8_bits() as usize;
                let reg3 = self.next_8_bits() as usize;
                self.registers[reg3] = self.registers[register1] / self.registers[register2];
                self.remainder = (self.registers[register1] % self.registers[register2]) as u32;
            }
            Opcode::JMP => {
                let reg1 = self.next_8_bits() as usize;
                self.pc = self.registers[reg1] as usize; // jump to the target.
            }
            Opcode::JMPF => {
                let reg1 = self.next_8_bits() as usize;
                self.pc += self.registers[reg1] as usize;
                // jump to the target.
            }
            Opcode::JMPB => {
                let reg1 = self.next_8_bits() as usize;
                self.pc -= self.registers[reg1] as usize;
                // jump to the target.
            }
            Opcode::EQ => {
                let reg1 = self.next_8_bits() as usize;
                let reg2 = self.next_8_bits() as usize;
                self.equality = self.registers[reg1] == self.registers[reg2];
                println!("{}", self.equality);
                self.pc += 1;
            }
            Opcode::NEQ => {
                let reg1 = self.next_8_bits() as usize;
                let reg2 = self.next_8_bits() as usize;
                self.equality = self.registers[reg1] != self.registers[reg2];
                self.pc += 1;
            }
            Opcode::GT => {
                let reg1 = self.next_8_bits() as usize;
                let reg2 = self.next_8_bits() as usize;
                self.equality = self.registers[reg1] > self.registers[reg2];
                self.pc += 1;
            }
            Opcode::LT => {
                let reg1 = self.next_8_bits() as usize;
                let reg2 = self.next_8_bits() as usize;
                self.equality = self.registers[reg1] < self.registers[reg2];
                self.pc += 1;
            }
            Opcode::GTQ => {
                let reg1 = self.next_8_bits() as usize;
                let reg2 = self.next_8_bits() as usize;
                self.equality = self.registers[reg1] >= self.registers[reg2];
                self.pc += 1;
            }
            Opcode::LTQ => {
                let reg1 = self.next_8_bits() as usize;
                let reg2 = self.next_8_bits() as usize;
                self.equality = self.registers[reg1] <= self.registers[reg2];
                self.pc += 1;
            }
            Opcode::JEQ => {
                if self.equality {
                    let reg1 = self.next_8_bits() as usize;
                    self.pc = self.registers[reg1] as usize;
                    // jump to the target.
                }
            }
            Opcode::ALOC => {
                let reg1 = self.next_8_bits() as usize;
                let bytes = self.registers[reg1];
                let new_end = self.heap.len() as i32+ bytes;
                self.heap.resize(new_end as usize,0);
            }
            Opcode::SPWN => {
                
            }
            Opcode::JOIN => {

            }
            Opcode::EXIT => {

            }
            Opcode::IGL => {
                // handle illegal opcodes
                println!("IGL opcode reached");
                return true;
            }
            
        }
        false
    }
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }
    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    // list of all opcodes:
    //
    #[test]
    fn test_create_vm() {
        // test creating a VM.
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
    }

    // HALT opcode explained :
    // halts the execution at the current opcode.
    #[test]
    fn test_opcode_hlt() {
        // test the HALT opcode
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        // test the ILLEGAL opcode
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }
    // LOAD opcode explained:
    // [opcode, index, number as 2 bytes in little endian.]
    #[test]
    fn test_opcode_load() {
        // test the LOAD opcode
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 1, 244];
        test_vm.run();
        println!("{:?}", test_vm.registers);
        assert_eq!(test_vm.registers[0], 500);
    }
    // ADD SUB MUL and DIV tests are following:
    #[test]
    fn test_opcode_add() {
        // test the ADD opcode
        let mut test_vm = VM::new();
        // load 1 and 1 into the 0 and 1 registers. add them and output to the 2 register.
        test_vm.program = vec![0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 2];
        test_vm.run();
        println!("{:?}", test_vm.registers);
        assert_eq!(test_vm.registers[2], 2);
    }
    #[test]
    fn test_opcode_sub() {
        // test the SUB opcode
        let mut test_vm = VM::new();
        // load 1 and 1 into the 0 and 1 registers. subtract them and output to the 2 register.
        test_vm.program = vec![0, 0, 0, 1, 0, 1, 0, 1, 2, 0, 1, 2];
        test_vm.run();
        println!("{:?}", test_vm.registers);
        assert_eq!(test_vm.registers[2], 0);
    }
    #[test]
    fn test_opcode_mul() {
        // test the MUL opcode
        let mut test_vm = VM::new();
        // load 3 and 3 into the 0 and 1 registers. multiply them and output to the 2 register.
        test_vm.program = vec![0, 0, 0, 3, 0, 1, 0, 3, 3, 0, 1, 2];
        test_vm.run();
        println!("{:?}", test_vm.registers);
        assert_eq!(test_vm.registers[2], 9);
    }
    #[test]
    fn test_opcode_div() {
        // test the DIV opcode
        let mut test_vm = VM::new();
        // load 4 and 3 into the 0 and 1 registers. divide them and output to the 2 register.
        test_vm.registers[0] = 4;
        test_vm.registers[1] = 3;
        test_vm.program = vec![4, 0, 1, 2];
        test_vm.run();
        println!("{}", test_vm.remainder);
        println!("{:?}", test_vm.registers);
        assert_eq!(test_vm.registers[2], 1);
        assert_eq!(test_vm.remainder, 1);
    }
    #[test]
    fn test_opcode_jmp() {
        // test the JMP opcode
        let mut test_vm = VM::new();
        // Jump to the beginning of the 0th instruction

        test_vm.registers[0] = 0;
        test_vm.program = vec![7, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }
    #[test]
    fn test_opcode_eq() {
        // test the EQ opcode
        let mut test_vm = VM::new();
        // test both the true and false cases.

        test_vm.registers[0] = 0;
        test_vm.registers[1] = 1;
        test_vm.program = vec![9, 0, 0, 0, 9, 1, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equality, true);
        test_vm.run_once();
        assert_eq!(test_vm.equality, false);
    }
    #[test]
    fn test_opcode_neq() {
        // test the NEQ opcode
        let mut test_vm = VM::new();
        // test both the true and false cases.

        test_vm.registers[0] = 0;
        test_vm.registers[1] = 1;
        test_vm.program = vec![10, 0, 0, 0, 10, 1, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equality, false);
        test_vm.run_once();
        assert_eq!(test_vm.equality, true);
    }
    #[test]
    fn test_opcode_gt() {
        // test the GT opcode
        let mut test_vm = VM::new();
        // test both the true and false cases.

        test_vm.registers[0] = 0;
        test_vm.registers[1] = 1;
        test_vm.program = vec![14, 0, 0, 0, 14, 1, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equality, false);
        test_vm.run_once();
        assert_eq!(test_vm.equality, true);
    }
    #[test]
    fn test_opcode_lt() {
        // test the LT opcode
        let mut test_vm = VM::new();
        // test both the true and false cases.

        test_vm.registers[0] = 0;
        test_vm.registers[1] = 1;
        test_vm.program = vec![15, 0, 0, 0, 15, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equality, false);
        test_vm.run_once();
        assert_eq!(test_vm.equality, true);
    }
    #[test]
    fn test_opcode_gtq() {
        // test the LT opcode
        let mut test_vm = VM::new();
        // test both the true and false cases.

        test_vm.registers[0] = 0;
        test_vm.registers[1] = 1;
        test_vm.program = vec![12, 1, 1, 0, 12, 1, 0, 0, 12, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equality, true);
        test_vm.run_once();
        assert_eq!(test_vm.equality, true);
        test_vm.run_once();
        assert_eq!(test_vm.equality, false);
    }
}
