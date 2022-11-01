import { AxiosRequestConfig, AxiosResponse } from 'axios';

export type AxiosRequest = <T = any, R = AxiosResponse<T, any>, D = any>(
    url: string,
    body?: any,
    config?: AxiosRequestConfig<D> | undefined
) => Promise<R>;