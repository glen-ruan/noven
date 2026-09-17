# Noven

Noven（原 RustText）是基于 [Markion](https://github.com/willmove/markion) v0.3.10 制作的中文友好型发行版。底层保持 Rust + GPUI 原生实现，不使用 Electron、Tauri 或 WebView。

## 与上游相比的默认设置

- 产品和窗口名称为 Noven。
- 使用独立的 `~/.config/rusttext` 与 `~/.cache/rusttext`，不会覆盖 Markion 配置。
- 首次启动和重置首选项时使用简体中文。
- 默认使用内置的 **Noven Jade** 深色主题。
- 主题默认字体：
  - Markdown 源码：Noto Sans Mono CJK SC
  - 阅读与可视化编辑：Noto Serif CJK SC
  - 代码块：Noto Sans Mono CJK SC

这些只是默认值。打开 **首选项 → 外观** 后仍可修改主题、编辑字体、阅读字体、代码字体、字号和段落间距，也可以调节毛玻璃、界面透明度和环境粒子效果，或在原 RustText 兼容主题目录中添加 TOML 自定义主题。代码块配色会自动跟随当前应用主题。

## 主要功能

- 源码编辑、可视化编辑、分栏预览和阅读模式
- 多标签页、撤销/重做、自动保存与崩溃恢复
- 查找替换、文件树、文档大纲和字数统计
- 图片、链接、任务列表、表格、公式和 Mermaid
- HTML、PDF、DOCX、LaTeX、PNG/JPEG 等导出路径
- 主题、字体、字号、段落间距、毛玻璃、透明度、环境粒子与快捷键设置

## 构建

Ubuntu / Debian 先安装构建依赖：

```bash
sudo apt install cargo rustc build-essential pkg-config libgtk-4-dev libxkbcommon-x11-dev
```

然后在本仓库根目录执行：

```bash
cargo test --workspace
cargo build --release
```

直接运行 `target/release/markion`。内部二进制和 crate 名暂时保留为 `markion`，以避免破坏上游工作区依赖；打包后的产品名称为 Noven，安装后的命令仍为 `rusttext`，以便覆盖升级旧版本。

安装本仓库生成的 Debian 包：

```bash
sudo apt install ./rusttext_0.3.10-5_amd64.deb
rusttext
```

Linux 安装包默认优先使用 X11 窗口后端，让桌面环境的窗口主题接管标题栏、圆角、拖动和窗口按钮。需要原生 Wayland 时可运行 `RUSTTEXT_USE_WAYLAND=1 rusttext`。

文档命名和编辑方式与常见编辑器一致：按 `Ctrl+N` 新建，直接输入内容，按 `Ctrl+S` 选择名称和保存位置；已有 Markdown 或纯文本文件可用 `Ctrl+O` 打开。主题、字体和字号位于 **首选项 → 外观**。

## 许可证与归属

Noven 保留 Markion 的 MIT 许可证、原作者归属和所有第三方声明。完整信息见 `LICENSE` 与 `THIRD_PARTY_NOTICES.md`。
