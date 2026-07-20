import { THEME_KEY } from './config'

/// 主题类型
export type Theme = 'light' | 'dark'

/// 获取当前主题
export function getTheme(): Theme {
  return (wx.getStorageSync(THEME_KEY) as Theme) || 'light'
}

/// 保存主题设置
export function setTheme(theme: Theme): void {
  wx.setStorageSync(THEME_KEY, theme)
}

/// 切换主题
export function toggleTheme(): Theme {
  const current = getTheme()
  const next: Theme = current === 'light' ? 'dark' : 'light'
  setTheme(next)
  return next
}

/// 应用主题到导航栏
export function applyTheme(theme?: Theme): Theme {
  const t = theme || getTheme()
  const navBg = t === 'dark' ? '#16213e' : '#409eff'
  const navText = t === 'dark' ? '#e0e0e0' : '#ffffff'
  wx.setNavigationBarColor({
    frontColor: navText,
    backgroundColor: navBg,
  })
  return t
}

/// 获取页面根 class（用于 WXML 中设置 dark class）
export function pageClass(): string {
  return getTheme() === 'dark' ? 'dark' : ''
}
