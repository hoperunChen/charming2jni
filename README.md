# 概要
该项目主要依赖于charming项目，将charming部分api封装成jni方便javaer使用。

# 关于libc版本
项目使用github actions构建的，动态链接库依赖于libc版本的影响，可能不通用。如果你构建出来了某个平台某个libc版本的链接库欢迎提交pr 方便其他人使用。

动态链接库存放在 artifact 目录中


# 打包方式:

## 1 确保安装了rust环境

**MacOs**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
按提示选择 1（默认安装），安装完成后：
```bash
source $HOME/.cargo/env
```
验证:
```bash
cargo --version
```

**Windows**

1.	下载并运行官方安装程序：https://www.rust-lang.org/tools/install
2.	安装完后重启终端（或 CMD / PowerShell）
3.	验证安装：
```bash
cargo --version
```

## 2 进入项目根目录,打包

```bash
cd <project_root>
cargo build --release -v
```