use anchor_lang::prelude::*;

#[account]
pub struct Offer {
    /// Collection
    pub collection: Pubkey,

    /// Offer Amount
    pub offer_lamport_amount: u64,

    /// Repay Amount
    pub repay_lamport_amount: u64,

    /// Lender
    pub lender: Pubkey,

    /// Loan Taken
    pub is_loan_taken: bool,

    /// Borrower
    pub borrower: Pubkey,

    /// Bump
    pub bump: u8,
}

impl Offer {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 32 + 1 + 32 + 1;
}

#[account]
pub struct Vault {
    /// The offer this vault is linked to
    pub offer: Pubkey,

    /// Bump
    pub bump: u8,
}

impl Vault {
    pub const LEN: usize = 8 + 32 + 1;
}

#[account]
pub struct ActiveLoan {
    /// Collection
    pub collection: Pubkey,

    /// Offer Account
    pub offer_account: Pubkey,

    /// Lender
    pub lender: Pubkey,

    /// Borrower
    pub borrower: Pubkey,

    /// NFT Mint
    pub mint: Pubkey,

    /// Loan Taken Timestamp
    pub loan_ts: i64,

    /// Repayment Timestamp
    pub repay_ts: i64,

    /// Repaid
    pub is_repaid: bool,

    /// Liquidated
    pub is_liquidated: bool,

    /// Bump
    pub bump: u8,
}

impl ActiveLoan {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 32 + 32 + 8 + 8 + 1 + 1 + 1;
}