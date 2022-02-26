import Vuex from 'vuex';

export default new Vuex.Store({
	state: {
        MAX_GAS: "300000000000000",
        MINT_PRICE: "10000000000000000000000",
        msg: "ArtPay",
        near: null,
        accountId: null,
        wallet: null,
		count: 0,
		abi: {
			nft: {
				contractAddr: "nft.artpay.testnet",
				viewMethods: ["nft_metadata", "nft_token"],
				changeMethods: ["nft_mint", "allowance", "nft_transfer"],
			},
			escrow: {
				contractAddr: "escrow.artpay.testnet",
				viewMethods: ["get_escrow", "get_escrow_new", "get_escrows_as_contractor", "get_escrows_as_client"],
				changeMethods: ["release_escrow", "contractor_approval", "client_approval", 
				"create_new_escrow", "take_my_money", "set_deliverable", "set_nft_deliverable",
				],
			}
		},
	},
	mutations: {
        setNear(state, payload) { !state.near && (state.near = payload) },
        setNearWallet(state, payload) { !state.wallet && (state.wallet = payload) },
        setAccountId(state, payload) { !state.accountId && (state.accountId = payload) },
		subtract(state, payload) {
			payload ? (state.count -= payload) : state.count--;
		}
	},
	actions: {
		addThreeAsync({ commit }) {
			setTimeout(() => commit('add', 3), 3000);
		}
	}
});