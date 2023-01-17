use crate::*;
use near_sdk::{
     serde_json::json,
};
#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        //we add an optional parameter for perpetual royalties
        perpetual_royalties: Option<HashMap<AccountId, u32>>,
    ) {
        assert_eq!(self.status_minter, true, "The mint of NFTs is unavailable");
        let id: u64 = self.token_metadata_by_id.len()+1;
        let token_id: String = id.to_string();
        let owner_id: AccountId=receiver_id.clone();
        let creator_id: AccountId=receiver_id.clone();
        let approved_account_ids=Default::default();
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        // create a royalty map to store in the token
        let mut royalty = HashMap::new();

        // if perpetual royalties were passed into the function: 
        if let Some(perpetual_royalties) = perpetual_royalties {
            //make sure that the length of the perpetual royalties is below 7 since we won't have enough GAS to pay out that many people
            assert!(perpetual_royalties.len() < 7, "Cannot add more than 6 perpetual royalty amounts");

            //iterate through the perpetual royalties and insert the account and amount in the royalty map
            for (account, amount) in perpetual_royalties {
                royalty.insert(account, amount);
            }
        }
        let royalty_copy=royalty.clone();
        //specify the token struct that contains the owner ID 
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id.clone(),
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //set the creator ID equal to the receiver ID passed into the function
            creator_id: receiver_id.clone(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &metadata );

        //call the internal method for adding the token to the creator
        self.internal_add_token_to_creator(&token.owner_id, &token_id);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        let _data =JsonTokenLog {
            token_id,
            //owner of the token
            owner_id,
            //token metadata
            metadata ,
            //creator of the token
            creator_id,
            //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
            approved_account_ids ,
            //keep track of the royalty percentages for the token in a hash map
            royalty:royalty_copy,
            
        };
        

        let formated_content=&json!({   
            "standard": NFT_STANDARD_NAME.to_string(),
            "version": NFT_METADATA_SPEC.to_string(),
            "event":"NftMintInt",
            "data": { "type": "NftMint","params": _data}
         }).to_string(); 
        //EMIT THE LOG
        env::log_str(&format!("EVENT_JSON:{}",formated_content).to_string(), );
        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());
        

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    }
}