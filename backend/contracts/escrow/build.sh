#!/bin/bash          

MASTERACCOUNT="artpay.testnet"
SUBACCOUNT="escrow.artpay.testnet" 

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

# main

# CLI testing commands
# near call $SUBACCOUNT create_new_escrow '{"contractor": "escrow.artpay.testnet", "nft_address": "escrow.artpay.testnet", "token_id": "1"}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT create_new_escrow '{"contractor": "escrow.artpay.testnet", "nft_address": "escrow.artpay.testnet", "token_id": "1"}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT create_new_escrow '{"contractor": "escrow.artpay.testnet", "nft_address": "escrow.artpay.testnet", "token_id": "1"}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT get_escrow '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT client_approval '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT release_escrow '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT contractor_approval '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT
# near call $SUBACCOUNT release_escrow '{"client": "escrow.artpay.testnet", "id": 1}' --accountId $SUBACCOUNT