import { api, UserItem } from '../../utils/api'
import { isLoggedIn } from '../../utils/auth'

Page({
  data: {
    users: [],
    keyword: '',
    page: 1,
    pageSize: 10,
    total: 0,
    loading: false,
    hasMore: true,
  },

  onLoad() {
    if (!isLoggedIn()) {
      wx.reLaunch({ url: '/pages/login/login' })
      return
    }
    this.loadUsers(true)
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
