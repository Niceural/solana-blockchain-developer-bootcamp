// use Borsh for serializing and deserializing data 
use borsh::{BorshDeserialize, BorshSerialize};
// define the program as solana program that takes the standard parameter inputs
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// creates a struct 'GreetingAccount' that will define how we read and store data in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount{
    pub name: String,
}

// define the entrypoint of the program as the process_instruction function 
entrypoint!(process_instruction);

// defines the skeleton of the process_instruction function 
pub fn process_instruction(
    program_id: &Pubkey, // public key of the account the GM program was loaded into
    accounts: &[AccountInfo], // the account to say GM to
    input: &[u8],
) -> ProgramResult {
    msg!("GM program entry point");

    // iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // get the account to say gm to
    let account = next_account_info(accounts_iter)?;

    // the account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // deserialize the input data, and store it in a GreetingAccount struct
    let input_data = GreetingAccount::try_from_slice(&input).unwrap();

    // say gm in the program output
    msg!("GM {}", input_data.name);

    // serialize the name, and store it in the passed in account
    input_data.serialize(&mut &mut account.try_borrow_mut_data()?[..])?;

    Ok(())
}