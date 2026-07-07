import { TOKEN_KEY } from './config'

/// 获取 token
export function getToken(): string {
  return wx.getStorageSync(TOKEN_KEY) || ''
}

/// 设置 token
export function setToken(token: string): void {
  wx.setStorageSync(TOKEN_KEY, token)
}

/// 清除 token
export function clearToken(): void {
  wx.removeStorageSync(TOKEN_KEY)
}

/// 是否已登录
export function isLoggedIn(): boolean {
  return !!getToken()
}
