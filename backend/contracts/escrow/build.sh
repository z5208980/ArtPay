#!/bin/bash          

MASTERACCOUNT="artpay.testnet"
SUBACCOUNT="escrow.artpay.testnet" 
NFTACCOUNT="nft.artpay.testnet" 

# Reset current subaccount
reset_sub_account() {
    near delete $SUBACCOUNT $MASTERACCOUNT
    near create-account $SUBACCOUNT --masterAccount $MASTERACCOUNT --initialBalance 50
}

deploy_contract() {
    # Deploy
    near deploy --accountId $SUBACCOUNT --wasmFile target/wasm32-unknown-unknown/release/escrow.wasm
    # constructor()
    near call $SUBACCOUNT constructor '{"owner_id": "escrow.artpay.testnet"}' --accountId $SUBACCOUNT
}

main() {
    cargo build --target wasm32-unknown-unknown --release       # Compile
    reset_sub_account   # Make subaccount for new contract
    deploy_contract     # deploy contract to fresh subaccount
}

# CLI testing commands
init_escrows() {
    near call $SUBACCOUNT create_new_escrow '{"contractor": "'$MASTERACCOUNT'", "timestamp": 1234423, "title": "project X", "description": "NFT project", "escrow_type": "1", "requirement": "IPFS documentation", "license_code": "String", "license_desc": "String", "license_url": "String" }' --accountId $MASTERACCOUNT # --deposit 5
    near call $SUBACCOUNT create_new_escrow '{"contractor": "'$MASTERACCOUNT'", "timestamp": 1234234, "title": "project X", "description": "NFT project", "escrow_type": "1", "requirement": "IPFS documentation", "license_code": "String", "license_desc": "String", "license_url": "String" }' --accountId $NFTACCOUNT
    near call $SUBACCOUNT create_new_escrow '{"contractor": "'$MASTERACCOUNT'", "timestamp": 3243543, "title": "project X", "description": "NFT project", "escrow_type": "1", "requirement": "IPFS documentation", "license_code": "String", "license_desc": "String", "license_url": "String" }' --accountId $MASTERACCOUNT
    near call $SUBACCOUNT create_new_escrow '{"contractor": "'$MASTERACCOUNT'", "timestamp": 3243543, "title": "project X", "description": "NFT project", "escrow_type": "1", "requirement": "IPFS documentation", "license_code": "String", "license_desc": "String", "license_url": "String" }' --accountId $MASTERACCOUNT 
    near call $SUBACCOUNT create_new_escrow '{"contractor": "'$MASTERACCOUNT'", "timestamp": 3243543, "title": "project X", "description": "NFT project", "escrow_type": "1", "requirement": "IPFS documentation", "license_code": "String", "license_desc": "String", "license_url": "String" }' --accountId $MASTERACCOUNT
}

test_checkin() {
    # creating checkin #1
    near call $SUBACCOUNT create_escrow_checkin '{"client": "'$MASTERACCOUNT'", "id": 1, "update_type": "draft #1", "message": "message", "media_url": "IPFS", "timestamp": 1231232143}' --accountId $MASTERACCOUNT
    # checkin that exist, should return above information
    near call $SUBACCOUNT get_escrow_checkin '{"client": "'$MASTERACCOUNT'", "contractor": "'$MASTERACCOUNT'", "id": 1, "checkin_id": 0}' --accountId $MASTERACCOUNT 
    # escrow exist but checkin does not
    near call $SUBACCOUNT get_escrow_checkin '{"client": "'$MASTERACCOUNT'", "contractor": "'$MASTERACCOUNT'", "id": 1, "checkin_id": 1000}' --accountId $MASTERACCOUNT
    # escrow does not exist, hence checkin MUST not exist
    near call $SUBACCOUNT get_escrow_checkin '{"client": "'$MASTERACCOUNT'", "contractor": "'$MASTERACCOUNT'", "id": 100, "checkin_id": 1}' --accountId $MASTERACCOUNT
    # creating checkin #2
    near call $SUBACCOUNT create_escrow_checkin '{"client": "'$MASTERACCOUNT'", "id": 1, "update_type": "draft #2", "message": "message", "media_url": "IPFS", "timestamp": 1231232143}' --accountId $SUBACCOUNT   
    # Should return 2 checkins for given escrow
    near call $SUBACCOUNT get_escrow_checkins_list '{"client": "'$MASTERACCOUNT'", "contractor": "'$MASTERACCOUNT'", "id": 1}' --accountId $MASTERACCOUNT 
}

TOKENNAME="token-testsadcnae"
test_approval() {
    # # Both approval should be false
    # near call $SUBACCOUNT get_escrow_new '{"client": "'$MASTERACCOUNT'", "contractor": "'$MASTERACCOUNT'", "id": 1}' --accountId $MASTERACCOUNT
    
    # # Can't approval In Wrong State
    # near call $SUBACCOUNT client_approval '{"contractor": "'$MASTERACCOUNT'", "id": 1}' --accountId $MASTERACCOUNT
    # near call $SUBACCOUNT contractor_approval '{"client": "'$MASTERACCOUNT'", "id": 1}' --accountId $MASTERACCOUNT

    # mint token and set as deliverable
    near call $NFTACCOUNT nft_mint '{"token_id": "'$TOKENNAME'", "metadata": {"title": "NFT Tutorial Token", "description": "Testing the transfer call functiewrewron", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif", "copyright": "copright information", "right_assign": "FULL" }, "receiver_id": "'$MASTERACCOUNT'"}' --accountId $MASTERACCOUNT --amount 0.1
    near call $SUBACCOUNT set_nft_deliverable '{"client": "'$MASTERACCOUNT'", "id": 1, "nft_address": "nft.artpay.testnet", "token_id": "'$TOKENNAME'"}' --accountId $MASTERACCOUNT --depositYocto 1 --gas 200000000000000
  
    # # Can approve now     
    # near call $SUBACCOUNT client_approval '{"contractor": "'$MASTERACCOUNT'", "id": 1}' --accountId $MASTERACCOUNT
    # near call $SUBACCOUNT contractor_approval '{"client": "'$MASTERACCOUNT'", "id": 1}' --accountId $MASTERACCOUNT

    # # Both approval should be true
    # near call $SUBACCOUNT get_escrow_new '{"client": "'$MASTERACCOUNT'", "contractor": "'$MASTERACCOUNT'", "id": 1}' --accountId $MASTERACCOUNT
}

test_getter() {
    # Get existing escrow, should return something
    near call $SUBACCOUNT get_escrow_new '{"client": "'$MASTERACCOUNT'", "contractor": "'$MASTERACCOUNT'", "id": 1}' --accountId $MASTERACCOUNT

    # Get non existing escrow, return null
    near call $SUBACCOUNT get_escrow_new '{"client": "'$MASTERACCOUNT'", "contractor": "'$MASTERACCOUNT'", "id": 1000}' --accountId $MASTERACCOUNT

    # Get all escrow of address as client. Note NFTACCOUNT should have 1 escrows
    near call $SUBACCOUNT get_escrows_as_client --accountId $NFTACCOUNT

    # Get all escrows of address as contractor, MASTERACCOUNT should have 4 escrows
    near call $SUBACCOUNT get_escrows_as_contractor --accountId $MASTERACCOUNT

    # Get all escrows by with filtered escrow states, should return 4 and 1 escrows respectively
    near call $SUBACCOUNT get_escrows_as_client_filtered_by_state '{"states": ["AWAITING", "APPROVAL"]}' --accountId $MASTERACCOUNT
    near call $SUBACCOUNT get_escrows_as_contractor_filtered_by_state '{"states": ["APPROVAL"]}' --accountId $MASTERACCOUNT
}

test_cancel() {
    # near call $SUBACCOUNT get_escrow_new '{"client": "'$MASTERACCOUNT'", "contractor": "'$MASTERACCOUNT'", "id": 1}' --accountId $MASTERACCOUNT

    # Canceling escrow
    near call $SUBACCOUNT cancel '{"client": "'$MASTERACCOUNT'", "contractor": "'$MASTERACCOUNT'", "id": 1}' --accountId $MASTERACCOUNT --depositYocto 1 --gas 200000000000000

    # Viewing escrow. Escrow must have CANCEL state
    near call $SUBACCOUNT get_escrow_new '{"client": "'$MASTERACCOUNT'", "contractor": "'$MASTERACCOUNT'", "id": 1}' --accountId $MASTERACCOUNT
}


TOKENNAME="token-testreleases1"
test_release() {
    # Create escrow with 5 NEAR locked for contractor
    near state $MASTERACCOUNT 
    near call $SUBACCOUNT create_new_escrow '{"contractor": "'$SUBACCOUNT'", "timestamp": 3243543, "title": "project X", "description": "NFT project", "escrow_type": "1", "requirement": "IPFS documentation", "license_code": "String", "license_desc": "String", "license_url": "String" }' --accountId $MASTERACCOUNT --amount 5
    near state $MASTERACCOUNT 

    # contractor to mint and set the delieverable
    near call $NFTACCOUNT nft_mint '{"token_id": "'$TOKENNAME'", "metadata": {"title": "NFT Tutorial Token", "description": "Testing the transfer call functiewrewron", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif", "copyright": "copright information", "right_assign": "FULL" }, "receiver_id": "'$MASTERACCOUNT'"}' --accountId $MASTERACCOUNT --amount 0.1
    near call $SUBACCOUNT set_nft_deliverable '{"client": "'$MASTERACCOUNT'", "id": 0, "nft_address": "nft.artpay.testnet", "token_id": "'$TOKENNAME'"}' --accountId $SUBACCOUNT --depositYocto 1 --gas 200000000000000

    # near call $SUBACCOUNT get_escrow_new '{"client": "'$MASTERACCOUNT'", "contractor": "'$SUBACCOUNT'", "id": 0}' --accountId $SUBACCOUNT
    # near view $NFTACCOUNT nft_token '{"token_id": "'$TOKENNAME'"}'

    # Trying to release escrow will result in failure
    near call $SUBACCOUNT release_escrow '{"client": "'$MASTERACCOUNT'", "contractor": "'$SUBACCOUNT'", "id": 0}' --accountId $MASTERACCOUNT --depositYocto 1 --gas 200000000000000

    near call $SUBACCOUNT client_approval '{"contractor": "'$SUBACCOUNT'", "id": 0}' --accountId $MASTERACCOUNT
    near call $SUBACCOUNT contractor_approval '{"client": "'$MASTERACCOUNT'", "id": 0}' --accountId $SUBACCOUNT
    # Success only after two parties approve
    near state $SUBACCOUNT 
    near call $SUBACCOUNT release_escrow '{"client": "'$MASTERACCOUNT'", "contractor": "'$SUBACCOUNT'", "id": 0}' --accountId $MASTERACCOUNT --depositYocto 1 --gas 200000000000000

    # since client, NFT should belong to them
    near view $NFTACCOUNT nft_token '{"token_id": "'$TOKENNAME'"}'
    # 5 NEAR should be added to contractor
    near state $SUBACCOUNT 

    # Escrow should be COMPLETE
    near call $SUBACCOUNT get_escrow_new '{"client": "'$MASTERACCOUNT'", "contractor": "'$SUBACCOUNT'", "id": 0}' --accountId $SUBACCOUNT
}

# main
# init_escrows
# test_checkin
# test_approval
# test_getter
# test_cancel
# test_release
