use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata,
    },
    token::{self, FreezeAccount, Mint, ThawAccount, Token, TokenAccount},
};

declare_id!("");

mod admin {
    use anchor_lang::declare_id;
    declare_id!("");
}

/// The program derived address seed.
#[program]
pub mod pledge_contract {

    use super::*;

    /// Initializes a new mint and account.
    pub fn initialize_mint(ctx: Context<InitalizeMint>) -> Result<()> {
        create_metadata_accounts_v3(
            CpiContext::new(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.metadata.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    mint_authority: ctx.accounts.mint_authority.to_account_info(),
                    update_authority: ctx.accounts.mint_authority.to_account_info(),
                    payer: ctx.accounts.mint_authority.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            DataV2 {
                name: String::from("MOMO KIWI GOLD"),
                symbol: String::from("KIWIG"),
                uri: String::new(),
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            true,
            true,
            None,
        )?;

        Ok(())
    }

    /// Mint tokens
    pub fn mint_to(ctx: Context<MintTo>, claim_id: u32, amount: u64) -> Result<()> {
        if ctx.accounts.user_info.claim_nonce != claim_id {
            return Err(ErrorCode::ClaimIdUsed.into());
        }

        let token_account = ctx.accounts.to.clone();
        if token_account.is_frozen() {
            // unfreeze account
            let cpi_context = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                ThawAccount {
                    account: ctx.accounts.to.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            );
            token::thaw_account(cpi_context)?;
        }

        // mint tokens
        let cpi_accounts = anchor_spl::token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        anchor_spl::token::mint_to(cpi_ctx, amount)?;

        // freeze account
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            FreezeAccount {
                account: ctx.accounts.to.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        );
        token::freeze_account(cpi_context)?;

        ctx.accounts.user_info.claim_nonce += 1;
        Ok(())
    }

}

/// Initializes a new mint and account.
#[derive(Accounts)]
pub struct InitalizeMint<'info> {
    #[account(mut, address = admin::ID)]
    pub mint_authority: Signer<'info>,
    #[account(init, payer = mint_authority, mint::decimals = 2, mint::authority = mint_authority, mint::freeze_authority = mint_authority)]
    pub mint: Account<'info, Mint>,
    /// CHECK
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

/// Mint tokens
#[derive(Accounts)]
pub struct MintTo<'info> {
    #[account(address = admin::ID)]
    pub mint_authority: Signer<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, mint::authority = mint_authority)]
    pub mint: Account<'info, Mint>,
    #[account(init_if_needed, payer = user, associated_token::mint = mint, associated_token::authority = user)]
    pub to: Account<'info, TokenAccount>,
    #[account(init_if_needed, payer = user, seeds = [user.key().as_ref()], bump, space = 8 + 4)]
    pub user_info: Account<'info, UserInfo>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

/// User info
#[account]
pub struct UserInfo {
    pub claim_nonce: u32,
}

/// Error
#[error_code]
pub enum ErrorCode {
    #[msg("ClaimId is already in use")]
    ClaimIdUsed,
}
