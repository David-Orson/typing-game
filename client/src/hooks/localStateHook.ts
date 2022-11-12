import { useStore } from "../store";

export const useLocalState = () => {
  let store = useStore();

  const addState = (field: string, value: any) => {
    store.state[field as keyof typeof store.state] = JSON.stringify(value);
    localStorage.setItem(field, JSON.stringify(value));
  };

  const clearState = () => {
    store.state.authToken = null;
    store.state.account = null;
    localStorage.removeItem("authToken");
    localStorage.removeItem("account");
  };

  return {
    addState,
    clearState,
  };
};
