use std::collections::btree_map::Values;
use solana_program::entrypoint;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info, next_account_infos},
    entrypoint::{ ProgramResult},
    msg,
    pubkey::Pubkey
};
entrypoint!(process_instructoion);

#[derive(BorshSerialize,BorshDeserialize)]
struct  Counter{
    count:u32
}

#[derive(BorshSerialize,BorshDeserialize)]
enum Counter_instruction{
    Increment(u32),
    Decrement(u32)
}
pub fn process_instructoion(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data:&[u8]
)->ProgramResult{
    let acc=next_account_info(&mut accounts.iter())?;
    let mut Counter=Counter::try_from_slice(&acc.data.borrow())?;
    let c=Counter_instruction::try_from_slice(instruction_data)?;
    match c{
        Counter_instruction::Increment(Values)=>{
               Counter.count+=Values;
           
        }
        Counter_instruction::Decrement(Values)=>{
            Counter.count-=Values;
           
        }
        

    }
    Counter.serialize(&mut *acc.data.borrow_mut());
    msg!("counter new value {}",Counter.count);
    Ok(())
}
