use anchor_lang::prelude::*;

declare_id!("FRsXNq1VbUbSgsd4Qx3AzRhzKpf4ctp1CqS2aiz38x91");






const MAX_METADATA_URI_LEN: usize = 200;
const MAX_SYMBOL_LEN: usize = 10;





#[program]
pub mod nft_project {
    use super::*;

    pub fn initialize(ctx: Context<CreateNFT>, mint : Pubkey , owner : Pubkey , metadata_uri : String , symbol : String , seller_fee_basis : u16 )  -> Result<()>{
        
        let init_nft = &mut ctx.accounts.nft_acc;
        init_nft.mint = mint;
        init_nft.owner = owner;
        init_nft.metadata_uri = metadata_uri;
        init_nft.symbol = symbol;
        init_nft.seller_fee_basis = seller_fee_basis;

        Ok(())
    }

    
    pub fn buy(ctx : Context<BuyNFT>) -> Result<()> {
        let nft_acc = &mut ctx.accounts.nft_acc;
        
        nft_acc.owner = *ctx.accounts.buyer.key;

        Ok(())
    }


}

#[derive(Accounts)]
pub struct CreateNFT<'info>{
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 32 + 4 + MAX_METADATA_URI_LEN + 4 + MAX_SYMBOL_LEN + 2
    )]

    pub nft_acc : Account<'info , Nft>,
    
    #[account(mut)]
    
    pub signer : Signer<'info>,
    pub system_program : Program<'info , System>
}


#[derive(Accounts)]
pub struct BuyNFT<'info>{
    #[account(
        mut,
        has_one = owner
    )]

    pub nft_acc : Account<'info , Nft>,

    #[account(signer)]
    /// CHECK: branded unsafe 
    pub owner : AccountInfo<'info>,

    #[account(mut)]
    pub buyer : Signer<'info>,

}




#[account]
pub struct Nft{
    pub mint : Pubkey,
    pub owner : Pubkey,
    pub metadata_uri : String,
    pub symbol : String,
    pub seller_fee_basis : u16
}







