export interface Enterprise {
  id: number;
  enterprise: string;
  group: string;
  type: number;
}
// * 请求响应参数(不包含data)
export interface Result {
  code: string;
  msg: string;
  enterprise: Enterprise[];
}


// * 请求响应参数(包含data)
export interface ResultData<T = any> extends Result {
  [x: string]: any;
}

// * 分页响应参数
export interface ResPage<T> {
  datalist: T[];
  pageNum: number;
  pageSize: number;
  total: number;
}

// * 分页请求参数
export interface ReqPage {
  pageNum: number;
  pageSize: number;
}

// * 登录模块
export namespace Login {
  export interface ReqLoginForm {
    username: string;
    password: string;
    auto_login: boolean;
  }
  export interface ResLogin {
    token: string;
  }
  export interface ResAuthButtons {
    [key: string]: any;
  }
  export interface userInfo {
    id: number;
    username: string;
    name: string;
    phone: string | null;
    enterprise: Enterprise[] | null;
    admin: boolean;
  }
  export interface userPhoto {
    id: number;
    image: string | null;
  }
  export interface UserProfile {
    email: string | null;
    is_bind_weixin: boolean;
    name: string | null;
    phone: string | null;
    photo: userPhoto | null;
    username: string | null;
  }
}

