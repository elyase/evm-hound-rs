use std::collections::HashSet;

use crate::{disasm::disasm, opcodes::Opcode, utils};

pub fn selectors_from_bytecode(code: &[u8]) -> Vec<String> {
    let bytecode = disasm(code);

    let mut selectors: HashSet<String> = HashSet::new();

    let mut i = 4usize;

    while i < bytecode.len() {
        let five_seq = &bytecode[i - 4..i + 1];
        i += 1;

        //  We're looking for a pattern that looks like:
        //  DUP1 PUSH4 <SELECTOR> EQ PUSH2/3 <OFFSET> JUMPI
        //  https://github.com/ethereum/solidity/blob/develop/libsolidity/codegen/ContractCompiler.cpp

        if five_seq[0].opcode == Opcode::Dup1
            && five_seq[1].opcode == Opcode::Push4
            && five_seq[2].opcode == Opcode::Eq
            && five_seq[3].opcode.is_value_push()
            && five_seq[4].opcode == Opcode::Jumpi
        {
            let value = five_seq[1].push_value.unwrap();
            let selector = format!("0x{}", utils::bytes_to_hex(value));

            selectors.insert(selector);
        }
    }

    selectors.into_iter().collect()
}