// app.ts
import { isLoggedIn } from './utils/auth'
import { applyTheme } from './utils/theme'

App<IAppOption>({
  globalData: {},
  onLaunch() {
    // 应用主题
    applyTheme()

    // 检查登录状态，未登录则跳转登录页
    if (!isLoggedIn()) {
      wx.reLaunch({ url: '/pages/login/login' })
    }
  },
})
