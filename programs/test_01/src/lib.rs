use anchor_lang::{prelude::*, solana_program::system_program};
use std::str;

declare_id!("7Hs1xi3oK5esA1UjmbTf6AwkEhZPtj4cx75M9awCbMzW");

#[program]
pub mod test_01 {


    use super::*;

    pub fn create_account(ctx: Context<CreateAccount>) -> Result<()> {
        let test = ctx.accounts.data_holder.load_mut()?.long_string[2];
        msg!("{:?}", test);
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let number = ctx.accounts.data_holder.load_mut()?.long_string[1];
        msg!("Number: {:?}", number);
        // ctx.accounts.data_holder.load_mut()?.listofusers[1] = ctx.accounts.signer.key();
        // msg!("Array: {:?}", ctx.accounts.data_holder.load_mut()?.listofusers.copy_from_slice(&[ctx.accounts.data_holder.load_mut()?.listofusers[1]]));
        Ok(())
    }
}


#[derive(Accounts)] 
pub struct CreateAccount<'info> {
    #[account(
        init_if_needed, 
        seeds = [b"data_holder_drou", 
        signer.key().as_ref()], 
        bump, 
        payer=signer, 
        space= 10 * 1024 as usize)]
    pub data_holder: AccountLoader<'info, DataHolder>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub data_holder: AccountLoader<'info, DataHolder>,
    #[account(mut)]
    pub signer: Signer<'info>,
}


 #[account(zero_copy)]
 #[repr(C)]
 pub struct DataHolder {
     // 40952 = 40960 - 8 (account desciminator)
     pub long_string: [u8; 10228],
 }
