use solana_program::{account_info::{Account, AccountInfo, next_account_info}, address_lookup_table::instruction, entrypoint::ProgramResult, program::invoke_signed, pubkey::{self, Pubkey}, system_instruction::create_account, system_program};
 use solana_program::entrypoint;

entrypoint!(process_instruction);
//create a new pda on chain
pub fn process_instruction(
    program_id:  &Pubkey,
    accounts : &[AccountInfo],
    instruction_data : &[u8]

)->ProgramResult{
    //pda key,user acc key, contract address
    let iter=&mut accounts.iter();
    let pda=next_account_info(iter)?;
    let user_acc=next_account_info(iter)?;
    let system_program=next_account_info(iter)?;
    // let system_program =next_account_info(&mut accounts.iter())?;
    let seeds=&[user_acc.key.as_ref(),b"user"];
    let (pubkey,bump)=Pubkey::find_program_address(seeds, pda.key);
let ix=create_account(
    user_acc.key, 
    pda.key, 
    1000000, 
    8, 
    program_id);

    invoke_signed(&ix, accounts, &[&[user_acc.key.as_ref(), b"user", &[bump]]])


}