import { useAccountService } from "./account";
import { useAuthService } from "./auth";

export const useServices = () => {
  return {
    ...useAccountService(),
    ...useAuthService(),
  };
};
