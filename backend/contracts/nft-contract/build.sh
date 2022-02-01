#!/bin/bash          

MASTERACCOUNT="artpay.testnet"
SUBACCOUNT="escrow.artpay.testnet" 
NFTACCOUNT="nft.artpay.testnet" 

# Reset current subaccount
reset_sub_account() {
    near delete $NFTACCOUNT $MASTERACCOUNT
    near create-account $NFTACCOUNT --masterAccount $MASTERACCOUNT --initialBalance 50
}

deploy_contract() {
    # Deploy
    near deploy --accountId $NFTACCOUNT --wasmFile target/wasm32-unknown-unknown/release/non_fungible_token.wasm
    # constructor()
    near call $NFTACCOUNT new_default_meta '{"owner_id": "'$NFTACCOUNT'"}' --accountId $NFTACCOUNT
}

main() {
    cargo build --target wasm32-unknown-unknown --release       # Compile
    reset_sub_account   # Make subaccount for new contract
    deploy_contract     # deploy contract to fresh subaccount
}

main

# TESTING
near call $NFTACCOUNT nft_mint '{"token_id": "token-1", "metadata": {"title": "NFT Tutorial Token", "description": "Testing the transfer call function", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}, "receiver_id": "'$NFTACCOUNT'"}' --accountId $NFTACCOUNT --amount 0.1
near call $NFTACCOUNT nft_mint '{"token_id": "token-2", "metadata": {"title": "NFT Tutorial Token", "description": "Testing the transfer call function", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif", "copyright": "copright information", "right_assign": 1 }, "receiver_id": "'$MASTERACCOUNT'"}' --accountId $MASTERACCOUNT --amount 0.1

# near view $NFTACCOUNT nft_token '{"token_id": "token-1"}'
# near call $NFTACCOUNT nft_transfer '{"receiver_id": "rafaam.testnet", "token_id": "token-1", "approval_id": 2, "msg": "foo"}' --accountId $NFTACCOUNT --depositYocto 1 --gas 200000000000000