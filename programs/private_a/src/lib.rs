use structs::*;
use functions::*;
mod structs;
mod constants;
mod functions;

use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;

use anchor_lang::prelude::*;

declare_id!("6rVFHSb2cqn1VXB9iE9P8nRxdTP3mbgGFZfwXDz2Q7c9");

#[program]
pub mod private_a {
    use anchor_spl::token::accessor::amount;
    use anchor_spl::token::TransferChecked;
    use super::*;

    use crate::constants::{SECONDS_IN_YEAR, CLIFF_IN_SECONDS, Errors};

    /*
        Admin functions for loading tokens to smart contract,
        Must be use first only once
    */
    pub fn init(ctx: Context<InitMain>, amount: u64) -> Result<()> {
        validate_init(
            amount,
            &ctx.accounts.sales_mint_of_token.key(),
            &ctx.accounts.admin.key(),
            &ctx.accounts.mint_of_token.key()
        )?;

        let transfer_inst = Transfer {
            from: ctx.accounts.admin_token_ata.to_account_info(),
            to: ctx.accounts.mint_bank.to_account_info(),
            authority: ctx.accounts.admin.to_account_info()
        };

        let binding = [ctx.bumps.mint_bank];
        let seed = vec![
            b"mint_bank",
            binding.as_slice()
        ];
        let outer = vec![seed.as_slice()];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_inst,
            outer.as_slice()
        );

        anchor_spl::token::transfer(cpi_ctx, amount)?;

        ctx.accounts.mint_stat.total_amount = amount;
        ctx.accounts.mint_stat.amount_left = amount;

        ctx.accounts.mint_stat.start = Clock::get().unwrap().unix_timestamp + CLIFF_IN_SECONDS;
        ctx.accounts.mint_stat.end = ctx.accounts.mint_stat.start + SECONDS_IN_YEAR;

        Ok(())
    }

    /*
        User function for minting tokens
    */
    pub fn mint(ctx: Context<Mint>, amount: u64) -> Result<()> {
        let amount_with_reward = (amount as f64 * 1.1) as u64;

        validate_mint(
            amount,
            &ctx.accounts.sales_mint_of_token.key(),
            ctx.accounts.mint_stat.amount_left,
            amount_with_reward
        )?;

        let purchase_price = calculate_purchase_price(amount);
        require!(purchase_price > 0, Errors::ProvidedAmountTooSmall);

        let transfer_to_bank_inst = Transfer {
            from: ctx.accounts.user_sales_token_ata.to_account_info(),
            to: ctx.accounts.sales_mint_bank.to_account_info(),
            authority: ctx.accounts.user.to_account_info()
        };

        let binding = [ctx.bumps.sales_mint_bank];
        let seed = vec![
            b"sales_mint_bank",
            binding.as_slice()
        ];
        let outer = vec![seed.as_slice()];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_to_bank_inst,
            outer.as_slice()
        );

        anchor_spl::token::transfer(cpi_ctx, purchase_price)?;

        ctx.accounts.mint_stat.amount_left -= amount_with_reward;
        ctx.accounts.user_mint_stat.token_amount = amount_with_reward;
        ctx.accounts.mint_stat.profit += purchase_price;

        Ok(())
    }

    /*
        User function to withdraw minting tokens
    */
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        validate_withdraw(&ctx.accounts.mint_of_token.key())?;

        if ctx.accounts.user_mint_stat.last_reward >= ctx.accounts.mint_stat.end {
            return Ok(());
        }

        let amount = calculate_withdraw_amount(
            &ctx.accounts.user_mint_stat,
            &ctx.accounts.mint_stat
        );

        if amount == 0 {
            return Ok(())
        }

        let transfer_inst = TransferChecked {
            from: ctx.accounts.mint_bank.to_account_info(),
            to: ctx.accounts.user_token_ata.to_account_info(),
            authority: ctx.accounts.mint_bank.to_account_info(),
            mint: ctx.accounts.mint_of_token.to_account_info()
        };

        let binding = [ctx.bumps.mint_bank];
        let seed = vec![
            b"mint_bank",
            binding.as_slice()
        ];
        let outer = vec![seed.as_slice()];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_inst,
            outer.as_slice()
        );

        anchor_spl::token::transfer_checked(
            cpi_ctx,
            amount,
            ctx.accounts.mint_of_token.decimals
        )?;

        ctx.accounts.user_mint_stat.last_reward = Clock::get().unwrap().unix_timestamp;

        Ok(())
    }

    /*
        Admin function to withdraw earned USDC from contract
    */
    pub fn withdraw_profit(ctx: Context<WithdrawProfit>) -> Result<()> {
        validate_withdraw_profit(
            &ctx.accounts.sales_mint_of_token.key(),
            &ctx.accounts.admin.key()
        )?;

        if ctx.accounts.mint_stat.profit == 0 {
            return Ok(());
        }

        let transfer_inst = TransferChecked {
            from: ctx.accounts.sales_mint_bank.to_account_info(),
            to: ctx.accounts.admin_sales_token_ata.to_account_info(),
            authority: ctx.accounts.sales_mint_bank.to_account_info(),
            mint: ctx.accounts.sales_mint_of_token.to_account_info()
        };

        let binding = [ctx.bumps.sales_mint_bank];
        let seed = vec![
            b"sales_mint_bank",
            binding.as_slice()
        ];
        let outer = vec![seed.as_slice()];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_inst,
            outer.as_slice()
        );

        anchor_spl::token::transfer_checked(
            cpi_ctx,
            ctx.accounts.mint_stat.profit,
            ctx.accounts.sales_mint_of_token.decimals
        )?;

        ctx.accounts.mint_stat.profit = 0;

        Ok(())
    }
}

