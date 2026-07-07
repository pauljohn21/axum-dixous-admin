// app.ts
import { isLoggedIn } from './utils/auth'

App<IAppOption>({
  globalData: {},
  onLaunch() {
    // 检查登录状态，未登录则跳转登录页
    if (!isLoggedIn()) {
      wx.reLaunch({ url: '/pages/login/login' })
    }
  },
})
