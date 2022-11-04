
export interface Config{
  appid: string | null
  app_secret: string
  host: string 
  default_vm: string 
}


export interface GlobalState {
  config: Config | null,
}
