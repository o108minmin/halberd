# halberd

halberdはTTS(Text To Speech)ソフトウェアのファイルから字幕を生成するCLIツールです。

<!-- # Badges -->

[![Github issues](https://img.shields.io/github/issues/o108minmin/halberd)](https://github.com/o108minmin/halberd/issues)
[![Github forks](https://img.shields.io/github/forks/o108minmin/halberd)](https://github.com/o108minmin/halberd/network/members)
[![Github stars](https://img.shields.io/github/stars/o108minmin/halberd)](https://github.com/o108minmin/halberd/stargazers)
[![Github top language](https://img.shields.io/github/languages/top/o108minmin/halberd)](https://github.com/o108minmin/halberd/)
[![Github license](https://img.shields.io/github/license/o108minmin/halberd)](https://github.com/o108minmin/halberd/)

## 対応TTS

- [VOICEROID](https://www.ah-soft.com/voiceroid/)
- [CoeFont](https://coefont.cloud/)

## 動作例

WSL2(Ubuntu 20.04)
### VOICEROID

```bash
$ ls tests/data/tts/vocieroid/
01.txt  01.wav  02.txt  02.wav
$ iconv -f SJIS tests/data/tts/vocieroid/01.txt
テスト
$ iconv -f SJIS tests/data/tts/vocieroid/02.txt
こんにちは、だみーです。17歳です

$ halberd voiceroid tests/data/tts/vocieroid/
1
00:00:00,000 --> 00:00:01,269
テスト

2
00:00:01,270 --> 00:00:06,295
こんにちは、だみーです。17歳です
```

```bash
# Generate srt file

$ halberd voiceroid tests/data/tts/vocieroid/ > voiceroid.srt

$ cat voiceroid.srt
1
00:00:00,000 --> 00:00:01,269
テスト

2
00:00:01,270 --> 00:00:06,295
こんにちは、だみーです。17歳です
```

### coefont studio

```bash
$ ls coefont/
20210815-0108_Dummy_わたしわだ.txt  20210815-0108_Dummy_わたしわだ.wav  20210815-0118_Dummy_こんにちわ.txt  20210815-0118_Dummy_こんにちわ.wav
$ cat coefont/20210815-0108_Allial_わたしわあ.txt
私はだみーです。よろしくお願いします。
$ cat tests/data/tts/coefont/20210815-0118_Dummy_こんにちは.txt
こんにちは
$ halberd coefont tests/data/tts/coefont/
1
00:00:00,000 --> 00:00:03,499
私はだみーです。よろしくお願いします。

2
00:00:03,500 --> 00:00:04,475
こんにちは
```

## インストール方法

### クイックスタート

[releaseページ](https://github.com/o108minmin/halberd/releases)からダウンロードし、下記のように同名のwavファイルとtxtファイルを配置したら `quick_halberd` を実行してください

```bash
$ ls coefont/
20210815-0108_Dummy_わたしわだ.txt  20210815-0108_Dummy_わたしわだ.wav  20210815-0118_Dummy_こんにちわ.txt  20210815-0118_Dummy_こんにちわ.wav
```

### 上級インストール

[releaseページ](https://github.com/o108minmin/halberd/releases)からダウンロードし、 `halberd` を 任意の場所( `/usr/local/bin` など)に配置後、パスを通してください

# Contributors

- [o108minmin](https://github.com/o108minmin)
