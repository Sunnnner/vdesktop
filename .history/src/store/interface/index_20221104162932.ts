
export interface Config{
  appid: string | null
  app_secret: string | null
  host: string | null
  default_vm: string | null
}


export interface GlobalState {
  token: string | null,
  config: Config | null,
}
