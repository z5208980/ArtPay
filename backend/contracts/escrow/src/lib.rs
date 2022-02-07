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

// #[ext_contract(nft_approve_interface)]
// trait NonFungibleTokenApprovalManagement: NonFungibleToken {
//     // change methods
//     fn nft_approve(&mut self, token_id: String, account_id: String, msg: Option<String>);
//     fn nft_revoke(&mut self, token_id: String, account_id: String);
//     fn nft_revoke_all(&mut self, token_id: String);

//     // view methods
//     fn nft_is_approved(&self, token_id: String, approved_account_id: String, approval_id: Option<u64>) -> bool;
// }

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Copy, Clone, Eq, PartialEq)]
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
    escrow_id: u128,
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
    
    license_code: String,  // Store short code abbreviations eg: 'by-nc-sa'
    license_desc: String,  // Human readable
    license_url: String, // link to contract (maybe HTTPS or IPFS address) 
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EscrowCheckin {
    escrow_id: u128,
    update_type: String, // draft, draft#2 etc
    message: String, 
    media_url: String, // url or IPFS storage
    timestamp: u128, // epoch_time
}

#[ext_contract(ext_self)]
pub trait ArtPay {
    fn my_callback(&mut self, client: AccountId, id: u128, nft_address: AccountId, token_id: TokenId) -> bool;
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
    total_checkins: u128,

    n_escrows: HashMap<AccountId, u128>,  // ClientId mapped to Escrow Id   !NOTE: CAN REMOVE
    contractor_escrows: HashMap<AccountId, u128>, // Contractor mapped to Escrow Id !NOTE: CAN REMOVE
    escrow_list: HashMap<AccountId, HashMap<u128, Escrow>>, // Client Account mapping   !NOTE: CAN REMOVE

    n_escrow_checkins: HashMap<u128, u128>,
    escrow_checkins: HashMap<u128, HashMap<u128, EscrowCheckin>>, // escrowId, updateId
    // escrow_checkins: HashMap<String, HashMap<u128, EscrowCheckin>>, // mapping(client+contractor+escrow_id -> mapping(checkin_id -> EscrowCheckin))

    // Updated data structure
    client_contractor: HashMap<AccountId, HashMap<AccountId, u128>>, // mapping(client -> mapping(contractor -> n_escrows))
    contractor_client: HashMap<AccountId, HashMap<AccountId, u128>>, // mapping(contractor -> mapping(client -> n_escrows))
    escrows: HashMap<String, Escrow>, // mapping(client+contractor+escrow_id -> Escrow)


}

#[near_bindgen]
impl ArtPay {
    #[init]
    pub fn constructor(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            total_escrows: 0,
            total_checkins: 0,

            n_escrows: HashMap::new(),
            contractor_escrows: HashMap::new(),
            escrow_list: HashMap::new(),
            n_escrow_checkins: HashMap::new(), // manage cbyount
            escrow_checkins: HashMap::new(),


            client_contractor: HashMap::new(),
            contractor_client: HashMap::new(),
            escrows: HashMap::new(),
        }
    }

    /* Testing payable */
    #[payable]
    pub fn donate_to_artpay(&mut self) -> bool { true }

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
                );
                // .then(ext_self::my_callback(&env::current_account_id(), 0, MAX_GAS));

                let to = escrow.client.clone();
                Promise::new(to).transfer(escrow.locked_amount); 

                escrow.locked_amount = 0;
                escrow.escrow_state = EscrowState::CANCEL;
                self.escrow_list.entry(client).or_insert_with(HashMap::new).insert(id, escrow);
            },
            None => {},
        };

    }

    fn my_callback(&mut self, 
        client: AccountId, id: u128,
        nft_address: AccountId, token_id: TokenId
    ) -> bool {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");
        match env::promise_result(0) {
            PromiseResult::NotReady => nv::panic(b"Error"),
            PromiseResult::Failed => env::panic(b"Error"),
            PromiseResult::Successful(_result) => true,
        }
    }

    #[payable]   
    pub fn set_nft_deliverable(&mut self, 
        client: AccountId, id: u128,
        nft_address: AccountId, token_id: TokenId
    ) -> Promise {
        let contractor = env::current_account_id();
        /* Validation checking */
        // if let Some(escrow) = self.get_escrow(client.clone(), id.clone()) {
        if let Some(escrow) = self.get_escrow_new(client.clone(), contractor.clone(), id.clone()) {
            assert!(escrow.escrow_state == EscrowState::AWAITING, "Wrong State");
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

        let callback = ext_self::my_callback(
            client.clone(), id.clone(), nft_address.clone(), token_id.clone(),
            &contractor
            , NO_DEPOSIT, MAX_GAS
        );
        let response = call.then(callback);

        /* Todo make sure that a false response doesn't set_deliverable! */ 
        self.set_deliverable(client.clone(), id, nft_address.clone(), token_id);

        response
    }

    pub fn create_escrow_checkin(
        &mut self,
        escrow_id: u128,
        update_type: String, // draft, draft#2 etc
        message: String, 
        media_url: String, // url or IPFS storage
        timestamp: u128, // epoch_time

    ) -> u128 {

        let account_id = env::signer_account_id(); // msg.sender is contractor

        /* handle counts of total and checkin id */
        self.total_checkins += 1;

        let mut checkin_id = 0;
        if let Some(n) = self.n_escrow_checkins.get(&escrow_id) {
            checkin_id = n + 1;
        }
        self.n_escrow_checkins.insert(escrow_id, checkin_id); // contractorId mapped
        //TODO: Assert Escrow is related to Contractor (account_id)


        /* add new escrow checkin to list */
        self.escrow_checkins.entry(escrow_id.clone()).or_insert_with(HashMap::new).insert(checkin_id, EscrowCheckin {
            escrow_id,
            update_type, // draft, draft#2 etc
            message, 
            media_url, // url or IPFS storage
            timestamp, // epoch_time
        });
        // Use env::log to record logs permanently to the blockchain!
        env::log(format!("New Escrow Checkin created by {}", account_id).as_bytes());

        checkin_id
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

        license_code: String,  // Store short code abbreviations eg: 'by-nc-sa'
        license_desc: String,  // Human readable
        license_url: String, // link to contract (maybe HTTPS or IPFS address) 

    ) -> u128 {
        let account_id = env::signer_account_id(); // msg.sender

        /* handle counts of total and clients escrows id */
        self.total_escrows += 1;
        // let mut escrow_id = 0;
        // if let Some(n) = self.n_escrows.get(&account_id) {
        //     escrow_id = n + 1;
        // }
        // self.n_escrows.insert(account_id.clone(), escrow_id); // ClientId mapped
        // self.contractor_escrows.insert(contractor.clone(), escrow_id); //ContractorId mapped

        // /* add new escrow to list */
        // self.escrow_list.entry(account_id.clone()).or_insert_with(HashMap::new).insert(escrow_id, Escrow {
        //     client: account_id.clone(),
        //     contractor,
        //     locked_amount: near_sdk::env::attached_deposit(),
        //     client_approval: false,
        //     contractor_approval: false,
        //     nft_address: "".to_string(),
        //     token_id: "".to_string(),
        //     timestamp, 
        //     escrow_state: EscrowState::AWAITING,
        //     title,
        //     escrow_type,
        //     description,
        //     requirement,

        //     license_code, 
        //     license_desc, 
        //     license_url
        // });

        // match self.escrow_list.get(&client) {
        //     Some(escrows) => return escrows.get(&id).cloned(),
        //     None => return None,
        // };

        // With new data struture

        let mut escrow_id = 0;
        if let Some(contractor_escrows) = self.client_contractor.get(&account_id) {
            if let Some(n) = contractor_escrows.get(&contractor).cloned() {
                escrow_id = n + 1;
            }
        }

        // Note client_contractor[client][contractor] == contractor_client[contractor][client]
        self.client_contractor.entry(account_id.clone()).or_insert_with(HashMap::new).insert(contractor.clone(), escrow_id);
        self.contractor_client.entry(contractor.clone()).or_insert_with(HashMap::new).insert(account_id.clone(), escrow_id);


        // Id for escrows is now client+contractor+id
        self.escrows.insert(self.generate_key(account_id.clone(), contractor.clone(), escrow_id), Escrow {
            escrow_id,
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

            license_code, 
            license_desc, 
            license_url
        });

        // Use env::log to record logs permanently to the blockchain!
        env::log(format!("New Escrow create by {}", account_id).as_bytes());
    
        escrow_id
    }

    // pub fn client_approval(&mut self, client: AccountId, id: u128) -> bool {
    //     match self.get_escrow(client.clone(), id.clone()){
    //         Some(mut escrow) => {
    //             assert!(escrow.escrow_state == EscrowState::APPROVAL, "Wrong State");
    //             let account_id = env::signer_account_id();
    //             assert!(account_id == escrow.contractor, "You are not the client");
    //             escrow.client_approval = true;
    //             self.escrow_list.entry(client).or_insert_with(HashMap::new).insert(id, escrow);
    //             return true;
    //         },
    //         None => return false,
    //     };
    // }

    pub fn client_approval(&mut self, contractor: AccountId, id: u128) -> bool {
        let client = env::signer_account_id(); // msg.sender    
        if let Some(mut escrow) = self.get_escrow_new(client.clone(), contractor.clone(), id) {
            assert!(client == escrow.contractor, "You are not the client");
            assert!(escrow.escrow_state == EscrowState::APPROVAL, "Wrong State");
            
            escrow.client_approval = true;
            self.escrows.insert(self.generate_key(client.clone(), contractor.clone(), id), escrow);
            return true;
        };
        false
    }
  
    // pub fn contractor_approval(&mut self, client: AccountId, id: u128) -> bool {
    //     match self.get_escrow(client.clone(), id.clone()){
    //         Some(mut escrow) => {
    //             assert!(escrow.escrow_state == EscrowState::APPROVAL, "Wrong State");
    //             let account_id = env::signer_account_id();
    //             assert!(account_id == escrow.contractor, "You are not the contractor!");
    //             escrow.contractor_approval = true;
    //             self.escrow_list.entry(client).or_insert_with(HashMap::new).insert(id, escrow);
    //             return true;
    //         },
    //         None => return false,
    //     };
    // }

    pub fn contractor_approval(&mut self, client: AccountId, id: u128) -> bool {
        let contractor = env::signer_account_id(); // msg.sender
        if let Some(mut escrow) = self.get_escrow_new(client.clone(), contractor.clone(), id) {
            self.check_state(escrow.clone(), EscrowState::APPROVAL);
            assert!(contractor == escrow.client, "You are not the contractor");
            escrow.contractor_approval = true;
            self.escrows.insert(self.generate_key(client.clone(), contractor.clone(), id), escrow);
            return true;

        };
        false
    }

    /* Internal function for set nft info on escrow */
    fn set_deliverable(&mut self, 
        client: AccountId, id: u128,
        nft_address: AccountId, token_id: TokenId
    ) -> bool {
        let contractor = env::signer_account_id();

        match self.get_escrow_new(client.clone(), contractor.clone(), id.clone()){
            Some(mut escrow) => {
                self.check_state(escrow.clone(), EscrowState::AWAITING);
                assert!(contractor == escrow.contractor, "You are not the contractor!");
                escrow.nft_address = nft_address;
                escrow.token_id = token_id;
                escrow.escrow_state = EscrowState::APPROVAL;
                self.escrows.insert(self.generate_key(client.clone(), contractor.clone(), id.clone()), escrow);
                return true;
            },
            None => return false,
        };
    }

    #[payable]
    pub fn release_escrow(&mut self, client: AccountId, id: u128) -> bool {
        match self.get_escrow(client.clone(), id.clone()) {
            Some(mut escrow) => {
                assert!(escrow.client_approval && escrow.contractor_approval, "Not fully approved");
                let to = escrow.contractor.clone();
                Promise::new(to).transfer(escrow.locked_amount); // FUNDS RELEASED HERE! pay to contractor the locked funds of this escrow
                nft_interface::nft_transfer(
                    escrow.client.to_string(), 
                    escrow.token_id.clone(),
                    Some(0), Some("Transfer Escrow ArtPay".to_string()),
                    &(escrow.nft_address), // nft contract
                    1, MAX_GAS // deposit, gas
                );
                // .then(ext_self::my_callback(&env::current_account_id(), 0, MAX_GAS));

                escrow.locked_amount = 0;
                escrow.escrow_state = EscrowState::COMPLETE;
                self.escrow_list.entry(client).or_insert_with(HashMap::new).insert(id, escrow);
                return true;
            },
            None => {},
        };
        false
    }


    /*
        get checkins associated with an escrow 
    */
    pub fn get_escrow_checkins_list(&self, escrow_id: u128) -> Vec<Option<EscrowCheckin>> {
        let mut vec = Vec::new();
        match self.escrow_checkins.get(&escrow_id) {
            // return a list of checkins as a list 
            Some(checkins) => {
                let c  = checkins.len() as u128;
                for id in 0..c-1 {
                    let cur = checkins.get(&id);
                    vec.push(cur.cloned());
                }
            }
            None => {}
        };
        vec
    }

    /*
        get checkin associated with a checkin id 
    */
    pub fn get_escrow_checkin(&self,  checkin_id: u128) -> Option<EscrowCheckin> {

        match self.n_escrow_checkins.get(&checkin_id) 
        {
            Some(found_escrow_id) => { 
                // we now can traverse the checkins on the found escrow
                //let checkins = self.escrow_checkins.get(&found_escrow_id);
                // locate the one matching the checkin_id
                match self.escrow_checkins.get(&found_escrow_id) {
                    Some(checkins) => return checkins.get(&checkin_id).cloned(),
                    None => return None
                }
            }
            None => return None
        }
    }


    /*
        get an escrow for particular account with particular id
    */
    pub fn get_escrow(&self, client: AccountId, id: u128) -> Option<Escrow> {
        match self.escrow_list.get(&client) {
            Some(escrows) => return escrows.get(&id).cloned(),
            None => return None,
        };
    }

    // Escrow Id in format: {client}{contractor}{id}
    pub fn generate_key(&self, cl: AccountId, con: AccountId, id: u128) -> String {
        let mut key: String = cl.to_string();
        key.push_str(&con.to_string());
        key.push_str(&id.to_string());
        key
    }

    pub fn get_escrow_new(&self, client: AccountId, contractor: AccountId, id: u128) -> Option<Escrow> {
        let key: String = self.generate_key(client, contractor, id);
        if let Some(escrow) = self.escrows.get(&key) {
            return Some(escrow).cloned();
        }
        None
    }

    /* 
        O(n^2) may be costly,but the most simpliest and space efficient structure 
        Get all escrow related to the client
    */
    pub fn get_escrows_as_client(&self) -> Vec<Option<Escrow>>{
        let mut vec = Vec::new();
        let account_id = env::signer_account_id();

        if let Some(contractor_escrows) = self.client_contractor.get(&account_id.clone()) { // Hashmap of contractors
            for (contractor, v) in contractor_escrows.iter() { // Loop through contractors
                for id in 0..(*v+1) {  // eg. 0..4 = [0,1,2,3]
                    vec.push(self.get_escrow_new(account_id.clone(), contractor.to_string(), id));
                }
            }
        }

        vec
    }

    /* 
        O(n^2) may be costly,but the most simpliest and space efficient structure 
        Get all escrows related to the contractor
    */
    pub fn get_escrows_as_contractor(&self) -> Vec<Option<Escrow>>{
        let mut vec = Vec::new();
        let account_id = env::signer_account_id();

        if let Some(contractor_escrows) = self.contractor_client.get(&account_id.clone()) { // Hashmap of contractors
            for (contractor, v) in contractor_escrows.iter() { // Loop through contractors
                for id in 0..(*v+1) {  // eg. 0..4 = [0,1,2,3]
                    vec.push(self.get_escrow_new(account_id.clone(), contractor.to_string(), id));
                }
            }
        }

        vec
    }

    /* Using O(n^2) method with O(n) == O(n^2) in efficient */
    pub fn get_escrows_as_contractor_filtered_by_state(&self, states: Vec<EscrowState>) -> Vec<Option<Escrow>> {
        let mut vec = Vec::new();

        for i in self.get_escrows_as_contractor().into_iter() {
            if let Some(escrow) = i {
                if states.iter().any(|v| v == &escrow.escrow_state ) {
                    vec.push(Some(escrow.clone()));
                }
            }
        };

        vec
    }

    pub fn get_escrows_as_client_filtered_by_state(&self, states: Vec<EscrowState>) -> Vec<Option<Escrow>> {
        let mut vec = Vec::new();

        for i in self.get_escrows_as_client().into_iter() {
            if let Some(escrow) = i {
                if states.iter().any(|v| v == &escrow.escrow_state ) {
                    vec.push(Some(escrow.clone()));
                }
            }
        };

        vec
    }

    /*
        get escrows for particular account (as contractor or client) matching statuses
    */
    // pub fn get_escrows_filter(&self, as_contractor: bool, account: AccountId, status_include: Vec<EscrowState>) -> Vec<Option<Escrow>> {
    //     let mut vec = Vec::new();

    //     if as_contractor 
    //     {
    //         // As Contractor match on contractor_escrows accountId
    //         if let Some(n) = self.contractor_escrows.get(&account)
    //         {
    //             if let Some(escrows) = self.escrow_list.get(&account.clone()) {
    //                 for id in 0..(*n+1) {  // eg. 0..4 = [0,1,2,3]
    //                     let cur = escrows.get(&id);
    //                     match cur {
    //                         Some(x) => {
    //                             // one only matching either include criteria
    //                             if status_include.iter().any(|v| v == &x.escrow_state ) {
    //                                 vec.push(cur.cloned());
    //                             }  
    //                         },
    //                         None => {}
    //                     }
    //                 }
    //             }
    //         }
    //     } else {
    //         // As Client match on n_escrows accountId
    //         if let Some(n) = self.n_escrows.get(&account) 
    //         {
    //             if let Some(escrows) = self.escrow_list.get(&account.clone()) {
    //                 for id in 0..(*n+1) {  // eg. 0..4 = [0,1,2,3]
    //                     let cur = escrows.get(&id);
    //                     match cur {
    //                         Some(x) => {
    //                             // one only matching either include criteria
    //                             if status_include.iter().any(|v| v == &x.escrow_state ) {
    //                                 vec.push(cur.cloned());
    //                             }  
    //                         },
    //                         None => {}
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     vec
    // }

    /* Modifier */
    /* Still pending if this may cost more than a simple assert rather than another method call */
    fn check_state(&self, escrow: Escrow, state: EscrowState) {
        assert!(escrow.escrow_state == state, "Wrong State!");
    }
}