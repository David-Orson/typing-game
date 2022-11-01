// npm
import axios from "axios";

// models
import { AxiosRequest } from "@/data/models";

export const useAxios = () => {
  const wrapRequest =
    (reqMethod: AxiosRequest) =>
    (url: string, body?: {}): any => {
      return reqMethod(url, body).catch((err: any) => {
        console.error(err);
      });
    };

  return {
    get: wrapRequest(axios.get),
    post: wrapRequest(axios.post),
    put: wrapRequest(axios.put),
    deleteId: wrapRequest(axios.delete),
  };
};
