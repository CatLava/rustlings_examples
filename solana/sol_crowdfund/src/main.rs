// crate definitions
use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use std::convert::TryInto;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
}


// Program for Solana blockchain
// Pubkey references public key of wallet
// This is the program data struct for usage
\#\[derive(BorshSerialize, BorshDeserialize, Debug)\]
pub struct Campaign_account{
    pub campaign_owner: PubKey,
    pub campaign_amount: u64,
    pub campaign_description: String,
    pub campaign_fulfilled: u64,
}

entrypoint!(process_instruction);

//should this be a func? looks like an error
// func goes as fn in Rust
// this is Sol specific defintion
pub fn process_instruction (
    program_id : &PubKey,
    accounts : &[AccountInfo],
    data : &[u8],
) -> ProgramResult {

    // Iterate available accounts
    let accounts_iter = &mut accounts.iter();
    let campaign_account = next_account_info(accounts_iter)?;

    let (instruction_byte, rest_of_data) = data.split_first().unwrap();

    let amount = rest_of_data
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .unwrap();


    // Converting rest of data into strin
    let description = String::from_utf8(rest_of_data\[9..\].to_vec()).unwrap()


    if *instruction_byte == 0 {
    // create campaign
        let campaign_owner_account = next_account_info(accounts_iter)?;

        // Now we need to store campaign object in Hashmap
        // Hashmap is defined at the top of this article
        let mut campaign_account_data = CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;
        campaign_account_data.campaign_amount = amount;
        campaign_account_data.campaign_description = description;
        campaign_account_data.campaign.fulfilled = 0;
        campaign_account_data.serialize(&mut &mut campaign_account.data.borrow_mut()[..])?;


    }

    if instruction_byte == 1 {
        // fund a campaign
    }

    if *instruction_byte == 2 {
        // return funds left in account
        // lets check status of the campaign_owner
        // ? is a match check
        let mut campaign_account_data = CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;

         msg!("{}",campaign_account_data.campaign_amount - campaign_account_data.campaign_fulfilled);

    }

    if *instruction_byte == 3 {
        // withdraw funds and close campaign
    }
Ok(())

}
