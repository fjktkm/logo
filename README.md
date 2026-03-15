# logo

[![Build](https://github.com/fjktkm/logo/actions/workflows/build.yml/badge.svg)](https://github.com/fjktkm/logo/actions/workflows/build.yml)

Rust + Cairo でロゴアセットを生成するリポジトリです。
生成したアセットは GitHub Pages で公開しています。

## Preview

画像をクリックすると SVG を開きます。

| Theme | Light | Dark |
|:---:|:---:|:---:|
| Preview | [![Light logo](https://fjktkm.github.io/logo/logo_light_512.png)](https://fjktkm.github.io/logo/logo_light.svg) | [![Dark logo](https://fjktkm.github.io/logo/logo_dark_512.png)](https://fjktkm.github.io/logo/logo_dark.svg) |
| Assets | [SVG](https://fjktkm.github.io/logo/logo_light.svg)<br>[PDF](https://fjktkm.github.io/logo/logo_light.pdf)<br>PNG: [256px](https://fjktkm.github.io/logo/logo_light_256.png) / [512px](https://fjktkm.github.io/logo/logo_light_512.png) / [1024px](https://fjktkm.github.io/logo/logo_light_1024.png) | [SVG](https://fjktkm.github.io/logo/logo_dark.svg)<br>[PDF](https://fjktkm.github.io/logo/logo_dark.pdf)<br>PNG: [256px](https://fjktkm.github.io/logo/logo_dark_256.png) / [512px](https://fjktkm.github.io/logo/logo_dark_512.png) / [1024px](https://fjktkm.github.io/logo/logo_dark_1024.png) |

## Usage

このリポジトリは [`.devcontainer/devcontainer.json`](.devcontainer/devcontainer.json) を使う前提です。
Dev Container で開けば、Cairo を含む必要な依存関係は揃った状態で作業できます。

### Generate assets

全アセットを生成するには、次のコマンドを実行します。

```sh
cargo run --release
```

生成物は `dist/` に出力されます。

### Run tests

テストを実行するには、次のコマンドを使います。

```sh
cargo test
```
