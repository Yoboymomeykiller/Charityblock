use anchor_lang::prelude::*;
declare_id!("7zkgjDgZReMvhMuJtMU6Rx9ijsqKVpCH1sf6GcCp2dnW");
#[program]
pub mod charityblock {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> { Ok(()) }
}
#[derive(Accounts)]
pub struct Initialize {}
