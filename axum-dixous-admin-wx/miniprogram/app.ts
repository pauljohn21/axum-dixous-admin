// app.ts
import { isLoggedIn } from './utils/auth'
import { applyTheme } from './utils/theme'

App<IAppOption>({
  globalData: {
    theme: 'light' as 'light' | 'dark',
  },
  onLaunch() {
    // 应用主题
    const theme = applyTheme()
    this.globalData.theme = theme

    // 检查登录状态，未登录则跳转登录页
    if (!isLoggedIn()) {
      wx.reLaunch({ url: '/pages/login/login' })
    }
  },
})
