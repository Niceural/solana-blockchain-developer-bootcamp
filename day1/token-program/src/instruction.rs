use borsh::{BorshDeserialize, BorshSerialize};

// defines an enum that defines all the possible instructions that can be sent to the program
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum TokenInstruction {
    CreateToken,
    CreateTokenAccount,
    Mint { amount : u64 },
    Transfer { amount: u64 }
}