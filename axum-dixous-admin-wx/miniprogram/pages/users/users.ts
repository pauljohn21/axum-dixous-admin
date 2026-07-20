import { api, UserItem } from '../../utils/api'
import { isLoggedIn } from '../../utils/auth'
import { applyTheme } from '../../utils/theme'

Page({
  data: {
    users: [],
    keyword: '',
    page: 1,
    pageSize: 10,
    total: 0,
    loading: false,
    hasMore: true,
    themeClass: '',
  },

  onLoad() {
    if (!isLoggedIn()) {
      wx.reLaunch({ url: '/pages/login/login' })
      return
    }
    this.loadUsers(true)
  },

  onShow() {
    const theme = applyTheme()
    this.setData({ themeClass: theme === 'dark' ? 'dark' : '' })
  },

  onPullDownRefresh() {
    this.loadUsers(true)
  },

  onReachBottom() {
    if (this.data.hasMore && !this.data.loading) {
      this.loadUsers(false)
    }
  },

  onSearchInput(e: WechatMiniprogram.Input) {
    this.setData({ keyword: e.detail.value })
  },

  onSearch() {
    this.loadUsers(true)
  },

  /// 点击用户项 — 弹出操作菜单
  onUserTap(e: WechatMiniprogram.TouchEventData) {
    const id = e.currentTarget.dataset.id as number
    const enable = e.currentTarget.dataset.enable as number
    const actionText = enable ? '禁用' : '启用'

    wx.showActionSheet({
      itemList: [actionText],
      success: (res) => {
        if (res.tapIndex === 0) {
          this.toggleUserEnable(id, enable)
        }
      },
    })
  },

  /// 切换用户启用/禁用状态
  async toggleUserEnable(id: number, currentEnable: number) {
    const newEnable = currentEnable ? 0 : 1
    try {
      await api.updateUser(id, { enable: newEnable })
      wx.showToast({ title: newEnable ? '已启用' : '已禁用', icon: 'success' })
      this.loadUsers(true)
    } catch (e: any) {
      wx.showToast({ title: e.message || '操作失败', icon: 'none' })
    }
  },

  async loadUsers(reset: boolean) {
    if (this.data.loading) return

    const page = reset ? 1 : this.data.page
    this.setData({ loading: true })

    try {
      const result = await api.userList(page, this.data.pageSize, this.data.keyword)
      const users = reset ? result.list : [...this.data.users, ...result.list]
      const hasMore = users.length < result.total

      this.setData({
        users,
        total: result.total,
        page: page + 1,
        hasMore,
      })
    } catch (e: any) {
      wx.showToast({ title: e.message || '加载失败', icon: 'none' })
    } finally {
      this.setData({ loading: false })
      wx.stopPullDownRefresh()
    }
  },
})
