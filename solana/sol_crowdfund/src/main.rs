// Program for Solana blockchain
// Pubkey references public key of wallet
pub struct Campaign_account{
    pub campaign_owner: PubKey,
    pub campaign_amount: u64,
    pub campaign_descriptions: String,
    pub campaign_fulfilled: u64,
}

//should this be a func? looks like an error
// func goes as fn in Rust
// this is Sol specific defintion
pub fun process_instruction (
    program_id : &PubKey,
    accounts : &\[AccountInfo\],
    data : &\[u8\],
) -> ProgramResult

let (instruction_byte, all_other_bytes) = data.split_first().unwrap();
if\*instruction_byte == 0{
    // create campaign
}

else if\*instruction_byte == 1{
    // fund a campaign
}

else if\*instruction_byte == 2 {
    // return funds left in account
}

else if\*instruction_byte == 3 {
    // withdraw funds and close campaign
}

let iterable_accounts = &mut accounts.iter();
let campaign_account = next_account_info(iterable_accounts);
