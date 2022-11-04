
export interface Config{
  appid: string;
  appsecret: string;
  url: string;
  'default-vm': string;
}


export interface GlobalState {
  token: string | null,
  config: Config | null,
}
