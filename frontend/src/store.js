import { createStore } from 'vuex';
import * as nearApi from 'near-api-js';
import getConfigurations from './config';



const store = createStore({
  state: {
    contract: null,
    currentUser: null,
    wallet: null,
    nearConfig: null
  },
  getters: {
    contract: state => state.contract,
    currentUser: state => state.currentUser,
    wallet: state => state.wallet,
    nearConfig: state => state.nearConfig
  },
  mutations: {
    setupNear(state, payload) {
      state.contract = payload.contract;
      state.currentUser = payload.currentUser;
      state.wallet = payload.wallet;
      state.nearConfig = payload.nearConfig;
    }
  },
  actions: {
    async initNear({ commit }) {
      const nearConfig = getConfigurations('testnet');
      const near = await nearApi.connect({
        deps: {
          keyStore: new nearApi.keyStores.BrowserLocalStorageKeyStore()
        },
        ...nearConfig
      });

      const wallet = new nearApi.WalletConnection(near);

      let currentUser;

      if (wallet.getAccountId()) {
        currentUser = {
          accountId: wallet.getAccountId(),
          balance: (await wallet.account().state()).amount,
          balanceInNear : (await wallet.account().state()).amount / (10 ** 24),
        }
      }

      console.log(currentUser)

      const contract = await new nearApi.Contract(wallet.account(), process.env.VUE_APP_CONTRACT_NAME || 'crypto-keep.testnet', {
        viewMethods: ['get_passwords'],
        changeMethods: ['add_password','update_password','delete_password'],
        sender: wallet.getAccountId()
      });
      console.log(contract);
      // Commit and send to mutation.
      commit('setupNear', { contract, currentUser, wallet, nearConfig });
    }
  }
});

export default store;