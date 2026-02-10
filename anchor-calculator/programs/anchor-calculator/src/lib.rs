use anchor_lang::prelude::*;

declare_id!("7rKmw9oiebMvu2CkSX4p82wyd6tHKdsgJ7KoRL2xnMFk");

#[program]
pub mod anchor_calculator {
    use anchor_lang::context;

    use super::*;

    pub fn init(ctx:Context<Initialize>,init_value :u32)-> Result<()>{
        ctx.accounts.account.num=init_value;
        Ok(())



    }
    pub fn add(ctx: Context<Add>, value:u32)-> Result<()>{
            ctx.accounts.account.num+=value;
             Ok(())
    }
     pub fn double(ctx: Context<Double>)-> Result<()>{
        ctx.accounts.account.num*=2;
         Ok(())
    }
    

}

#[account]
pub struct Datashape{
    pub num : u32
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer=signer,space = 8+4)]
    pub account : Account<'info,Datashape>,
    pub system_program : Program<'info, System>,
    #[account(mut)]
    pub signer: Signer<'info>


}
#[derive(Accounts)]
pub struct Double<'info> {
       #[account(mut)]
    pub account : Account<'info,Datashape>,
    #[account(mut)]
    pub signer: Signer<'info>


}
#[derive(Accounts)]
pub struct Add<'info> {
       #[account(mut)]
    pub account : Account<'info,Datashape>,
    #[account(mut)]
    pub signer: Signer<'info>


}


