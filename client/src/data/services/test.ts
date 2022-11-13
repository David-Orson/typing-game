// hooks
import { useAxios } from "@/hooks/axiosHook";
import { useStore } from "@/store";

// models

export const useTestService = () => {
  // hooks
  const store = useStore();
  const apiUrl = store.state.apiUrl;
  const axios = useAxios();

  // methods
  const finishTest = async (test: string, typed: string) => {
    await axios.post(`${apiUrl}/test/finish`, {
      test,
      typed,
      account: store.getters.account.id,
    });
  };

  return {
    finishTest,
  };
};
