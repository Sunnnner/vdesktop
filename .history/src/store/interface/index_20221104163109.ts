
export interface Config{
  appid: string | null
  app_secret: string | null
  host: string | null
  default_vm: string | null
}


export interface GlobalState {
  config: Config | null,
}
