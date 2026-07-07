import { api, UserInfo } from '../../utils/api'
import { isLoggedIn, clearToken } from '../../utils/auth'

Page({
  data: {
    userInfo: null,
    loading: true,
    showPasswordForm: false,
    oldPassword: '',
    newPassword: '',
    confirmPassword: '',
    wxBinding: false,
  },

  onLoad() {
    if (!isLoggedIn()) {
      wx.reLaunch({ url: '/pages/login/login' })
      return
    }
    this.loadUserInfo()
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

    try {
      await api.changePassword(oldPassword, newPassword)
      wx.showToast({ title: '密码修改成功', icon: 'success' })
      this.setData({ showPasswordForm: false, oldPassword: '', newPassword: '', confirmPassword: '' })
    } catch (e: any) {
      wx.showToast({ title: e.message || '修改失败', icon: 'none' })
    }
  },

  /// 绑定微信号
  async bindWechat() {
    // 已绑定则提示
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
      // 重新加载用户信息以更新绑定状态
      await this.loadUserInfo()
    } catch (e: any) {
      wx.showToast({ title: e.message || '绑定失败', icon: 'none' })
    } finally {
      this.setData({ wxBinding: false })
    }
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
