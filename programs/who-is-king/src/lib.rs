use anchor_lang::prelude::*;

declare_id!("D3vLgy2e7MYRgsH4Q4vSxyUrRkpykad1yoids7d9WWne");

#[program]
pub mod who_is_king {
    use super::*;
    pub fn guess_king(_ctx: Context<Guess>, perobly: String) -> ProgramResult {

        if perobly != "0xdeep".to_string() {
            return Err(print_error!(KingError::NGMIBruh)().into())
        }
        msg!("You are king ser 👑");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Guess {}

#[error]
pub enum KingError {
    #[msg("Afraid to tell, you are NGMI 😜")]
    NGMIBruh,
}

#[macro_export]
macro_rules! print_error {
    ($err:expr) => {{
        || {
            let error_code: KingError = $err;
            msg!("{:?} thrown at {}:{}", error_code, file!(), line!());
            $err
        }
    }};
}