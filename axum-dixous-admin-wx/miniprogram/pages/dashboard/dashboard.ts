import { api, DashboardStats } from '../../utils/api'
import { isLoggedIn } from '../../utils/auth'

Page({
  data: {
    stats: null as DashboardStats | null,
    loading: true,
    systemInfo: {
      backend: 'Axum 0.8',
      frontend: 'WeChat MiniProgram',
      orm: 'SeaORM 1',
      database: 'MySQL 8',
      auth: 'Casbin RBAC',
    },
  },

  onLoad() {
    if (!isLoggedIn()) {
      wx.reLaunch({ url: '/pages/login/login' })
      return
    }
    this.loadStats()
  },

  onPullDownRefresh() {
    this.loadStats()
  },

  async loadStats() {
    this.setData({ loading: true })
    try {
      const stats = await api.dashboardStats()
      this.setData({ stats })
    } catch (e: any) {
      wx.showToast({ title: e.message || '加载失败', icon: 'none' })
    } finally {
      this.setData({ loading: false })
      wx.stopPullDownRefresh()
    }
  },
})
