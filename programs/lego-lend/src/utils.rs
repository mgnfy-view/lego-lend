pub mod math {
    use crate::constants::general::*;

    pub fn is_zero(x: &u64) -> bool {
        *x == 0
    }

    pub fn exactly_one_zero(x: &u64, y: &u64) -> bool {
        is_zero(x) ^ is_zero(y)
    }

    pub fn l_mul_down(x: &u128, y: &u128) -> u128 {
        mul_div_down(x, y, E9 as u128)
    }

    pub fn mul_div_down(x: &u128, y: &u128, d: u128) -> u128 {
        x.checked_mul(*y).unwrap().checked_div(d as u128).unwrap()
    }

    pub fn l_taylor_compounded(x: &u128, n: &u128) -> u128 {
        let first_term = x.checked_mul(*n).unwrap();
        let second_term = mul_div_down(&first_term, &first_term, 2 * E9 as u128);
        let third_term = mul_div_down(&second_term, &first_term, 3 * E9 as u128);

        first_term + second_term + third_term
    }

    pub fn to_shares_down(assets: &u128, total_assets: &u128, total_shares: &u128) -> u128 {
        mul_div_down(
            assets,
            &(total_assets + VIRTUAL_ASSETS as u128),
            *total_shares + VIRTUAL_SHARES as u128,
        )
    }
}

pub mod general {
    use anchor_lang::{prelude::*, system_program};
    use anchor_spl::{
        token::TokenAccount,
        token_2022::{
            self,
            spl_token_2022::{
                self,
                extension::{ExtensionType, StateWithExtensions},
            },
        },
        token_interface::{
            initialize_account3, spl_token_2022::extension::BaseStateWithExtensions,
            InitializeAccount3,
        },
    };

    pub fn create_token_account<'a>(
        authority: &AccountInfo<'a>,
        payer: &AccountInfo<'a>,
        token_account: &AccountInfo<'a>,
        mint_account: &AccountInfo<'a>,
        system_program: &AccountInfo<'a>,
        token_program: &AccountInfo<'a>,
        signer_seeds: &[&[u8]],
    ) -> Result<()> {
        let space = {
            let mint_info = mint_account.to_account_info();
            if *mint_info.owner == token_2022::Token2022::id() {
                let mint_data = mint_info.try_borrow_data()?;
                let mint_state =
                    StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;
                let mint_extensions = mint_state.get_extension_types()?;
                let required_extensions =
                    ExtensionType::get_required_init_account_extensions(&mint_extensions);
                ExtensionType::try_calculate_account_len::<spl_token_2022::state::Account>(
                    &required_extensions,
                )?
            } else {
                TokenAccount::LEN
            }
        };
        create_or_allocate_account(
            token_program.key,
            payer.to_account_info(),
            system_program.to_account_info(),
            token_account.to_account_info(),
            signer_seeds,
            space,
        )?;
        initialize_account3(CpiContext::new(
            token_program.to_account_info(),
            InitializeAccount3 {
                account: token_account.to_account_info(),
                mint: mint_account.to_account_info(),
                authority: authority.to_account_info(),
            },
        ))
    }

    pub fn create_or_allocate_account<'a>(
        program_id: &Pubkey,
        payer: AccountInfo<'a>,
        system_program: AccountInfo<'a>,
        target_account: AccountInfo<'a>,
        siger_seed: &[&[u8]],
        space: usize,
    ) -> Result<()> {
        let rent = Rent::get()?;
        let current_lamports = target_account.lamports();

        if current_lamports == 0 {
            let lamports = rent.minimum_balance(space);
            let cpi_accounts = system_program::CreateAccount {
                from: payer,
                to: target_account.clone(),
            };
            let cpi_context = CpiContext::new(system_program.clone(), cpi_accounts);
            system_program::create_account(
                cpi_context.with_signer(&[siger_seed]),
                lamports,
                u64::try_from(space).unwrap(),
                program_id,
            )?;
        } else {
            let required_lamports = rent
                .minimum_balance(space)
                .max(1)
                .saturating_sub(current_lamports);
            if required_lamports > 0 {
                let cpi_accounts = system_program::Transfer {
                    from: payer.to_account_info(),
                    to: target_account.clone(),
                };
                let cpi_context = CpiContext::new(system_program.clone(), cpi_accounts);
                system_program::transfer(cpi_context, required_lamports)?;
            }
            let cpi_accounts = system_program::Allocate {
                account_to_allocate: target_account.clone(),
            };
            let cpi_context = CpiContext::new(system_program.clone(), cpi_accounts);
            system_program::allocate(
                cpi_context.with_signer(&[siger_seed]),
                u64::try_from(space).unwrap(),
            )?;

            let cpi_accounts = system_program::Assign {
                account_to_assign: target_account.clone(),
            };
            let cpi_context = CpiContext::new(system_program.clone(), cpi_accounts);
            system_program::assign(cpi_context.with_signer(&[siger_seed]), program_id)?;
        }
        Ok(())
    }
}
