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

main

# CLI testing commands
# near call $SUBACCOUNT create_new_escrow '{"contractor": "'$MASTERACCOUNT'", "timestamp": 1234423, "title": "project X", "description": "NFT project", "escrow_type": "1", "requirement": "IPFS documentation" }' --accountId $MASTERACCOUNT --deposit 5
# near call $SUBACCOUNT create_new_escrow '{"contractor": "'$MASTERACCOUNT'", "timestamp": 1234234, "title": "project X", "description": "NFT project", "escrow_type": "1", "requirement": "IPFS documentation" }' --accountId $MASTERACCOUNT
# near call $SUBACCOUNT create_new_escrow '{"contractor": "'$MASTERACCOUNT'", "timestamp": 3243543, "title": "project X", "description": "NFT project", "escrow_type": "1", "requirement": "IPFS documentation" }' --accountId $MASTERACCOUNT
# near call $SUBACCOUNT get_escrow '{"client": "'$MASTERACCOUNT'", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT client_approval '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT release_escrow '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT contractor_approval '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT get_escrow '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT release_escrow '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT get_escrow '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT get_all_escrow '{"client": "'$MASTERACCOUNT'" }' --accountId $MASTERACCOUNT
# near call $SUBACCOUNT get_escrows_filter '{"as_contractor": 1, "account": "$MASTERACCOUNT", "status_include":"AWAITING,APPROVAL"}' --accountId $MASTERACCOUNT

# near view $NFTACCOUNT nft_metadata
# TOKENNAME="token-nft2"
# near call $NFTACCOUNT nft_mint '{"token_id": "'$TOKENNAME'", "metadata": {"title": "NFT Tutorial Token", "description": "Testing the transfer call function", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif", "copyright": "copright information", "right_assign": "FULL" }, "receiver_id": "'$MASTERACCOUNT'"}' --accountId $MASTERACCOUNT --amount 0.1
# near call $SUBACCOUNT set_nft_deliverable '{"client": "'$MASTERACCOUNT'", "id": 0, "nft_address": "nft.artpay.testnet", "token_id": "'$TOKENNAME'"}' --accountId $MASTERACCOUNT --depositYocto 1 --gas 200000000000000
# near view $NFTACCOUNT nft_token '{"token_id": "'$TOKENNAME'"}'
# near call $SUBACCOUNT get_escrow '{"client": "'$MASTERACCOUNT'", "id": 0}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT cancel '{"client": "'$MASTERACCOUNT'", "id": 0}' --accountId $MASTERACCOUNT --gas 200000000000000 # --depositYocto 1
# near view $NFTACCOUNT nft_token '{"token_id": "'$TOKENNAME'"}'

# near call $SUBACCOUNT get_all_escrow '{"client": "'$MASTERACCOUNT'"}' --accountId $MASTERACCOUNT