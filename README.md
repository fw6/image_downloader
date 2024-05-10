# imagekit SDK

使用Rust编写的一系列图片操作相关工具，如: 网络图片下载、图片格式转换、生成[Julia分形几何图片](https://en.wikipedia.org/wiki/Julia_set)、[网页地址快照](https://support.apple.com/en-au/guide/iphone/iph1fbef4daa/ios)、。它在Windows、macOS和Linux上都有一流的支持。

> [!WARNING]
> 目前是MVP版本，后续会支持更多的图片操作

## 特性

- 生成的二进制文件体积小
- 速度非常快🚀
- 跨平台支持
- 跨语言调用
  - 支持通过`uniffi`编译为多种平台的动态链接库(Android、iOS、Windows、macOS、Linux)
  - 支持通过protobuf定义接口，支持多种语言调用
  - 支持通过`tauri`提供GUI界面
  - 支持通过`wasm-pack`编译为WebAssembly

## 安装(CLI)

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

## 使用(CLI)

目前为止, 支持2个子命令：

1. `download`：下载网络图片(支持指定下载的图片格式)
2. `juliafatou`：生成分形几何图片


```shell
$ imagekit download --help
Download image from url

Usage: imagekit download [OPTIONS]

Options:
  -u, --url <URL>             The url of the image e.g. -u https://t7.baidu.com/it/u=1595072465,3644073269&fm=193&f=GIF
  -f, --formats <FORMATS>...  The image format to download, support multiple formats. Supported formats: png (png)、jpeg (jpg)、webp (webp)、gif (gif)、avif (avif)、bmp (bmp)、tiff (tiff)、ico (ico)
  -F, --filename <FILE NAME>  The filename of the output image [default: image]
  -h, --help                  Print help
```

```shell
$ imagekit juliafatou --help
Generate an Julia Fatou image set. example: cargo run --package imagekit_cli --bin imagekit juliafatou --blur 0.6 --scale 1 -c eleven --complex -0.4,0.6 -w 3

Usage: imagekit juliafatou [OPTIONS]

Options:
  -d, --dimensions <USIZExUSIZE>  Image dimensions [default: 1200x1200]
  -o, --output-file <FILE>        Output file [default: output.png]
  -s, --offset <F64:F64>          offset [default: 0.0:0.0]
  -x, --scale <F64>               scale factor [default: 3]
      --blur <F32>                blur (sigma) [default: 1]
  -w, --power <U8>                the 'x' in the equation z^x + c [default: 2]
  -f, --factor <F64>              multiplication factor of the secondary julia set (intensity) [default: -0.25]
  -c, --color-style <CM>          Select color gradient [default: greyscale] [possible values: bookworm, jellyfish, ten, eleven, mint, greyscale, christmas, chameleon, plasma, plasma2, config, random]
      --diverge <F64>             difference between the two rendered julia sets [default: 0.01]
  -p, --complex <F64,F64>         the 'c' in the equation z^x + c [default: -0.4,0.6]
  -i, --intensity <F64>           overall intensity multiplication factor [default: 3]
      --inverse                   invert color gradient
      --threads <USIZE>           number of threads (optional), defaults to 'available parallelism'
      --take-time                 measure render time
  -h, --help                      Print help
```

## Roadmap

- [x] 接入`uniffi`，支持跨平台调用
- [ ] 支持编译为WebAssembly
- [ ] 通过`napi-rs`支持Node.js调用
- [x] 支持鸿蒙系统调用([ohos-rs](https://github.com/ohos-rs))
- [x] 提供Open API支持及文档([openapi-generator](https://github.com/OpenAPITools/openapi-generator))
- [ ] 提供RPC服务: [prost](https://docs.rs/prost/0.12.3/prost/) + [tonic](https://docs.rs/tonic/0.10.2/tonic/)
- [x] 通过[`schemars`](https://docs.rs/schemars/latest/schemars/)为SDK生成`JSON Schema`文档
- [x] 为各类目标语言提供类型定义文件([`quicktype`](https://www.npmjs.com/package/quicktype))

## FAQ

### `Gitlab CI`发布报错了?

参考(#48)[https://github.com/axodotdev/cargo-dist/issues/48].

## 贡献

欢迎任何形式的贡献！如果你发现了bug或者有新的功能请求，请[创建一个issue](https://git.dev.sh.ctripcorp.com/feng.w/imagekit/-/issues/new)。

## 许可证

根据 MIT 或 [UNLICENSE](https://unlicense.org) 获得双重许可。

## 联系

如果你有任何问题或者建议，欢迎[联系我](https://c.ctrip.cn/s/e/TR036101)。
