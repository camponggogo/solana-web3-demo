use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// here is like a main function
// when an instruction executes, it will run into here first.
// it will also pass three arguments
// - program id => this is the current program id
// - accounts => passed by client, you will need to pass all accounts you need in this operation, you can't load them dynamically
// - instruction_data => passed by client, the data you need, the format will be uint8 array. how to separate them into meaningful segment depends your prgoram

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("program id {:?}", program_id);
    for account in accounts {
        msg!("accounts {:?}", account);
    }
    msg!("data {:?}", instruction_data);
    Ok(())
}
