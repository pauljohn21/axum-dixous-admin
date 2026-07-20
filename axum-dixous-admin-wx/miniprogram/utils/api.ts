import { post, get, put, del } from './request'

// ==================== 通用类型 ====================

/// 分页响应
export interface PageResponse<T> {
  list: T[]
  total: number
  page: number
  page_size: number
}

// ==================== 用户相关 ====================

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

/// 用户更新参数
export interface UserUpdateParams {
  nick_name?: string
  phone?: string
  email?: string
  enable?: number
}

// ==================== 角色相关 ====================

/// 角色列表项
export interface RoleItem {
  id: number
  name: string | null
  keyword: string | null
  desc: string | null
  created_at: string | null
  updated_at: string | null
}

/// 角色新增/编辑参数
export interface RoleParams {
  name: string
  keyword: string
  desc?: string
}

// ==================== 菜单相关 ====================

/// 菜单项
export interface MenuItem {
  id: number
  parent_id: number | null
  name: string | null
  path: string | null
  component: string | null
  icon: string | null
  sort: number | null
  enable: number | null
  created_at: string | null
  updated_at: string | null
  children?: MenuItem[]
}

// ==================== 字典相关 ====================

/// 字典列表项
export interface DictItem {
  id: number
  name: string | null
  type: string | null
  status: number | null
  desc: string | null
  created_at: string | null
  updated_at: string | null
}

// ==================== API 封装 ====================

export const api = {
  // ========== 用户 ==========
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

  /// 更新用户（启用/禁用等）
  updateUser: (id: number, data: UserUpdateParams) =>
    put(`/api/user/${id}`, data),

  // ========== 角色 ==========
  /// 角色列表
  roleList: (page = 1, page_size = 10, keyword = '') =>
    get<PageResponse<RoleItem>>('/api/role/list', { page, page_size, keyword }),

  /// 新增角色
  createRole: (data: RoleParams) => post('/api/role', data),

  /// 获取角色详情
  getRole: (id: number) => get<RoleItem>(`/api/role/${id}`),

  /// 更新角色
  updateRole: (id: number, data: RoleParams) => put(`/api/role/${id}`, data),

  /// 删除角色
  deleteRole: (id: number) => del(`/api/role/${id}`),

  // ========== 菜单 ==========
  /// 菜单列表（平铺）
  menuList: () => get<MenuItem[]>('/api/menu/list'),

  /// 获取菜单详情
  getMenu: (id: number) => get<MenuItem>(`/api/menu/${id}`),

  // ========== 字典 ==========
  /// 字典列表
  dictList: (page = 1, page_size = 10, keyword = '') =>
    get<PageResponse<DictItem>>('/api/dictionary/list', { page, page_size, keyword }),

  /// 获取字典详情
  getDict: (id: number) => get<DictItem>(`/api/dictionary/${id}`),
}
