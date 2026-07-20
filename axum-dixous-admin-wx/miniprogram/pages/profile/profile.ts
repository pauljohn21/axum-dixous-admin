import { api, UserInfo } from '../../utils/api'
import { isLoggedIn, clearToken } from '../../utils/auth'
import { applyTheme, toggleTheme } from '../../utils/theme'

Page({
  data: {
    userInfo: null as UserInfo | null,
    loading: true,
    showPasswordForm: false,
    oldPassword: '',
    newPassword: '',
    confirmPassword: '',
    wxBinding: false,
    themeClass: '',
    isDark: false,
  },

  onLoad() {
    if (!isLoggedIn()) {
      wx.reLaunch({ url: '/pages/login/login' })
      return
    }
    this.loadUserInfo()
  },

  onShow() {
    const theme = applyTheme()
    this.setData({
      themeClass: theme === 'dark' ? 'dark' : '',
      isDark: theme === 'dark',
    })
  },

  onPullDownRefresh() {
    this.loadUserInfo()
  },

  async loadUserInfo() {
    this.setData({ loading: true })
    try {
      const info = await api.getUserInfo()
      this.setData({ userInfo: info })
    } catch (e: any) {
      wx.showToast({ title: e.message || '加载失败', icon: 'none' })
    } finally {
      this.setData({ loading: false })
      wx.stopPullDownRefresh()
    }
  },

  togglePasswordForm() {
    this.setData({
      showPasswordForm: !this.data.showPasswordForm,
      oldPassword: '',
      newPassword: '',
      confirmPassword: '',
    })
  },

  onOldPasswordInput(e: WechatMiniprogram.Input) {
    this.setData({ oldPassword: e.detail.value })
  },

  onNewPasswordInput(e: WechatMiniprogram.Input) {
    this.setData({ newPassword: e.detail.value })
  },

  onConfirmPasswordInput(e: WechatMiniprogram.Input) {
    this.setData({ confirmPassword: e.detail.value })
  },

  async changePassword() {
    const { oldPassword, newPassword, confirmPassword } = this.data

    if (!oldPassword || !newPassword || !confirmPassword) {
      wx.showToast({ title: '请填写完整', icon: 'none' })
      return
    }

    if (newPassword !== confirmPassword) {
      wx.showToast({ title: '两次密码不一致', icon: 'none' })
      return
    }

    if (newPassword.length < 6) {
      wx.showToast({ title: '密码至少 6 位', icon: 'none' })
      return
    }

    try {
      await api.changePassword(oldPassword, newPassword)
      wx.showToast({ title: '密码修改成功', icon: 'success' })
      this.setData({ showPasswordForm: false, oldPassword: '', newPassword: '', confirmPassword: '' })
    } catch (e: any) {
      wx.showToast({ title: e.message || '修改失败', icon: 'none' })
    }
  },

  /// 切换暗色/亮色主题
  onThemeToggle() {
    const theme = toggleTheme()
    applyTheme(theme)
    this.setData({
      themeClass: theme === 'dark' ? 'dark' : '',
      isDark: theme === 'dark',
    })
  },

  /// 绑定微信号
  async bindWechat() {
    if (this.data.userInfo && this.data.userInfo.wx_openid) {
      wx.showModal({
        title: '提示',
        content: '当前账号已绑定微信号，确定要重新绑定吗？',
        success: async (res) => {
          if (res.confirm) {
            await this.doBindWechat()
          }
        },
      })
      return
    }

    await this.doBindWechat()
  },

  async doBindWechat() {
    this.setData({ wxBinding: true })
    try {
      const { code } = await wx.login()
      if (!code) {
        wx.showToast({ title: '获取微信凭证失败', icon: 'none' })
        return
      }

      await api.wxBind(code)
      wx.showToast({ title: '绑定成功', icon: 'success' })
      await this.loadUserInfo()
    } catch (e: any) {
      wx.showToast({ title: e.message || '绑定失败', icon: 'none' })
    } finally {
      this.setData({ wxBinding: false })
    }
  },

  /// 跳转菜单管理
  goToMenus() {
    wx.navigateTo({ url: '/pages/menus/menus' })
  },

  /// 跳转字典管理
  goToDicts() {
    wx.navigateTo({ url: '/pages/dicts/dicts' })
  },

  async doLogout() {
    wx.showModal({
      title: '提示',
      content: '确定退出登录吗？',
      success: async (res) => {
        if (res.confirm) {
          try {
            await api.logout()
          } catch (e) {
            // 忽略退出登录的接口错误
          }
          clearToken()
          wx.reLaunch({ url: '/pages/login/login' })
        }
      },
    })
  },
})
