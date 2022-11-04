
export interface Config{
  appid: string | null
  app_secret: string | null
  host: string | null
  default_vm: string 
}


export interface GlobalState {
  config: Config | null,
}
