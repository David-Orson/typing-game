import { createStore } from "vuex";

const environment = "production"

let store = {
    state: {
        apiUrl: "http://localhost:8086",
        metricUrl: "https://api.orsonhub.com",
        authToken: localStorage.getItem("authToken"),
        account: localStorage.getItem("account"),
        sessionPr: localStorage.getItem("sessionPr"),
        sessionToken: localStorage.getItem("sessionToken"),
        environment
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
