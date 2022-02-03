/*
    Future works:
    - See, https://docs.near.org/docs/api/naj-cookbook#wrap-and-unwrap-near for NEAR to wNEAR
    - Integrate expiry system, using
        use std::time::SystemTime;   // Will need to getting over due escrows
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).as_sec();

*/

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{
    env, near_bindgen, AccountId, setup_alloc, PanicOnDefault,
    Promise, ext_contract, Gas, PromiseResult, Balance
};
use std::collections::HashMap;
use near_contract_standards::non_fungible_token::{TokenId}; 

setup_alloc!();

const MAX_GAS: Gas = 30_000_000_000_000 ;
const NO_DEPOSIT: Balance = 0;

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

    nft_address: AccountId,
    token_id: TokenId,

    timestamp: u128, // epoch_time
    escrow_state: EscrowState,

    // escrow metadata
    title: String,
    escrow_type: String,
    description: String,
    requirement: String,
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
                    1, MAX_GAS // deposit, gas
                )
                .then(ext_self::my_callback(&env::current_account_id(), 0, MAX_GAS));

                let to = escrow.client.clone();
                Promise::new(to).transfer(escrow.locked_amount); 

                escrow.locked_amount = 0;
                escrow.escrow_state = EscrowState::CANCEL;
                self.escrow_list.entry(client).or_insert_with(HashMap::new).insert(id, escrow);
            },
            None => {},
        };

    }

    pub fn my_callback(&self) -> bool {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");
        match env::promise_result(0) {
            PromiseResult::NotReady => false,
            PromiseResult::Failed => false,
            PromiseResult::Successful(_result) => true,
        }
    }

    #[payable]   
    pub fn set_nft_deliverable(&mut self, 
        client: AccountId, id: u128,
        nft_address: AccountId, token_id: TokenId
    ) -> Promise {
        if let Some(escrow) = self.get_escrow(client.clone(), id.clone()) {
            assert!(escrow.nft_address == "".to_string(), "Deliverable already set");
        }

        let call = nft_interface::nft_transfer(
            "escrow.artpay.testnet".to_string(), // give to this contract for locking
            token_id.clone(),
            Some(0),
            Some("Transfer Escrow ArtPay".to_string()),
            &nft_address,
            1, MAX_GAS
        );

        let callback = ext_self::my_callback(&env::current_account_id(), NO_DEPOSIT, MAX_GAS);
        let response = call.then(callback);

        self.set_deliverable(client.clone(), id, nft_address.clone(), token_id);
        self.client_approval(client.clone(), id);

        response
    }

    #[payable]
    pub fn create_new_escrow(
        &mut self, 
        contractor: AccountId, 
        timestamp: u128,
        // escrow metadata
        title: String,
        escrow_type: String,
        description: String,
        requirement: String,
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
            nft_address: "".to_string(),
            token_id: "".to_string(),
            timestamp, 
            escrow_state: EscrowState::AWAITING,
            title,
            escrow_type,
            description,
            requirement,
        });

        // Use env::log to record logs permanently to the blockchain!
        env::log(format!("New Escrow create by {}", account_id).as_bytes());
    
        escrow_id
    }

    pub fn client_approval(&mut self, client: AccountId, id: u128) -> bool {
        match self.get_escrow(client.clone(), id.clone()){
            Some(mut escrow) => {
                let account_id = env::signer_account_id();
                assert!(account_id == escrow.contractor, "You are not the client!");
                escrow.client_approval = true;
                self.escrow_list.entry(client).or_insert_with(HashMap::new).insert(id, escrow);
                return true;
            },
            None => return false,
        };
    }

    pub fn contractor_approval(&mut self, client: AccountId, id: u128) -> bool {
        match self.get_escrow(client.clone(), id.clone()){
            Some(mut escrow) => {
                let account_id = env::signer_account_id();
                assert!(account_id == escrow.contractor, "You are not the contractor!");
                escrow.contractor_approval = true;
                self.escrow_list.entry(client).or_insert_with(HashMap::new).insert(id, escrow);
                return true;
            },
            None => return false,
        };
    }

    /*
        Should be internal, ie. remove `pub`
    */
    fn set_deliverable(&mut self, 
        client: AccountId, id: u128,
        nft_address: AccountId, token_id: TokenId
    ) -> bool {
        match self.get_escrow(client.clone(), id.clone()){
            Some(mut escrow) => {
                let account_id = env::signer_account_id();
                assert!(account_id == escrow.contractor, "You are not the contractor!");

                escrow.nft_address = nft_address;
                escrow.token_id = token_id;
                escrow.escrow_state = EscrowState::APPROVAL;
                self.escrow_list.entry(client).or_insert_with(HashMap::new).insert(id, escrow);
                return true;
            },
            None => return false,
        };
    }

    /*
        To delete
    */
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

    /*
        get an escrow for particular account with particular id
    */
    pub fn get_escrow(&self, client: AccountId, id: u128) -> Option<Escrow> {
        match self.escrow_list.get(&client) {
            Some(escrows) => {
                return escrows.get(&id).cloned();
            },
            None => return None,
        };
    }

    /*
        get ALL escrows for particular account
    */
    pub fn get_all_escrow(&self, client: AccountId) -> Vec<Option<Escrow>> {
        let mut vec = Vec::new();
        if let Some(n) = self.n_escrows.get(&client) {
            if let Some(escrows) = self.escrow_list.get(&client.clone()) {
                for id in 0..(*n+1) {  // eg. 0..4 = [0,1,2,3]
                    vec.push(escrows.get(&id).cloned());
                }
            }
        }
        vec
    }
}