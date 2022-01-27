<template>
  <div class="my-5">
    <h1 class="font-bold text-center">{{ msg }}</h1>
    <div>
      {{ nftOutput }}
    </div>
    <div class="container mx-auto p-16">
      <div class="grid grid-rows-3 grid-flow-col gap-4">
        <div class="row-span-3 border rounded-md px-7">
          <h1 class="font-bold">ArtPay NFT</h1>
          <template v-if="!accountId">
            Login to continue!
          </template>
          <template v-else>
            <div @click="getMetaData()" :class="btnStyle">
              getMetaData
            </div>
            <div @click="mintNFT()" :class="btnStyle">
              mintNFT
            </div>
            <input v-model="tokenId" class="placeholder:italic placeholder:text-slate-400 block bg-white w-full border border-slate-300 rounded-md py-2 pl-9 pr-3 shadow-sm focus:outline-none focus:border-sky-500 focus:ring-sky-500 focus:ring-1 sm:text-sm" placeholder="Search for anything..." type="text" name="search"/>
            <div @click="viewNFTToken()" :class="btnStyle">
              viewNFTToken
            </div>
          </template>
        </div>
        <div class="col-span-2 border rounded-md px-7">
          <h1 class="font-bold">NEAR Wallet</h1>
          <template v-if="!accountId">
            <div @click="login()" :class="btnStyle">
              Login with NEAR Wallet
            </div>
          </template>
          <template v-else>
            <div @click="logout()" :class="btnStyle">
              {{ accountId }} Logout
            </div>

            <div @click="getWallet()" :class="btnStyle">
              Get Account Details
            </div>
            <p class="text-sm">See Console</p>

            <div @click="sendOneNear()" :class="btnStyle">
              Donate One Near to ArtPay
            </div>
          </template>
        </div>
        <div class="row-span-2 col-span-2 border rounded-md px-7">
          <h1 class="font-bold">ArtPay Escrow</h1>
          <input v-model="tokenId" class="placeholder:italic placeholder:text-slate-400 block bg-white w-full border border-slate-300 rounded-md py-2 pl-9 pr-3 shadow-sm focus:outline-none focus:border-sky-500 focus:ring-sky-500 focus:ring-1 sm:text-sm" placeholder="Search for anything..." type="text" name="search"/>
          <div @click="setNFTDeliverable()" :class="btnStyle">
            setNFTDeliverable
          </div>
          <p class="text-sm">This will transfer owner_id of NFT to escrow.artpay.testnet</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
const nearAPI = require("near-api-js");
const { connect, KeyPair, keyStores, WalletConnection, utils } = nearAPI;

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
    changeMethods: ["nft_mint", "allowance", "nft_transfer"],
  },
  escrow: {
    contractAddr: "escrow.artpay.testnet",
    viewMethods: ["get_escrow"],
    changeMethods: ["release_escrow", "contractor_approval", "client_approval", "create_new_escrow", "take_my_money"],
  }
}

export default {
  name: "HelloWorld",
  data() {
    return {
      msg: "ArtPay",
      near: null,
      accountId: null,
      wallet: null,

      // nft
      tokenId: 0,
      nftOutput: null
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
    async getMetaData() {
     
      const contract = this.loadNFTContract("nft");
      this.nftOutput = await contract.nft_metadata();
    },
    async getNumberOfToken() {
      const contract = this.loadNFTContract("nft");
      console.log(await contract.get_n_tokens());
    },
    async viewNFTToken() {
      console.log("HERE");
      const contract = this.loadNFTContract('nft');
      this.nftOutput = await contract.nft_token( { token_id: this.tokenId.toString() } );
    },
    async mintNFT() {
      const contract = this.loadNFTContract('nft');
      const y = utils.format.formatNearAmount("5460000000000000000000");
      console.log(y);
      const response = await contract.nft_mint( 
        {
          receiver_id: this.wallet.getAccountId(),
          token_id: `token-${Math.floor(Math.random() * 1000000000)}`,
          metadata: { 
            title: "ArtPay", 
            description: "Example ArtPay NFT!", 
            media: "URL", 
            copies: 1
          }
        },
        null,
        utils.format.parseNearAmount(y),
      );
      this.nftOutput = `Token minted as: ${response}`;
    },
    async setNFTDeliverable() {
      // # near call $NFTACCOUNT nft_transfer '{"receiver_id": "rafaam.testnet", "token_id": "token-1", "approval_id": 2, "msg": "foo"}' --accountId $NFTACCOUNT --depositYocto 1 --gas 200000000000000
      console.log("Sending One NEAR ...")
      // const contractEscrow = this.loadNFTContract('escrow');
      const contractNFT = this.loadNFTContract('nft');
      this.nftOutput = await contractNFT.nft_transfer(
        { receiver_id: "escrow.artpay.testnet", token_id: this.tokenId.toString(), approval_id: 2, msg: "For ArtPay Escrow" },
        200000000000000,
        1
      );
    },
    async sendOneNear() {
      console.log("Sending One NEAR ...")
      const contract = this.loadNFTContract('escrow');
      const value = utils.format.parseNearAmount("1");
      const gas = utils.format.parseNearAmount("0.0000000003");
      const response = await contract.take_my_money(
        {},
        gas, // attached GAS (optional)
        value // unit: yoctoNEAR 
      );
      console.log(response);
    },
    loadNFTContract(contract) {
      return new nearAPI.Contract(
        this.wallet.account(),
        abi[contract].contractAddr,
        {
          viewMethods: abi[contract].viewMethods,
          changeMethods: abi[contract].changeMethods,
          sender: this.wallet.getAccountId(), 
        }
      )
    }
  },
  mounted() { this.init(); },
  computed: {
    btnStyle: function () {
      return "py-3 px-7 my-2 text-white transition-color rounded-md bg-indigo-600 hover:bg-indigo-700 duration-150"
    },
  }
}
</script>


