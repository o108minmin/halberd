# halberd

halberdはTTS(Text To Speech)ソフトウェアのファイルから字幕(srt, xml)を生成するCLIツールです

<p align="center">
   <img src="./docs/logo_2048.png" alt="logo" width="300px">
</p>

<!-- # Badges -->

[![Github issues](https://img.shields.io/github/issues/o108minmin/halberd)](https://github.com/o108minmin/halberd/issues)
[![Github forks](https://img.shields.io/github/forks/o108minmin/halberd)](https://github.com/o108minmin/halberd/network/members)
[![Github stars](https://img.shields.io/github/stars/o108minmin/halberd)](https://github.com/o108minmin/halberd/stargazers)
[![Github top language](https://img.shields.io/github/languages/top/o108minmin/halberd)](https://github.com/o108minmin/halberd/)
[![Github license](https://img.shields.io/github/license/o108minmin/halberd)](https://github.com/o108minmin/halberd/)

## 対応TTS

- [VOICEROID](https://www.ah-soft.com/voiceroid/)
- [CoeFont](https://coefont.cloud/)
- [VOICEVOX](https://voicevox.hiroshiba.jp/)
- [SofTalk](https://w.atwiki.jp/softalk/)
- [TALQu](https://booth.pm/ja/items/2755336ls)

halberdは上記TTSの権利を有する方々とは関係ない非公式のものになります

推奨される設定などは[tips](https://github.com/o108minmin/halberd/tree/main/docs/tips)に書いてあります

## 使い方

coefontの例

```bash
$ ls coefont/
20210815-0108_Dummy_わたしわだ.txt  20210815-0108_Dummy_わたしわだ.wav  20210815-0118_Dummy_こんにちわ.txt  20210815-0118_Dummy_こんにちわ.wav
```

1. `input` に `txt` `wav` ファイルが配置されたフォルダを指定する
1. `output` に保存するファイル名、拡張子を指定する
1. TTSの種類を選択する
1. 実行する
1. 生成したxml, srtなどのファイルを動画編集ソフトに読み込む

cli版は[こちら](https://github.com/o108minmin/halberd/tree/main/docs/tips/cli.md)に書いてあります  

## インストール方法

[release](https://github.com/o108minmin/halberd/releases)から利用したいOSの `halberd-gui_` をダウンロードし、インストーラーを実行してください

### ライセンス クレジット表記に関して

ソースコードとしての利用はLGPL v3になります。LICENSEを参照してください。
下記を明示します。

1. このソフトウェアを利用して作成した成果物(字幕、字幕ファイル)にLGPLは適用されません。
2. このソフトウェアを利用して作成した成果物(字幕、字幕ファイル)を利用した動画にもクレジット表記は不要です。

Copyright (c) 2021 o108minmin

# Contributors

- [o108minmin](https://github.com/o108minmin) / selfmiso
