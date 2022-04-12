use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;
pub use crate::events::*;
pub use crate::migrate::*;

mod internal;
mod approval; 
mod enumeration; 
mod metadata; 
mod mint; 
mod nft_core; 
mod royalty; 
mod events;
mod migrate;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "nft-1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

const DATA_IMAGE_SVG_NATIVO_ICON: &str = "data:image/jpeg;base64,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";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct OldContract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of all the token IDs for a given account
    pub tokens_per_creator: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,

    //status of the minter, to lock or unlock the mint of NFTs
    pub status_minter: bool,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of all the token IDs for a given account
    pub tokens_per_creator: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,

    //status of the minter, to lock or unlock the mint of NFTs
    pub status_minter: bool,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensPerCreator,
    TokenPerCreatorInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "Nativo NFT".to_string(),
                symbol: "NATIVO".to_string(),
                icon: Some(DATA_IMAGE_SVG_NATIVO_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_per_creator: LookupMap::new(StorageKey::TokensPerCreator.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id. 
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            status_minter:true,
        };

        //return the Contract object
        this
    }
    pub fn update_metadata_icon(&mut self, icon: String) {
        let mut metadata = self.metadata.get().unwrap();
        metadata.icon = Some(icon);
        self.metadata.set(&metadata);
    }

    //Now we can stop minting NFTs
    pub fn get_status_minter(&self) {
        // validate if the contract already exist,dont create a new one
        env::log_str(&self.status_minter.to_string());
    }
    //Changing the status_minter
    pub fn set_status_minter(&mut self,new_status:bool) {
        self.is_the_owner();
            //if the caller is the owner
        self.status_minter=new_status;
        env::log_str(&self.status_minter.to_string());
    }
    
    //Changing the owner_id
    pub fn get_owner_id(&self,) {
        // validate if the contract already exist,dont create a new one
        env::log_str(&self.owner_id.to_string());
    }
    pub fn set_owner_account(&mut self,new_account:AccountId) {
        self.is_the_owner();
        //if the caller is the owner
        self.owner_id=new_account;
        env::log_str(&self.owner_id.to_string());
    }

    fn is_the_owner(&self)   {
    //validate that only the owner contract add new contract address
        assert_eq!(
            self.owner_id==env::predecessor_account_id(),
            true,
            "¡You are not the contract owner address!"
        );
    }
}