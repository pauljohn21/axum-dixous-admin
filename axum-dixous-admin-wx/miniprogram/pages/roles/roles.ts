import { api, RoleItem, RoleParams } from '../../utils/api'
import { isLoggedIn } from '../../utils/auth'
import { applyTheme } from '../../utils/theme'

Page({
  data: {
    roles: [] as RoleItem[],
    keyword: '',
    page: 1,
    pageSize: 10,
    total: 0,
    loading: false,
    hasMore: true,
    themeClass: '',
    // 弹窗
    showForm: false,
    editingId: null as number | null,
    formName: '',
    formKeyword: '',
    formDesc: '',
    saving: false,
  },

  onLoad() {
    if (!isLoggedIn()) {
      wx.reLaunch({ url: '/pages/login/login' })
      return
    }
    this.loadRoles(true)
  },

  onShow() {
    const theme = applyTheme()
    this.setData({ themeClass: theme === 'dark' ? 'dark' : '' })
  },

  onPullDownRefresh() {
    this.loadRoles(true)
  },

  onReachBottom() {
    if (this.data.hasMore && !this.data.loading) {
      this.loadRoles(false)
    }
  },

  onSearchInput(e: WechatMiniprogram.Input) {
    this.setData({ keyword: e.detail.value })
  },

  onSearch() {
    this.loadRoles(true)
  },

  async loadRoles(reset: boolean) {
    if (this.data.loading) return

    const page = reset ? 1 : this.data.page
    this.setData({ loading: true })

    try {
      const result = await api.roleList(page, this.data.pageSize, this.data.keyword)
      const roles = reset ? result.list : [...this.data.roles, ...result.list]
      const hasMore = roles.length < result.total

      this.setData({
        roles,
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

  /// 打开新增弹窗
  onAdd() {
    this.setData({
      showForm: true,
      editingId: null,
      formName: '',
      formKeyword: '',
      formDesc: '',
    })
  },

  /// 打开编辑弹窗
  onEdit(e: WechatMiniprogram.TouchEvent) {
    const id = e.currentTarget.dataset.id as number
    const role = this.data.roles.find(r => r.id === id)
    if (!role) return

    this.setData({
      showForm: true,
      editingId: id,
      formName: role.name || '',
      formKeyword: role.keyword || '',
      formDesc: role.desc || '',
    })
  },

  onFormNameInput(e: WechatMiniprogram.Input) {
    this.setData({ formName: e.detail.value })
  },

  onFormKeywordInput(e: WechatMiniprogram.Input) {
    this.setData({ formKeyword: e.detail.value })
  },

  onFormDescInput(e: WechatMiniprogram.Input) {
    this.setData({ formDesc: e.detail.value })
  },

  /// 保存角色（新增或编辑）
  async onSave() {
    const { formName, formKeyword, editingId } = this.data

    if (!formName || !formKeyword) {
      wx.showToast({ title: '请填写角色名称和标识', icon: 'none' })
      return
    }

    const params: RoleParams = {
      name: formName,
      keyword: formKeyword,
      desc: this.data.formDesc || undefined,
    }

    this.setData({ saving: true })
    try {
      if (editingId) {
        await api.updateRole(editingId, params)
        wx.showToast({ title: '修改成功', icon: 'success' })
      } else {
        await api.createRole(params)
        wx.showToast({ title: '新增成功', icon: 'success' })
      }
      this.setData({ showForm: false })
      this.loadRoles(true)
    } catch (e: any) {
      wx.showToast({ title: e.message || '保存失败', icon: 'none' })
    } finally {
      this.setData({ saving: false })
    }
  },

  /// 关闭弹窗
  onFormClose() {
    this.setData({ showForm: false })
  },

  /// 删除角色
  onDelete(e: WechatMiniprogram.TouchEvent) {
    const id = e.currentTarget.dataset.id as number
    const role = this.data.roles.find(r => r.id === id)

    wx.showModal({
      title: '确认删除',
      content: `确定要删除角色「${role?.name || ''}」吗？`,
      dangerColor: true,
      success: async (res) => {
        if (res.confirm) {
          try {
            await api.deleteRole(id)
            wx.showToast({ title: '删除成功', icon: 'success' })
            this.loadRoles(true)
          } catch (e: any) {
            wx.showToast({ title: e.message || '删除失败', icon: 'none' })
          }
        }
      },
    })
  },

  /// 阻止弹窗内容点击冒泡
  stopPropagation() {},
})
