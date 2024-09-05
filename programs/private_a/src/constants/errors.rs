use anchor_lang::error_code;

#[error_code]
pub enum Errors {
    #[msg("You are not authorized to perform this action")]
    Forbidden,

    #[msg("Invalid token")]
    InvalidToken,

    #[msg("Incorrect amount")]
    IncorrectAmount,

    #[msg("Amount per wallet must be more than 0")]
    IncorrectAmountPerWallet,

    #[msg("Invalid sales token")]
    InvalidSalesToken,

    #[msg("Not enough tokens for a mint")]
    NotEnoughTokensForMint,

    #[msg("Already minted")]
    AlreadyMinted,

    #[msg("Provided amount too small")]
    ProvidedAmountTooSmall
}