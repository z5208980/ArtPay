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
    # reset_sub_account   # Make subaccount for new contract
    # deploy_contract     # deploy contract to fresh subaccount
}

main

# CLI testing commands
# near call $SUBACCOUNT create_new_escrow '{"contractor": "escrow.artpay.testnet", "nft_address": "escrow.artpay.testnet", "token_id": "1", "timestamp": 1234423}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT create_new_escrow '{"contractor": "nft.artpay.testnet", "nft_address": "escrow.artpay.testnet", "token_id": "1", "timestamp": 1234234}' --accountId $SUBACCOUNT --deposit 5
# near call $SUBACCOUNT create_new_escrow '{"contractor": "escrow.artpay.testnet", "nft_address": "escrow.artpay.testnet", "token_id": "1", "timestamp": 3243543}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT get_escrow '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT client_approval '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT release_escrow '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT contractor_approval '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT get_escrow '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT release_escrow '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT get_escrow '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT

# near view $NFTACCOUNT nft_metadata
# near call $NFTACCOUNT nft_mint '{"receiver_id": "'$MASTERACCOUNT'", "token_metadata": {"title": "My Non Fungible Team Token", "description": "The Team Most Certainly Goes :)", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"} }' --accountId $MASTERACCOUNT --amount 0.1
# near view $NFTACCOUNT nft_token '{"token_id": "3"}'
