export interface LoginCreds {
  email: string;
  password: string;
}

export interface SignupCreds {
  username: string;
  email: string;
  password: string;
}

export interface Token {
  hash: string;
  accountId: number;
  username: string;
}
