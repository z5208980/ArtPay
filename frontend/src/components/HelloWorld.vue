<template>
  <div class="">
    <h1>{{ msg }}</h1>
    <template v-if="!accountId">
      <div @click="login()" class="py-3 px-7 m-2 text-white transition-color rounded-md bg-indigo-600 hover:bg-indigo-700 duration-150">
        Login with NEAR Wallet
      </div>
    </template>
    <template v-else>
      <div @click="logout()" class="py-3 px-7 m-2 text-white transition-color rounded-md bg-indigo-600 hover:bg-indigo-700 duration-150">
        {{ accountId }} Logout
      </div>

      <div @click="getWallet()" class="py-3 px-7 m-2 text-white transition-color rounded-md bg-indigo-600 hover:bg-indigo-700 duration-150">
        getAccountDetails
      </div>
      <!-- <div @click="createFullAccessKey()" class="py-3 px-7 m-2 text-white transition-color rounded-md bg-indigo-600 hover:bg-indigo-700 duration-150">
        Generate Full Access Key
      </div> -->
    </template>

    <h1>NFT ArtPay</h1>
    <!-- <div @click="loadContract()" class="py-3 px-7 m-2 text-white transition-color rounded-md bg-indigo-600 hover:bg-indigo-700 duration-150">
      loadContract
    </div> -->

    <div @click="getMetaData()" class="py-3 px-7 m-2 text-white transition-color rounded-md bg-indigo-600 hover:bg-indigo-700 duration-150">
      getMetaData
    </div>

    <div @click="mintNFT()" class="py-3 px-7 m-2 text-white transition-color rounded-md bg-indigo-600 hover:bg-indigo-700 duration-150">
      mintNFT
    </div>

    <input type="text" v-model="tokenId" />
    <div @click="viewNFTToken()" class="py-3 px-7 m-2 text-white transition-color rounded-md bg-indigo-600 hover:bg-indigo-700 duration-150">
      viewNFTToken
    </div>

    <div @click="getNumberOfToken()" class="py-3 px-7 m-2 text-white transition-color rounded-md bg-indigo-600 hover:bg-indigo-700 duration-150">
      getNumberOfToken
    </div>


  </div>
</template>

<script>
const nearAPI = require("near-api-js");
const { connect, KeyPair, keyStores, WalletConnection } = nearAPI;

// const homedir = require("os").homedir();
// const CREDENTIALS_DIR = ".near-credentials";
// const credentialsPath = require("path").join(homedir, CREDENTIALS_DIR);
// const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);
const keyStore = new keyStores.BrowserLocalStorageKeyStore(window.localStorage, 'nearlib:keystore:');

console.log(keyStore);
const config = {
  networkId: "testnet",
  keyStore: keyStore,
  nodeUrl: "https://rpc.testnet.near.org",
  walletUrl: "https://wallet.testnet.near.org",
  helperUrl: "https://helper.testnet.near.org",
  explorerUrl: "https://explorer.testnet.near.org",
};

// requires viewMethods [String] , changeMethods [String]
const abi = {
  greeter: {
    contractAddr: "greeter.rafaam.testnet",
    viewMethods: ["get_greeting"],
    changeMethods: ["set_greeting"],
  },
  nft: {
    contractAddr: "nft.artpay.testnet",
    viewMethods: ["nft_metadata", "nft_token", "get_n_tokens"],
    changeMethods: ["nft_mint"],
  }
}

export default {
  name: "HelloWorld",
  data() {
    return {
      msg: "Wallet",
      near: null,
      accountId: null,
      wallet: null,

      // nft
      tokenId: 0,
    }
  },
  methods: {
    async init() {
      this.near = await connect(config);
      this.wallet = new WalletConnection(this.near);

      // if(!this.wallet.isSignedIn()) {
      //   this.wallet.requestSignIn();
      // }

      this.accountId = (this.wallet.isSignedIn() && this.wallet.getAccountId());
    },
    async login() {    
      await this.wallet.requestSignIn(abi.nft.contractAddr, "ArtPay");
    },
    async logout() {
      this.wallet.signOut();
      this.accountId = null;
    },
    async getWallet() {
      const account = await this.near.account(this.accountId);
      console.log(this.accountId);
      console.log(await account.getAccountDetails());
    },
    async deployNFT() {
      const account = await this.near.account(this.accountId);  
      const response = await account.deployContract('../assets/non_fungible_token.wasm');  
      console.log(response);
      console.log(await account.getAccessKeys());
      // console.log(await account.getAccountDetails());
    },
    async createFullAccessKey() {
      const keyPair = KeyPair.fromRandom("ed25519");
      const publicKey = keyPair.publicKey.toString();
      const near = await connect(config);
      const account = await near.account(this.accountId);
      await keyStore.setKey(config.networkId, publicKey, keyPair);
      await account.addKey(publicKey);
    },
    // async loadContract() {
    //   const contract = new nearAPI.Contract(
    //     this.wallet.account(),
    //     abi.greeter.contractAddr,
    //     {
    //       viewMethods: abi.greeter.viewMethods,
    //       changeMethods: abi.greeter.changeMethods,
    //       sender: this.wallet.getAccountId(), 
    //     }
    //   );
    //   console.log(contract);
    //   console.log(await contract.get_greeting( { account_id: "rafaam.testnet", } ));
    // },
    async getMetaData() {
      // near-cli equvilant
      // near call $ID nft_mint '{"token_id": "0", "receiver_id": "'$ID'", "token_metadata": { "title": "Some Art", "description": "My NFT media", "media": "https://bafkreiabag3ztnhe5pg7js4bj6sxuvkz3sdf76cjvcuqjoidvnfjz7vwrq.ipfs.dweb.link/", "copies": 1}}' --accountId $ID --deposit 0.1
      const contract = this.loadNFTContract();
      console.log(await contract.nft_metadata());
    },
    async getNumberOfToken() {
      // near-cli equvilant
      // near call $ID nft_mint '{"token_id": "0", "receiver_id": "'$ID'", "token_metadata": { "title": "Some Art", "description": "My NFT media", "media": "https://bafkreiabag3ztnhe5pg7js4bj6sxuvkz3sdf76cjvcuqjoidvnfjz7vwrq.ipfs.dweb.link/", "copies": 1}}' --accountId $ID --deposit 0.1
      const contract = this.loadNFTContract();
      console.log(await contract.get_n_tokens());
    },
    async viewNFTToken() {
      const contract = this.loadNFTContract();
      console.log(await contract.nft_token( { token_id: this.tokenId.toString() } ));
    },
    async mintNFT() {
      console.log("Minting NFT ...");
      const contract = this.loadNFTContract();
      const response = await contract.nft_mint( 
        {
          receiver_id: this.wallet.getAccountId(),
          token_metadata: { 
            title: "Olympus Mons", 
            description: "Tallest mountain in charted solar system", 
            media: "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Olympus_Mons_alt.jpg/1024px-Olympus_Mons_alt.jpg", 
            copies: 1
          }
        }
      );
      console.log(response); 
    },
    loadNFTContract() {
      return new nearAPI.Contract(
        this.wallet.account(),
        abi.nft.contractAddr,
        {
          viewMethods: abi.nft.viewMethods,
          changeMethods: abi.nft.changeMethods,
          sender: this.wallet.getAccountId(), 
        }
      )
    }
  },
  mounted() { this.init(); }
}
</script>


