<template>
  <HelloWorld />
</template>


<script>
import HelloWorld from './components/HelloWorld.vue'
import store from './store'

const nearAPI = require("near-api-js");
const { connect, keyStores, WalletConnection } = nearAPI;

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

export default {
  name: 'App',
  components: {
    HelloWorld,
  }, 
  async mounted() {
        store.commit('setNear', await connect(config));
        store.commit('setNearWallet', new WalletConnection(store.state.near));
        store.commit('setAccountId', store.state.wallet.getAccountId());

    },
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  /* text-align: center;
  color: #2c3e50;
  margin-top: 60px; */
}
</style>
