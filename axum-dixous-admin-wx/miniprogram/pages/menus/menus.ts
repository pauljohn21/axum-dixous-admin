import { api, MenuItem } from '../../utils/api'
import { isLoggedIn } from '../../utils/auth'
import { applyTheme } from '../../utils/theme'

/// 树形菜单节点（扩展 children + collapsed）
interface MenuTreeNode extends MenuItem {
  children: MenuTreeNode[]
  collapsed: boolean
  level: number
}

Page({
  data: {
    tree: [] as MenuTreeNode[],
    flatList: [] as MenuTreeNode[],
    loading: true,
    themeClass: '',
    selectedMenu: null as MenuItem | null,
    showDetail: false,
  },

  onLoad() {
    if (!isLoggedIn()) {
      wx.reLaunch({ url: '/pages/login/login' })
      return
    }
    this.loadMenus()
  },

  onShow() {
    const theme = applyTheme()
    this.setData({ themeClass: theme === 'dark' ? 'dark' : '' })
  },

  onPullDownRefresh() {
    this.loadMenus()
  },

  async loadMenus() {
    this.setData({ loading: true })
    try {
      const list = await api.menuList()
      const tree = this.buildTree(list)
      this.setData({ tree })
      this.updateFlatList()
    } catch (e: any) {
      wx.showToast({ title: e.message || '加载失败', icon: 'none' })
    } finally {
      this.setData({ loading: false })
      wx.stopPullDownRefresh()
    }
  },

  /// 构建树结构
  buildTree(list: MenuItem[]): MenuTreeNode[] {
    const map = new Map<number, MenuTreeNode>()
    const roots: MenuTreeNode[] = []

    // 初始化所有节点
    for (const item of list) {
      map.set(item.id, {
        ...item,
        children: [],
        collapsed: false,
        level: 0,
      })
    }

    // 构建父子关系
    for (const item of list) {
      const node = map.get(item.id)!
      if (item.parent_id && map.has(item.parent_id)) {
        const parent = map.get(item.parent_id)!
        node.level = parent.level + 1
        parent.children.push(node)
      } else {
        roots.push(node)
      }
    }

    return roots
  },

  /// 将树展平为可渲染的列表（考虑折叠状态）
  updateFlatList() {
    const flat: MenuTreeNode[] = []
    const traverse = (nodes: MenuTreeNode[]) => {
      for (const node of nodes) {
        flat.push(node)
        if (!node.collapsed && node.children.length > 0) {
          traverse(node.children)
        }
      }
    }
    traverse(this.data.tree)
    this.setData({ flatList: flat })
  },

  /// 切换折叠/展开
  onToggle(e: WechatMiniprogram.TouchEvent) {
    const id = e.currentTarget.dataset.id as number
    this.toggleNode(this.data.tree, id)
    this.updateFlatList()
  },

  /// 递归切换节点折叠状态
  toggleNode(nodes: MenuTreeNode[], id: number): boolean {
    for (const node of nodes) {
      if (node.id === id) {
        node.collapsed = !node.collapsed
        return true
      }
      if (this.toggleNode(node.children, id)) {
        return true
      }
    }
    return false
  },

  /// 点击菜单项查看详情
  async onMenuTap(e: WechatMiniprogram.TouchEvent) {
    const id = e.currentTarget.dataset.id as number
    try {
      const menu = await api.getMenu(id)
      this.setData({ selectedMenu: menu, showDetail: true })
    } catch (e: any) {
      wx.showToast({ title: e.message || '加载失败', icon: 'none' })
    }
  },

  /// 关闭详情弹窗
  onDetailClose() {
    this.setData({ showDetail: false })
  },

  stopPropagation() {},
})
