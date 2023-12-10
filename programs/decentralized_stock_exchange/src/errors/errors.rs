use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Argument too long")]
    SizeError,
    #[msg("Error with lamports amount")]
    AmountError,
    #[msg("The company cannot go public in the past")]
    Date,
    #[msg("The pubkey is not the same")]
    PubkeyError,
    #[msg("The amount you intend to purchase is greater than what is currently")]
    SupplyError,
    #[msg("Holder accounts are not the same")]
    HolderError,
    #[msg("The quantity you are specifying does not exist")]
    PriceError,
    #[msg("Prices cannot be repeated")]
    UniquePriceError,
}
