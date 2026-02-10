use anchor_lang::prelude::*;

declare_id!("HHPk7jM9igyF7HgwAu9GWdETZecMeDwc4SwfSoRM5TWs");

#[program]
pub mod cpi_using_anchor2 {
    use std::vec;

    use anchor_lang::prelude::{instruction::Instruction, program::invoke};

    use super::*;

    pub fn init(ctx: Context<Initialize>,amt:u32) -> Result<()> {
        let sender=ctx.accounts.sender.to_account_info();
        let receiever=ctx.accounts.receiver.to_account_info();
        let system_program=ctx.accounts.System_program.to_account_info();
        let acc_metadata=vec![
            AccountMeta::new(sender.key(), true),
            AccountMeta::new(receiever.key(), false)
        ];
        let discriminator:u32 =2;
        let mut insdata=Vec::with_capacity(4+8);
        insdata.extend_from_slice(&discriminator.to_le_bytes());
        insdata.extend_from_slice(&amt.to_le_bytes());
        let ins=Instruction{
             program_id: system_program.key(),
    /// Metadata describing accounts that should be passed to the program.
     accounts: acc_metadata,
    /// Opaque data passed to the program for its own interpretation.
     data: insdata,
        };
        invoke(&ins, &[sender,receiever,system_program])?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info>{
#[account(mut)]
sender : Signer<'info>,
#[account(mut)]

receiver : SystemAccount<'info>,
System_program : Program<'info,System>

}
