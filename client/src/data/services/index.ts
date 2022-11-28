import { useAccountService } from "./account";
import { useAuthService } from "./auth";
import { useMetricService } from "./metrics";
import { useTestService } from "./test";

export const useServices = () => {
    return {
        ...useAccountService(),
        ...useAuthService(),
        ...useMetricService(),
        ...useTestService(),
    };
};
