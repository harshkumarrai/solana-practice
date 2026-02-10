use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
declare_id!("8cNJ4c4aqQu5dCa58UA6akJRDQbbeoWxt5cuDShvb8Ff");
//cpi 
#[program]
pub mod cpi_using_anchor {
    use super::*;
    pub fn init(ctx: Context<sol_transfer>,amt : u64)->Result<()>{
        let sender=ctx.accounts.sender.to_account_info();
        let receiver=ctx.accounts.receipent.to_account_info();
        let program_id=ctx.accounts.system_program.to_account_info();
        let inst= CpiContext::new(program_id, 
        Transfer{
            from:sender,
            to : receiver
        },
        );
        
        //cpi call ka context

        transfer(inst, amt);
        Ok(()) 
    }
    
}

#[derive(Accounts)] 
pub struct sol_transfer<'info>{
    #[account(mut)]
    sender :Signer<'info>,
    #[account(mut)]
    receipent : SystemAccount<'info>,
    system_program : Program<'info,System>
}
