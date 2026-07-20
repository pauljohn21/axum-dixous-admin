import { api } from '../../utils/api'
import { setToken } from '../../utils/auth'
import { USERNAME_KEY } from '../../utils/config'
import { applyTheme } from '../../utils/theme'

Page({
  data: {
    username: 'admin',
    password: '',
    loading: false,
    wxLoading: false,
    errorMsg: '',
    themeClass: '',
  },

  onShow() {
    const theme = applyTheme()
    this.setData({ themeClass: theme === 'dark' ? 'dark' : '' })
  },

  onUsernameInput(e: WechatMiniprogram.Input) {
    this.setData({ username: e.detail.value })
  },

  onPasswordInput(e: WechatMiniprogram.Input) {
    this.setData({ password: e.detail.value })
  },

  /// 账号密码登录
  async doLogin() {
    const { username, password } = this.data

    if (!username || !password) {
      this.setData({ errorMsg: '请输入用户名和密码' })
      return
    }

    this.setData({ loading: true, errorMsg: '' })

    try {
      const result = await api.login({ username, password })
      setToken(result.token)
      wx.setStorageSync(USERNAME_KEY, username)

      wx.showToast({ title: '登录成功', icon: 'success' })
      setTimeout(() => {
        wx.switchTab({ url: '/pages/dashboard/dashboard' })
      }, 500)
    } catch (e: any) {
      this.setData({ errorMsg: e.message || '登录失败' })
    } finally {
      this.setData({ loading: false })
    }
  },

  /// 微信一键登录
  async doWxLogin() {
    this.setData({ wxLoading: true, errorMsg: '' })

    try {
      // 1. 调用 wx.login 获取临时 code
      const { code } = await wx.login()

      if (!code) {
        this.setData({ errorMsg: '获取微信登录凭证失败' })
        return
      }

      // 2. 将 code 发送到后端，换取 JWT token
      const result = await api.wxLogin(code)
      setToken(result.token)
      wx.setStorageSync(USERNAME_KEY, '微信用户')

      wx.showToast({ title: '登录成功', icon: 'success' })
      setTimeout(() => {
        wx.switchTab({ url: '/pages/dashboard/dashboard' })
      }, 500)
    } catch (e: any) {
      this.setData({ errorMsg: e.message || '微信登录失败' })
    } finally {
      this.setData({ wxLoading: false })
    }
  },
})
