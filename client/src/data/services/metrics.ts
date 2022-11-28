// hooks
import { useAxios } from "@/hooks/axiosHook";
import { useStore } from "@/store";


export const useMetricService = () => {
    // hooks
    const axios = useAxios();
    const store = useStore();

    // properties
    const metricUrl = store.state.metricUrl;

    const session = {
        accountId: store.getters.account ? store.getters.account.id : null,
        sessionToken: store.state.sessionToken
    }

    const onVisit = async () => {
        const res = await axios.post(`${metricUrl}/visit`, session);

        return res.data;
    };

    const onTestStart = async () => {
        const res = await axios.post(`${metricUrl}/test-start`, session);

        return res.data;
    };

    const onTestEnd = async () => {
        const res = await axios.post(`${metricUrl}/test-end`, session);

        return res.data;
    };

    return {
        onVisit,
        onTestStart,
        onTestEnd,
    };
};
