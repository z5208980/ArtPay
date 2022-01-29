use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{
    env, near_bindgen, AccountId, setup_alloc, PanicOnDefault,
    Promise,
    // json_types::U128,
};
// use near_sdk::collections::LookupMap;
use std::collections::HashMap;
// use std::time::SystemTime;   // Will need to getting over due escrows
//  SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).as_sec();
use near_contract_standards::non_fungible_token::{TokenId}; 

use near_sdk::{ext_contract};

setup_alloc!();

#[ext_contract(nft_interface)]
trait NonFungibleToken {
    // change methods
    fn nft_transfer(&mut self, receiver_id: String, token_id: String, approval_id: Option<u64>, memo: Option<String>);
    fn nft_transfer_call(&mut self, receiver_id: String, token_id: String, approval_id: Option<u64>, memo: Option<String>, msg: String) -> bool;

    // view method
    fn nft_token(&self, token_id: String) -> Option<Token>;
}

#[ext_contract(nft_approve_interface)]
trait NonFungibleTokenApprovalManagement: NonFungibleToken {
    // change methods
    fn nft_approve(&mut self, token_id: String, account_id: String, msg: Option<String>);
    fn nft_revoke(&mut self, token_id: String, account_id: String);
    fn nft_revoke_all(&mut self, token_id: String);

    // view methods
    fn nft_is_approved(&self, token_id: String, approved_account_id: String, approval_id: Option<u64>) -> bool;
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Copy, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum EscrowState {
    AWAITING,
    APPROVAL,
    COMPLETE,
    CANCEL,
}

/*
    escrow itself as struct
*/
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Escrow {
    client: AccountId,
    contractor: AccountId,

    locked_amount: u128,

    client_approval: bool,
    contractor_approval: bool,

    nft_address: AccountId, // since contract address === account_address
    token_id: TokenId,

    timestamp: u128, // epoch_time
    escrow_state: EscrowState,
}

#[ext_contract(ext_self)]
pub trait ArtPay {
    fn my_callback(&self) -> String;
}
/*
    This allows for O(1) lookup of a escrow for a particular user
    total no. escrow created,
    total escrows for given account
    mapping of escrow for given account
*/
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ArtPay {
    owner_id: AccountId,
    total_escrows: u128,
    n_escrows: HashMap<AccountId, u128>,   
    escrow_list: HashMap<AccountId, HashMap<u128, Escrow>>,
}

#[near_bindgen]
impl ArtPay {
    #[init]
    pub fn constructor(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            total_escrows: 0,
            n_escrows: HashMap::new(),
            escrow_list: HashMap::new(),
        }
    }

    pub fn cancel(&mut self, client: AccountId, id: u128) {
        match self.get_escrow(client.clone(), id.clone()){
            Some(mut escrow) => {
                let account_id = env::signer_account_id();
                assert!(escrow.client.clone() == account_id, "Not ownership of escrow to cancel");
                nft_interface::nft_transfer(
                    escrow.contractor.to_string(), 
                    escrow.token_id.clone(),
                    Some(0), Some("Transfer Escrow ArtPay".to_string()),
                    &(escrow.nft_address), // nft contract
                    1, 200000000000000 // deposit, gas
                )
                .then(ext_self::my_callback(&env::current_account_id(), 0, 200000000000000));

                let to = escrow.client.clone();
                Promise::new(to).transfer(escrow.locked_amount); 

                escrow.locked_amount = 0;
                escrow.escrow_state = EscrowState::CANCEL;
                self.escrow_list.entry(client).or_insert_with(HashMap::new).insert(id, escrow);
            },
            None => {},
        };

    }
    // REQUIRE FRONTEND APPROVAL FROM NFT OWNER
    pub fn set_nft_deliverable(&self, nft_address: AccountId,  token_id: TokenId,) -> Promise {
        nft_interface::nft_transfer(
            "escrow.artpay.testnet".to_string(), // give to this contract for locking
            token_id.clone(),
            Some(0),
            Some("Transfer Escrow ArtPay".to_string()),
            &nft_address, // nft contract
            1, // yocto NEAR to attach
            200000000000000 // gas to attach
        )
        .then(ext_self::my_callback(&env::current_account_id(), 0, 200000000000000))
    }

    pub fn my_callback(&self) -> bool {
        assert_eq!( env::promise_results_count(), 1, "This is a callback method");
        true
    }

    #[payable]
    pub fn create_new_escrow(
        &mut self, 
        contractor: AccountId, 
        nft_address: AccountId, 
        token_id: TokenId,
        timestamp: u128,
    ) -> u128 {
        let account_id = env::signer_account_id(); // msg.sender
        
        self.total_escrows += 1;

        let mut escrow_id = 0;
        match self.n_escrows.get(&account_id) {
            Some(n) => { 
                escrow_id = n + 1;
                self.n_escrows.insert(account_id.clone(), escrow_id)
            }
            None => self.n_escrows.insert(account_id.clone(), escrow_id)
        };

        self.escrow_list.entry(account_id.clone()).or_insert_with(HashMap::new).insert(escrow_id, Escrow {
            client: account_id.clone(),
            contractor,
            locked_amount: near_sdk::env::attached_deposit(),
            client_approval: false,
            contractor_approval: false,
            nft_address,
            token_id,
            timestamp, 
            escrow_state: EscrowState::AWAITING,
        });

        // Use env::log to record logs permanently to the blockchain!
        env::log(format!("New Escrow create by {}", account_id).as_bytes());
    
        escrow_id
    }

    pub fn client_approval(&mut self, client: AccountId, id: u128) -> bool {
        match self.get_escrow(client.clone(), id.clone()){
            Some(mut escrow) => {
                let account_id = env::signer_account_id();
                if account_id == escrow.client {
                    escrow.client_approval = true;
                    self.escrow_list.entry(client).or_insert_with(HashMap::new).insert(id, escrow);
                    return true;
                };
               return false;
            },
            None => return false,
        };
    }

    pub fn contractor_approval(&mut self, client: AccountId, id: u128) -> bool {
        match self.get_escrow(client.clone(), id.clone()){
            Some(mut escrow) => {
                let account_id = env::signer_account_id();
                if account_id == escrow.contractor {
                    escrow.contractor_approval = true;
                    self.escrow_list.entry(client).or_insert_with(HashMap::new).insert(id, escrow);
                    return true;
                };
               return false;
            },
            None => return false,
        };
    }

    #[payable]
    pub fn take_my_money(&mut self) -> bool {
        true
    }

    #[payable]
    pub fn release_escrow(&mut self, client: AccountId, id: u128) -> bool {
        match self.get_escrow(client.clone(), id.clone()) {
            Some(mut escrow) => {
                if escrow.client_approval && escrow.contractor_approval {
                    let to = escrow.contractor.clone();
                    Promise::new(to).transfer(escrow.locked_amount); // FUNDS RELEASED HERE! pay to contractor the locked funds of this escrow
                    escrow.locked_amount = 0;
                    escrow.escrow_state = EscrowState::COMPLETE;
                    self.escrow_list.entry(client).or_insert_with(HashMap::new).insert(id, escrow);
                    return true;
                } else {
                    return false;
                }
            },
            None => return false,
        };
    }

    pub fn get_escrow(&self, client: AccountId, id: u128) -> Option<Escrow> {
        match self.escrow_list.get(&client) {
            Some(escrows) => {
                return escrows.get(&id).cloned();
            },
            None => return None,
        };
    }
}