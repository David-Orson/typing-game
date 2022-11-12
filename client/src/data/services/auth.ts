// hooks
import { useAxios } from "@/hooks/axiosHook";
import { useLocalState } from "@/hooks/localStateHook";
import { useStore } from "@/store";

// models
import { LoginCreds, SignupCreds, Account, Token } from "../models";

// services
import { useAccountService } from "../services/account";

export const useAuthService = () => {
  // hooks
  const axios = useAxios();
  const { addState, clearState } = useLocalState();
  const store = useStore();

  // services
  const { getAccount } = useAccountService();

  // properties
  const apiUrl = store.state.apiUrl;

  const getToken = async (loginCreds: LoginCreds) => {
    const res = await axios.post(`${apiUrl}/auth/login`, loginCreds);
    return res.data;
  };

  const logIn = async (loginCreds: LoginCreds) => {
    clearState();

    const token = await getToken(loginCreds);
    const account = await getAccount(token.account_id);

    addLoginState(token, account);
  };

  const signUp = async (signupCreds: SignupCreds) => {
    clearState();

    const { data: account } = await axios.post(
      `${apiUrl}/auth/signup`,
      signupCreds
    );
    const token = await getToken(signupCreds);

    addLoginState(token, account);
  };

  const addLoginState = (token: Token, account: Account) => {
    addState("authToken", token);
    addState("account", account);
  };

  return {
    getToken,
    logIn,
    signUp,
  };
};
