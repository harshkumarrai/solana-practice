use anchor_lang::prelude::*;
use anchor_lang::system_program;
declare_id!("25g6ekMicGjL1cto2Qb4h5XR7hsotKkSenPEFSTQLiLr");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        amount: u64,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;

        escrow.initializer = ctx.accounts.initializer.key();
        escrow.taker = ctx.accounts.taker.key();
        escrow.amount = amount;
        escrow.bump = ctx.bumps.escrow;

        // Transfer SOL from initializer -> vault (no signer needed)
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.initializer.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        );

        system_program::transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let escrow = &ctx.accounts.escrow;

        require!(
            ctx.accounts.taker.key() == escrow.taker,
            EscrowError::Unauthorized
        );

        // signer seeds for vault PDA: [b"vault", escrow_pubkey, bump]
    let escrow_key = escrow.key();

let vault_seeds: &[&[u8]] = &[
    b"vault",
    escrow_key.as_ref(),
    &[ctx.bumps.vault],
];

let signer_seeds = &[&vault_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.taker.to_account_info(),
            },
            signer_seeds,
        );

        system_program::transfer(cpi_ctx, escrow.amount)?;
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        let escrow = &ctx.accounts.escrow;

        require!(
            ctx.accounts.initializer.key() == escrow.initializer,
            EscrowError::Unauthorized
        );
let escrow_key = escrow.key();
        let vault_seeds: &[&[u8]] = &[
            b"vault",
            escrow_key.as_ref(),
            &[ctx.bumps.vault],
        ];
        let signer_seeds = &[&vault_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.initializer.to_account_info(),
            },
            signer_seeds,
        );

        system_program::transfer(cpi_ctx, escrow.amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct InitializeEscrow<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    /// CHECK: taker pubkey stored in escrow account, validated in instructions
    pub taker: UncheckedAccount<'info>,

    #[account(
        init,
        payer = initializer,
        space = 8 + Escrow::SIZE,
        seeds = [b"escrow", initializer.key().as_ref(), taker.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    /// Vault is a PDA system account that will hold lamports.
    /// We use UncheckedAccount so Anchor can create it; no data is stored.
    #[account(
        init,
        payer = initializer,
        seeds = [b"vault", escrow.key().as_ref()],
        bump,
        space = 0
    )]
    /// CHECK: vault PDA to hold SOL (owned by system program)
    pub vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(
        mut,
        seeds = [b"escrow", escrow.initializer.as_ref(), escrow.taker.as_ref()],
        bump = escrow.bump,
        close = taker
    )]
    pub escrow: Account<'info, Escrow>,

    /// CHECK: vault PDA; we will transfer lamports out using invoke_signed
    #[account(mut, seeds = [b"vault", escrow.key().as_ref()], bump)]
    pub vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"escrow", escrow.initializer.as_ref(), escrow.taker.as_ref()],
        bump = escrow.bump,
        close = initializer
    )]
    pub escrow: Account<'info, Escrow>,

    /// CHECK: vault PDA
    #[account(mut, seeds = [b"vault", escrow.key().as_ref()], bump)]
    pub vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Escrow {
    pub initializer: Pubkey,
    pub taker: Pubkey,
    pub amount: u64,
    pub bump: u8,
}

impl Escrow {
    pub const SIZE: usize = 32 + 32 + 8 + 1;
}

#[error_code]
pub enum EscrowError {
    #[msg("Unauthorized caller")]
    Unauthorized,
}