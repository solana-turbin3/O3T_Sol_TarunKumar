use anchor_lang::prelude::*;

mod contexts;
mod states;
mod error;

use contexts::*;

declare_id!("7pDJppGDo1qfd1gUcYjru7dNLUA3w3Zg5324roqL62fe");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_and_close()
    }
}