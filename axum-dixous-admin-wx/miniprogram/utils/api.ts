import { post, get } from './request'

/// 登录请求
export interface LoginParams {
  username: string
  password: string
}

/// 登录响应
export interface LoginResult {
  token: string
}

/// 用户信息
export interface UserInfo {
  username: string
  nick_name: string | null
  phone: string | null
  email: string | null
  header_img: string | null
  wx_openid: string | null
}

/// 仪表盘统计
export interface DashboardStats {
  user_count: number
  role_count: number
  menu_count: number
  api_count: number
}

/// 分页响应
export interface PageResponse<T> {
  list: T[]
  total: number
  page: number
  page_size: number
}

/// 用户列表项
export interface UserItem {
  id: number
  username: string | null
  nick_name: string | null
  phone: string | null
  email: string | null
  enable: number | null
  wx_openid: string | null
  created_at: string | null
}

/// API 封装
export const api = {
  /// 登录
  login: (data: LoginParams) => post<LoginResult>('/api/user/login', data),

  /// 微信登录 — 传入 wx.login 获取的 code
  wxLogin: (code: string) => post<LoginResult>('/api/user/wx-login', { code }),

  /// 绑定微信号 — 将当前账号绑定到微信 openid
  wxBind: (code: string) => post('/api/user/bind-wechat', { code }),

  /// 退出登录
  logout: () => post('/api/user/logout'),

  /// 获取用户信息
  getUserInfo: () => get<UserInfo>('/api/user/info'),

  /// 修改密码
  changePassword: (old_password: string, new_password: string) =>
    put('/api/user/change_password', { old_password, new_password }),

  /// 仪表盘统计
  dashboardStats: () => get<DashboardStats>('/api/dashboard/stats'),

  /// 用户列表
  userList: (page = 1, page_size = 10, keyword = '') =>
    get<PageResponse<UserItem>>('/api/user/list', { page, page_size, keyword }),
}
