# RNES Web Frontend - React Pixel Game Style

这是一个使用React和像素游戏风格UI的NES模拟器web前端。

## 特性

### 🎮 像素游戏风格界面
- 使用Press Start 2P和VT323字体营造复古游戏氛围
- 像素化的边框、阴影和动画效果
- CRT扫描线效果和绿色发光效果
- 复古色彩调色板（绿色、青色、紫色等）

### ⚡ React + Framer Motion
- 现代化的React组件架构
- 流畅的动画和过渡效果
- 响应式设计，支持移动设备

### 📁 文件拖放
- 使用react-dropzone实现流畅的文件拖放体验
- 支持.nes文件格式
- 拖放状态的可视化反馈

### 🎯 交互式控制
- 像素风格的按钮设计
- 悬停和点击动画效果
- 状态驱动的UI更新

## 技术栈

- **React 18** - 现代化的UI框架
- **Framer Motion** - 动画库
- **React Dropzone** - 文件拖放功能
- **Vite** - 快速构建工具
- **CSS3** - 像素游戏风格样式

## 安装和运行

### 安装依赖
```bash
npm install
```

### 开发模式
```bash
npm run dev
```
访问 http://localhost:3000

### 构建生产版本
```bash
npm run build
```

### 预览生产版本
```bash
npm run preview
```

## 项目结构

```
src/
├── App.jsx          # 主应用组件
├── App.css          # 像素游戏风格样式
├── main.jsx         # React入口文件
└── style.css        # 基础样式（已弃用）

index.html           # HTML模板
vite.config.js       # Vite配置
package.json         # 项目依赖
```

## 设计特色

### 像素艺术风格
- 使用CSS变量定义像素大小和间距
- 像素化的边框和阴影效果
- 复古游戏色彩调色板

### 动画效果
- 页面加载动画
- 按钮悬停和点击动画
- 状态变化过渡效果

### 响应式设计
- 支持桌面、平板和移动设备
- 自适应布局和字体大小
- 触摸友好的交互设计

## 未来计划

- [ ] 集成WASM模拟器核心
- [ ] 添加游戏手柄支持
- [ ] 实现存档/读档功能
- [ ] 添加更多像素游戏风格的主题
- [ ] 支持更多ROM格式

## 贡献

欢迎提交Issue和Pull Request来改进这个项目！

## 许可证

MIT License
