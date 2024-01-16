# imagekit

使用Rust编写的命令行工具，用于下载网络图片并生成多种图片格式。它在Windows、macOS和Linux上都有一流的支持。

## 功能

- 下载网络图片
- 生成多种图片格式
- 支持Windows、macOS和Linux

## 安装

1. 下载最新的[发布版本](https://github.com/fw6/imagekit/releases)
2. 解压缩文件

### Windows

在Windows上，你可以通过以下步骤运行：
1. 打开`cmd`或者`PowerShell`, 进入解压后的文件夹
2. 运行`.\imagekit.exe run --help`查看帮助信息

### macOS

在macOS上，你可以通过以下步骤运行：
1. 打开`Terminal.app`, 进入解压后的文件夹
2. 运行`./imagekit run --help`查看帮助信息


### Linux

在Linux上，你可以通过以下步骤运行：
1. 通过`cd`命令进入解压后的文件夹
2. 运行`./imagekit run --help`查看帮助信息

## 使用

你可以通过以下命令下载并转换图片：
```shell
$ imagekit run --url https://example.com/image.jpg --formats png jpeg webp gif --filename image
```

由于业务需要, 增加了将`携程创作中心2023年度榜单`的上榜用户头像下载到本地的功能, 通过以下命令下载并转换图片格式：
```shell
$ imagekit rank2023 --env dev -o ./output -f png tiff ico gif avif bmp
```

## ROADMAP

- [x] 支持下载网络图片
- [x] 支持下载携程创作中心2023年度榜单上榜用户头像
- [ ] 支持通过本地图片生成多种图片格式
- [ ] 支持编译为WebAssembly
- [ ] 通过`uniffi`编译为多种平台的动态链接库(Android、iOS、Windows、macOS、Linux)
- [ ] 通过protobuf定义接口，支持多种语言调用
- [ ] 提供GUI界面(通过`tauri`)

## FAQ

### `Gitlab CI`发布报错了?

参考(#48)[https://github.com/axodotdev/cargo-dist/issues/48].

## 贡献

欢迎任何形式的贡献！如果你发现了bug或者有新的功能请求，请[创建一个issue](https://git.dev.sh.ctripcorp.com/feng.w/imagekit/-/issues/new)。

## 许可证

根据 MIT 或 [UNLICENSE](https://unlicense.org) 获得双重许可。

## 联系

如果你有任何问题或者建议，欢迎[联系我](https://c.ctrip.cn/s/e/TR036101)。
