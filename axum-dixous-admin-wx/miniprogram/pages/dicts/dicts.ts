import { api, DictItem } from '../../utils/api'
import { isLoggedIn } from '../../utils/auth'
import { applyTheme } from '../../utils/theme'

Page({
  data: {
    dicts: [] as DictItem[],
    keyword: '',
    page: 1,
    pageSize: 10,
    total: 0,
    loading: false,
    hasMore: true,
    themeClass: '',
    selectedDict: null as DictItem | null,
    showDetail: false,
  },

  onLoad() {
    if (!isLoggedIn()) {
      wx.reLaunch({ url: '/pages/login/login' })
      return
    }
    this.loadDicts(true)
  },

  onShow() {
    const theme = applyTheme()
    this.setData({ themeClass: theme === 'dark' ? 'dark' : '' })
  },

  onPullDownRefresh() {
    this.loadDicts(true)
  },

  onReachBottom() {
    if (this.data.hasMore && !this.data.loading) {
      this.loadDicts(false)
    }
  },

  onSearchInput(e: WechatMiniprogram.Input) {
    this.setData({ keyword: e.detail.value })
  },

  onSearch() {
    this.loadDicts(true)
  },

  async loadDicts(reset: boolean) {
    if (this.data.loading) return

    const page = reset ? 1 : this.data.page
    this.setData({ loading: true })

    try {
      const result = await api.dictList(page, this.data.pageSize, this.data.keyword)
      const dicts = reset ? result.list : [...this.data.dicts, ...result.list]
      const hasMore = dicts.length < result.total

      this.setData({
        dicts,
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

  /// 点击字典项查看详情
  async onDictTap(e: WechatMiniprogram.TouchEvent) {
    const id = e.currentTarget.dataset.id as number
    try {
      const dict = await api.getDict(id)
      this.setData({ selectedDict: dict, showDetail: true })
    } catch (e: any) {
      wx.showToast({ title: e.message || '加载失败', icon: 'none' })
    }
  },

  onDetailClose() {
    this.setData({ showDetail: false })
  },

  stopPropagation() {},
})
