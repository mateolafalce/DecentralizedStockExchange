use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("The company name must have less than or equal to 50 characters")]NameError,
    #[msg("The description of the company must have less than or equal to 200 characters")]DescriptionError,
    #[msg("The company cannot go public in the past")]Date,
    #[msg("The pubkey is not the same")]PubkeyError,
    #[msg("The amount you intend to purchase is greater than what is currently")]SupplyError,
    #[msg("Holder accounts are not the same")]HolderError,
    #[msg("Cannot be purchased for the equivalent of 0 shares")]AmountError,
    #[msg("The quantity you are specifying does not exist")]PriceError,
    #[msg("Prices cannot be repeated")]UniquePriceError,
}