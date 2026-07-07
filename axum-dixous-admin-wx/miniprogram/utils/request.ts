import { BASE_URL } from './config'
import { getToken, clearToken } from './auth'

/// 统一响应结构
interface R<T = any> {
  code: number
  message: string
  data: T
}

/// 请求选项
interface RequestOptions {
  url: string
  method?: 'GET' | 'POST' | 'PUT' | 'DELETE'
  data?: any
  auth?: boolean // 是否需要携带 token，默认 true
}

/// 发起请求 — 返回 data 部分，失败抛 Error
export function request<T = any>(opts: RequestOptions): Promise<T> {
  const { url, method = 'GET', data, auth = true } = opts

  const header: Record<string, string> = {
    'Content-Type': 'application/json',
  }

  if (auth) {
    const token = getToken()
    if (token) {
      header['Authorization'] = `Bearer ${token}`
    }
  }

  return new Promise((resolve, reject) => {
    wx.request({
      url: `${BASE_URL}${url}`,
      method,
      data,
      header,
      success: (res) => {
        const body = res.data as R<T>

        // HTTP 401 — token 失效，清除并跳转登录
        if (res.statusCode === 401) {
          clearToken()
          wx.reLaunch({ url: '/pages/login/login' })
          reject(new Error('未授权，请重新登录'))
          return
        }

        // HTTP 错误
        if (res.statusCode >= 400) {
          const msg = (body && body.message) || `请求失败 (${res.statusCode})`
          reject(new Error(msg))
          return
        }

        // 业务错误
        if (body.code !== 200) {
          reject(new Error(body.message || '请求失败'))
          return
        }

        resolve(body.data)
      },
      fail: (err) => {
        reject(new Error(err.errMsg || '网络请求失败'))
      },
    })
  })
}

/// GET 请求
export function get<T = any>(url: string, data?: any): Promise<T> {
  return request<T>({ url, method: 'GET', data })
}

/// POST 请求
export function post<T = any>(url: string, data?: any): Promise<T> {
  return request<T>({ url, method: 'POST', data })
}

/// PUT 请求
export function put<T = any>(url: string, data?: any): Promise<T> {
  return request<T>({ url, method: 'PUT', data })
}

/// DELETE 请求
export function del<T = any>(url: string): Promise<T> {
  return request<T>({ url, method: 'DELETE' })
}
