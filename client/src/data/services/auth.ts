// hooks
import { useAxios } from "@/hooks/axiosHook";

// models
import { LoginCreds, SignupCreds } from "../models";

export const useAuthService = () => {
  // hooks
  const axios = useAxios();

  // properties
  const apiUrl = "http://localhost:8085";

  const getToken = async (loginCreds: LoginCreds) => {
    const res = await axios.post(`${apiUrl}/auth/login`, loginCreds);
    return res.data;
  };

  const logIn = async (loginCreds: LoginCreds) => {
    const token = await getToken(loginCreds);
  };

  const signUp = async (signupCreds: SignupCreds) => {
    const { data: account } = await axios.post(
      `${apiUrl}/auth/signup`,
      signupCreds
    );
    const token = await getToken(signupCreds);
  };

  return {
    getToken,
    logIn,
    signUp,
  };
};
