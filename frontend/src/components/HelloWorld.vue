<template>
  <div class="my-5">
    <h1 class="font-bold text-center">{{ msg }}</h1>
    <div class="container mx-auto px-16">
      Message: {{ globalMessage }}
    </div>
    <div class="container mx-auto p-16">
      <div class="grid grid-rows-3 grid-flow-col gap-4">
        <div class="row-span-3 border rounded-md px-7">
          <h1 class="font-bold text-center">ArtPay NFT</h1>
          <template v-if="!accountId">
            Login to continue!
          </template>
          <template v-else>
            <div @click="getMetaData()" :class="btnStyle">
              getMetaData
            </div>

            <h2 class="font-bold">Mint new NFT</h2>
            <div class="py-2">Token ID</div>
            <input v-model="newNFT.tokenId" class="placeholder:italic placeholder:text-slate-400 block bg-white w-full border border-slate-300 rounded-md py-2 pl-9 pr-3 shadow-sm focus:outline-none focus:border-sky-500 focus:ring-sky-500 focus:ring-1 sm:text-sm" type="text" />
            <div class="py-2">Copyright</div>
            <input v-model="newNFT.copyright" class="placeholder:italic placeholder:text-slate-400 block bg-white w-full border border-slate-300 rounded-md py-2 pl-9 pr-3 shadow-sm focus:outline-none focus:border-sky-500 focus:ring-sky-500 focus:ring-1 sm:text-sm" placeholder="Copyright Policy" type="textarea" />
            
            <div @click="mintNFT()" :class="btnStyle">
              mintNFT
            </div>
            <p class="text-sm"><b>Note</b> Minting will incur {{ MINT_PRICE }} yoctoNEAR. Reason: For storage</p>

            <h2 class="font-bold">View NFT</h2>
            <input v-model="viewNFT.tokenId" class="placeholder:italic placeholder:text-slate-400 block bg-white w-full border border-slate-300 rounded-md py-2 pl-9 pr-3 shadow-sm focus:outline-none focus:border-sky-500 focus:ring-sky-500 focus:ring-1 sm:text-sm" type="text" />
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
            <!-- <div @click="getWallet()" :class="btnStyle">
              Get Account Details
            </div> 
            <p class="text-sm">See Console</p>
            -->
            <div @click="sendOneNear()" :class="btnStyle">
              Donate One Near to ArtPay
            </div>
          </template>
        </div>
        <div class="row-span-2 col-span-2 border rounded-md px-7">
          <h1 class="font-bold text-center">ArtPay Escrow</h1>

          <h2 class="font-bold">Create Escrow</h2>
          <div class="py-2">Contractor Address</div>
          <input v-model="newEscrowFormData.contractor" class="placeholder:italic placeholder:text-slate-400 block bg-white w-full border border-slate-300 rounded-md py-2 pl-9 pr-3 shadow-sm focus:outline-none focus:border-sky-500 focus:ring-sky-500 focus:ring-1 sm:text-sm" placeholder="contractor address" type="text"/>
          <div class="py-2">NFT Address</div>
          <input v-model="newEscrowFormData.nftAddress" class="placeholder:italic placeholder:text-slate-400 block bg-white w-full border border-slate-300 rounded-md py-2 pl-9 pr-3 shadow-sm focus:outline-none focus:border-sky-500 focus:ring-sky-500 focus:ring-1 sm:text-sm" placeholder="NFT Address" type="text" />
          <div class="py-2">Token TokenId</div>
          <input v-model="newEscrowFormData.tokenId" class="placeholder:italic placeholder:text-slate-400 block bg-white w-full border border-slate-300 rounded-md py-2 pl-9 pr-3 shadow-sm focus:outline-none focus:border-sky-500 focus:ring-sky-500 focus:ring-1 sm:text-sm" placeholder="TokenId for NFT" type="text" />
          <div class="py-2">Expiry Date</div>
          <input v-model="newEscrowFormData.timestamp" class="placeholder:italic placeholder:text-slate-400 block bg-white w-full border border-slate-300 rounded-md py-2 pl-9 pr-3 shadow-sm focus:outline-none focus:border-sky-500 focus:ring-sky-500 focus:ring-1 sm:text-sm" placeholder="Expiry Date" type="date" />
          <div class="py-2">Locked Funds (Near)</div>
          <input v-model="newEscrowFormData.lockedFunds" class="placeholder:italic placeholder:text-slate-400 block bg-white w-full border border-slate-300 rounded-md py-2 pl-9 pr-3 shadow-sm focus:outline-none focus:border-sky-500 focus:ring-sky-500 focus:ring-1 sm:text-sm" placeholder="Funds in NEAR" type="number" step="0.01" />
          <div @click="createEscrow()" :class="btnStyle">
            createEscrow
          </div>
          
          <h2 class="font-bold">setNFTDeliverable</h2>
          <input v-model="escrowDeliverable.tokenId" class="placeholder:italic placeholder:text-slate-400 block bg-white w-full border border-slate-300 rounded-md py-2 pl-9 pr-3 shadow-sm focus:outline-none focus:border-sky-500 focus:ring-sky-500 focus:ring-1 sm:text-sm" placeholder="Search for anything..." type="text" />
          <div @click="setNFTDeliverable()" :class="btnStyle">
            setNFTDeliverable
          </div>
          <p class="text-sm">This will transfer owner_id of NFT to escrow.artpay.testnet</p>


          <h2 class="font-bold">Create Escrow</h2>
          <div class="py-2">Client Address</div>
          <input v-model="viewEscrow.client" class="placeholder:italic placeholder:text-slate-400 block bg-white w-full border border-slate-300 rounded-md py-2 pl-9 pr-3 shadow-sm focus:outline-none focus:border-sky-500 focus:ring-sky-500 focus:ring-1 sm:text-sm" placeholder="Client escrow" type="text" />
          <div class="py-2">Escrow ID</div>
          <input v-model="viewEscrow.tokenId" class="placeholder:italic placeholder:text-slate-400 block bg-white w-full border border-slate-300 rounded-md py-2 pl-9 pr-3 shadow-sm focus:outline-none focus:border-sky-500 focus:ring-sky-500 focus:ring-1 sm:text-sm" placeholder="Escrow Id" type="text" />
          <div @click="viewEscrowToken()" :class="btnStyle">
            viewEscrow
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
const nearAPI = require("near-api-js");
const { connect, keyStores, WalletConnection, utils } = nearAPI;

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
  nft: {
    contractAddr: "nft.artpay.testnet",
    viewMethods: ["nft_metadata", "nft_token"],
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
      MAX_GAS: "300000000000000",
      MINT_PRICE: "10000000000000000000000",
      msg: "ArtPay",
      near: null,
      accountId: null,
      wallet: null,
      globalMessage: null,
      urlParams: null,

      // nft
      newNFT: {
        tokenId: "token-1",
        copyright: "Copyright © 2022 ArtPay all rights reserved. See URL for more information.",
      },
      viewNFT: {
        tokenId: "token-1",

      },

      // escrow
      newEscrowFormData: {
        contractor: "example.tesnet",
        nftAddress: "escrow.artpay.testnet",
        tokenId: "token-1",
        timestamp: new Date().toISOString().slice(0, 10),
        lockedFunds: 0,
      },
      escrowDeliverable: {
        tokenId: "token-1",
      },
      viewEscrow: {
        client: "escrow.artpay.testnet",
        tokenId: "1",
      },
    }
  },
  methods: {
    async init() {
      this.urlParams = new URLSearchParams(window.location.search);
      this.near = await connect(config);
      this.wallet = new WalletConnection(this.near);

      // if(!this.wallet.isSignedIn()) {
      //   this.wallet.requestSignIn();
      // }

      this.globalMessage = `Transaction: ${this.urlParams.has("transactionHashes") ? this.urlParams.get("transactionHashes") : "Transaction Failed"}`;
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
    async getMetaData() {
      this.globalMessage = `Getting ${abi.escrow.contractAddr} Metadata`;
      const contract = this.loadNFTContract("nft");
      this.globalMessage = await contract.nft_metadata();
    },
    async viewNFTToken() {
      this.globalMessage = `Getting ${this.viewNFT.tokenId} NFT`;
      const contract = this.loadNFTContract('nft');
      const response = await contract.nft_token( { token_id: this.viewNFT.tokenId.toString() } )
      this.globalMessage = response ? response : `${this.viewNFT.tokenId} doesn't exist for NFT`;
    },
    async mintNFT() {
      const contract = this.loadNFTContract('nft');
      const response = await contract.nft_mint( 
        {
          receiver_id: this.wallet.getAccountId(),
          token_id: this.newNFT.tokenId,
          metadata: { 
            title: "ArtPay", 
            description: "Example ArtPay NFT!", 
            media: "IPFS URL / URL", 
            copies: 1,
            copyright: this.newNFT.copyright// IMPORTANT FOR ARTPAY NFT
          }
        },
        this.MAX_GAS, this.MINT_PRICE,
      );
      this.globalMessage = `Token minted as: ${response}`;
    },
    async createEscrow() {
      let expiryDate = new Date(this.newEscrowFormData.timestamp);
      const contractNFT = this.loadNFTContract('escrow');
      await contractNFT.create_new_escrow(
        {
          contractor: this.newEscrowFormData.contractor,
          nft_address: this.newEscrowFormData.nftAddress,
          token_id: this.newEscrowFormData.tokenId,
          timestamp: expiryDate.getTime(),
        },
      );
    },
    async setNFTDeliverable() {
      const contractNFT = this.loadNFTContract('nft');
      this.globalMessage = await contractNFT.nft_transfer(
        { receiver_id: "escrow.artpay.testnet", token_id: this.escrowDeliverable.tokenId.toString(), approval_id: 2, msg: "For ArtPay Escrow" },
        200000000000000, 1
      );
    },
    async viewEscrowToken() {
      this.globalMessage = `Getting Escrow`;
      const contractEscrow = this.loadNFTContract('escrow');
      this.globalMessage = await contractEscrow.get_escrow(
        { client: this.viewEscrow.client, id: Number(this.viewEscrow.tokenId) }
      );
    },
    async sendOneNear() {
      this.globalMessage = "Sending One NEAR ...";
      const contract = this.loadNFTContract('escrow');
      const value = utils.format.parseNearAmount("1");  // 1 NEAR to yoctoNEAR
      const response = await contract.take_my_money(
        {}, this.MAX_GAS, value 
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


