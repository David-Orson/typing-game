import { useAuthService } from "./auth";

export const useServices = () => {
  return {
    ...useAuthService(),
  };
};
