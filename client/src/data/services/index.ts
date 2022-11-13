import { useAccountService } from "./account";
import { useAuthService } from "./auth";
import { useTestService } from "./test";

export const useServices = () => {
  return {
    ...useAccountService(),
    ...useAuthService(),
    ...useTestService(),
  };
};
