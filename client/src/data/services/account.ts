// hooks
import { useAxios } from "@/hooks/axiosHook";
import { useStore } from "@/store";
import { useLocalState } from "@/hooks/localStateHook";

// models
import { Account } from "../models";

export const useAccountService = () => {
  // hooks
  const store = useStore();
  const { addState } = useLocalState();
  const axios = useAxios();

  // properties
  const apiUrl = store.state.apiUrl;

  // methods
  const getAccount = async (id: number): Promise<Account> => {
    const res = await axios.get(`${apiUrl}/account/${id}`);

    if (store.getters.account && store.getters.account.id === res.data.id) {
      addState("account", res.data);
    }

    return res.data;
  };

  return {
    getAccount,
  };
};
