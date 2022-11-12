import { createStore } from "vuex";

let store = {
  state: {
    apiUrl: "http://localhost:8085",
    authToken: localStorage.getItem("authToken"),
    account: localStorage.getItem("account"),
  },
  getters: {
    account: (state: any) => {
      if (state.account) {
        return JSON.parse(state.account);
      }
    },
  },
  mutations: {},
  actions: {},
  modules: {},
};

export const useStore = () => {
  return createStore(store);
};

export default createStore(store);
