// hooks
import { useAxios } from "@/hooks/axiosHook";
import { useStore } from "@/store";

// models
import { Account } from "../models";

export const useAccountService = () => {
  const store = useStore();
  // hooks
  const apiUrl = store.state.apiUrl;
  const axios = useAxios();

  // methods
  const getAccount = async (id: number): Promise<Account> => {
    const res = await axios.get(`${apiUrl}/account/${id}`);
    return res.data;
  };

  return {
    getAccount,
  };
};
