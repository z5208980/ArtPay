<template>
  <div class="my-5">
    <nav-bar></nav-bar>

    <h1 class="font-bold text-center">{{ msg }}</h1>
    <div class="container mx-auto px-16">
      Message: {{ globalMessage }}
    </div>

<div class="container">
    <div class="grid grid-cols-5 gap-3 mx-16">
      <div class="col-span-2">
          <h1 class="font-bold text-center">ArtPay NFT</h1>
          <template v-if="!accountId">
            Login to continue!
          </template>
          <template v-else>
            <button @click="getMetaData()" :class="btnStyle">
              getMetaData
            </button>
            <h2 class="font-bold">Mint new NFT</h2>
            <!-- <div class="py-2">Token ID</div>
            <input v-model="newNFT.tokenId" :class="inputStyle" type="text" /> -->

            <div class="mb-6">
              <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">Token Id</label>
              <input v-model="newNFT.tokenId" type="text" :class="inputStyle" />
            </div>

            <div class="mb-6">
              <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">Licence / Copyright agreement</label>
              <textarea v-model="newNFT.copyright" row=4 :class="inputStyle" />
            </div>            

            <div class="mb-6">
              <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">Copyright</label>
              <select v-model="newNFT.rightAssign" :class="inputStyle">
                <option v-for="(item, i) in rightAssign" :key="i" :value="item">
                  {{item}}
                </option>
              </select>
            </div>  

            <div class="mb-6">
              <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">Composition or derivative work?</label>
              <div v-for="(item, i) in newNFT.attributeParties" :key="i">
                <input v-model="item.address" :class="inputStyle" /> %royalty <input v-model="item.value" :class="inputStyle" type="number" min="0" max="100" />
                <button @click="deleteParty(index)" :class="btnStyle">
                delete
                </button>
              </div>
            </div>  

            <div class="mb-6">
              <button @click="addParty" :class="btnStyle">
                Add Party
              </button>
            </div>
          <div class="mb-6">
            <button @click="mintNFT()" :class="btnStyle">
              mintNFT
            </button>
            <p class="text-sm"><b>Note</b> Minting will incur {{ MINT_PRICE }} yoctoNEAR. Reason: For storage</p>
          </div>

          <hr class="mb-6">

          <h2 class="font-bold">View NFT</h2>
          <input v-model="viewNFT.tokenId" :class="inputStyle" type="text" />
          <button @click="viewNFTToken()" :class="btnStyle">
            viewNFTToken
          </button>
          </template>
        </div>
      <div class="col-span-3">
          <h1 class="font-bold text-center">ArtPay Escrow</h1>

          <h2 class="font-bold">Create Escrow</h2>
          <div class="py-2">Contractor Address</div>
          <input v-model="newEscrowFormData.contractor" :class="inputStyle" placeholder="contractor address" type="text"/>
          <div class="py-2">Project Title</div>
          <input v-model="newEscrowFormData.title" :class="inputStyle" placeholder="Title" type="text" />
          <div class="py-2">Type</div>
          <select v-model="newEscrowFormData.escrowType" :class="inputStyle">
            <option value="1">Custom Artwork</option>
            <option value="2">Artwork 1</option>
            <option value="3">Free Artwork</option>
            <option value="4">Artwork Custom</option>
          </select>
          <div class="py-2">Description</div>
          <input v-model="newEscrowFormData.description" :class="inputStyle" placeholder="Description" type="text" />
          <div class="py-2">Requirements</div>
          <input v-model="newEscrowFormData.description" :class="inputStyle" placeholder="Description" type="text" />   
          <div class="py-2">Expiry Date</div>
          <input v-model="newEscrowFormData.timestamp" :class="inputStyle" placeholder="Expiry Date" type="date" />
          <div class="py-2">Locked Funds (Near)</div>
          <input v-model="newEscrowFormData.lockedFunds" :class="inputStyle" placeholder="Funds in NEAR" type="number" step="0.01" />
          <button @click="createEscrow()" :class="btnStyle">
            createEscrow
          </button>
          
          <h2 class="font-bold">setNFTDeliverable</h2>
          <div class="py-2">Client</div>
          <input v-model="escrowDeliverable.client" :class="inputStyle" placeholder="NFT Address" type="text" />
          <div class="py-2">Escrow Id</div>
          <input v-model="escrowDeliverable.escrowId" :class="inputStyle" placeholder="TokenId for NFT" type="text" />
          <div class="py-2">NFT Address</div>
          <input v-model="escrowDeliverable.nftAddress" :class="inputStyle" placeholder="NFT Address" type="text" />
          <div class="py-2">Token TokenId</div>
          <input v-model="escrowDeliverable.tokenId" :class="inputStyle" placeholder="TokenId for NFT" type="text" />          
          <button @click="setNFTDeliverable()" :class="btnStyle">
            setNFTDeliverable
          </button>
          <p class="text-sm">This will transfer owner_id of NFT to escrow.artpay.testnet</p>


          <h2 class="font-bold">View Escrow</h2>
          <input type="checkbox" id="checkbox" v-model="viewEscrow.you">
          <label for="checkbox">Are you client of escrow? {{ viewEscrow.you }}</label>
          <div class="py-2">Other Address</div>
          <input v-model="viewEscrow.client" :class="inputStyle" placeholder="Client escrow" type="text" />
          <div class="py-2">Escrow ID</div>
          <input v-model="viewEscrow.tokenId" :class="inputStyle" placeholder="Escrow Id" type="text" />
          <button @click="viewEscrowToken()" :class="btnStyle">
            viewEscrow
          </button>
          <template v-if="viewEscrow.you">
            <button @click="approveClient()" :class="btnStyle">
              approveClient
            </button>
          </template>
          <template v-else>
            <button @click="approveContractor()" :class="btnStyle">
              approveContractor
            </button>
          </template>

          <button @click="releaseEscrow()" :class="btnStyle">
            releaseEscrow
          </button>

          <hr>

          <button @click="viewEscrowAsClient()" :class="btnStyle">
            viewEscrowAsClient
          </button>

          <button @click="viewEscrowAsContractor()" :class="btnStyle">
            viewEscrowAsContractor
          </button>
      </div>
    </div>     
</div>
</div>
</template>

<script>
import NavBar from './NavBar.vue';
import store from '../store';

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
    viewMethods: ["get_escrow", "get_escrow_new", "get_escrows_as_contractor", "get_escrows_as_client"],
    changeMethods: ["release_escrow", "contractor_approval", "client_approval", 
    "create_new_escrow", "take_my_money", "set_deliverable", "set_nft_deliverable",
   ],
  }
}

export default {
  name: "HelloWorld",
  components: {
    NavBar
  },
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
      rightAssign: ["BYND", "BYNC", "BYSA", "BYNCND", "BYNCSA", "FULL"],

      // nft
      newNFT: {
        tokenId: "token-1",
        copyright: "Copyright © 2022 ArtPay all rights reserved. See URL for more information.",
        rightAssign: "BYND",
        attributeParties: []
      },
      viewNFT: {
        tokenId: store.state.count,
      },

      // escrow
      newEscrowFormData: {
        contractor: "example.tesnet",
        // nftAddress: "escrow.artpay.testnet",
        // tokenId: "token-1",
        timestamp: new Date().toISOString().slice(0, 10),
        lockedFunds: 0,
        title: "Project Title",
        description: "description",
        escrowType: "1",
        requirements: "Create NFT art of a donkey!"
      },
      escrowDeliverable: {
        client: "artpay.testnet",
        escrowId: "1",
        nftAddress: "nft.artpay.testnet",
        tokenId: "token-1",
      },
      viewEscrow: {
        you: true,
        client: "artpay.testnet",
        tokenId: "1",
      },
    }
  },
  methods: {
    async init() {
      this.urlParams = new URLSearchParams(window.location.search);
      this.near = await connect(config);
      this.wallet = new WalletConnection(this.near);
      
      this.globalMessage = `Transaction: ${this.urlParams.has("transactionHashes") ? this.urlParams.get("transactionHashes") : ""}`;
      this.accountId = (this.wallet.isSignedIn() && this.wallet.getAccountId());
    },
    async login() {    
      await store.state.wallet.requestSignIn(abi.nft.contractAddr, "ArtPay");
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
      let attributeParties = {}
      for (let i of this.newNFT.attributeParties) {
        attributeParties[i.address] = i.value;
      }

      console.log(this.newNFT.rightAssign);
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
            copyright: this.newNFT.copyright, // IMPORTANT FOR ARTPAY NFT
            right_assign: this.newNFT.rightAssign, // IMPORTANT FOR ARTPAY NFT
          },
          perpetual_royalties: attributeParties,
        },
        this.MAX_GAS, this.MINT_PRICE,
      );
      this.globalMessage = `Token minted as: ${response}`;
    },
    async createEscrow() {
      // See https://docs.near.org/docs/tutorials/contracts/nfts/royalty#creating-a-sub-account for royalty
      let expiryDate = new Date(this.newEscrowFormData.timestamp);
      const contractNFT = this.loadNFTContract('escrow');
      await contractNFT.create_new_escrow(
        {
          contractor: this.newEscrowFormData.contractor,
          // nft_address: this.newEscrowFormData.nftAddress,
          // token_id: this.newEscrowFormData.tokenId,
          timestamp: expiryDate.getTime(),
          title: this.newEscrowFormData.title,
          description: this.newEscrowFormData.description,
          escrow_type: this.newEscrowFormData.escrowType,
          requirement: "IPFS documentation", 
          license_code: "Licencse Code", license_desc: "Example Licence", license_url: "IPFS Link"
        }, this.MAX_GAS, utils.format.parseNearAmount(this.newEscrowFormData.lockedFunds.toString())
      );
    },
    async viewEscrowAsClient() {
      const contractNFT = this.loadNFTContract('escrow');
      this.globalMessage = await contractNFT.get_escrows_as_client({ account_id: this.wallet.getAccountId()});
      console.log(this.globalMessage);
    },
    async viewEscrowAsContractor() {
      const contractNFT = this.loadNFTContract('escrow');
      this.globalMessage = await contractNFT.get_escrows_as_contractor({ account_id: this.wallet.getAccountId()});
      console.log(this.globalMessage);
    },
    async approveClient() {
      const contractNFT = this.loadNFTContract('escrow');
      await contractNFT.client_approval({ contractor: this.viewEscrow.client, id: Number(this.viewEscrow.tokenId) });
    },
    async approveContractor() {
      const contractNFT = this.loadNFTContract('escrow');
      await contractNFT.contractor_approval({ client: this.viewEscrow.client, id: Number(this.viewEscrow.tokenId) });
    },
    async setNFTDeliverable() {
      const contractNFT = this.loadNFTContract('escrow');
      // this.globalMessage = await contractNFT.nft_transfer(
      //   { receiver_id: "escrow.artpay.testnet", token_id: this.escrowDeliverable.tokenId.toString(), approval_id: 0, msg: "For ArtPay Escrow" },
      //   this.MAX_GAS, 1
      // );

      this.globalMessage = await contractNFT.set_nft_deliverable({
        client: this.escrowDeliverable.client, id: Number(this.escrowDeliverable.escrowId), 
        nft_address: this.escrowDeliverable.nftAddress, "token_id": this.escrowDeliverable.tokenId.toString()
      }, 200000000000000, 1)

      this.globalMessage.then(result => {
        console.log(result);
      });
      // this.globalMessage = `Setting Deliverable ...`;
      // const contractEscrow = this.loadNFTContract('escrow');
      // // this.globalMessage = await contractNFT.set_nft_deliverable(
      // this.globalMessage = await contractEscrow.set_deliverable(
      //   { 
      //     client: this.escrowDeliverable.client, id: Number(this.escrowDeliverable.escrowId),
      //     nft_address: this.escrowDeliverable.nftAddress, token_id: this.escrowDeliverable.tokenId
      //   },
      // );
      // this.globalMessage = `NFT setted to ${this.escrowDeliverable.client}'s ${this.escrowDeliverable.escrowId} Escrow!`;
    },
    async viewEscrowToken() {
      this.globalMessage = `Getting Escrow`;
      const contractEscrow = this.loadNFTContract('escrow');

      let args =  { client: this.wallet.getAccountId(), contractor: this.viewEscrow.client, id: Number(this.viewEscrow.tokenId) }
      if (!this.viewEscrow.you) {
        args = { client: this.viewEscrow.client, contractor: this.wallet.getAccountId(), id: Number(this.viewEscrow.tokenId) }
      } 
      this.globalMessage = await contractEscrow.get_escrow_new(args);
    },
    async releaseEscrow() {
      const contractEscrow = this.loadNFTContract('escrow');
      let args =  { client: this.wallet.getAccountId(), contractor: this.viewEscrow.client, id: Number(this.viewEscrow.tokenId) }
      if (!this.viewEscrow.you) {
        args = { client: this.viewEscrow.client, contractor: this.wallet.getAccountId(), id: Number(this.viewEscrow.tokenId) }
      } 
      await contractEscrow.release_escrow(args, 200000000000000, 1)
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
    },
    addParty: function () {
      this.newNFT.attributeParties.push({ address: '', value: 0 });
    },
    deleteParty: function (index) {
      this.newNFT.attributeParties.splice(index, 1);
    }
  },
  mounted() { this.init(); },
  computed: {
    btnStyle: function () {
      return "mb-6 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
    },
    inputStyle: function () {
      return "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";
    },
  }
}
</script>


