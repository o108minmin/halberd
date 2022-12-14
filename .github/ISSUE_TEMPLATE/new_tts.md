---
name: new TTS support
about: TTSを追加してほしい時
title: '[TTS]'
labels: enhancement
assignees: o108minmin

---

**課題 or やりたいこと**

{TTS}を追加したい

**やること**

cliまでの対応はこのprを参考にする https://github.com/o108minmin/halberd/pull/36

- cli: 下記を含んだpr作成
    - [ ] cli実装
    - [ ] cliテスト追加
- gui: 下記を含んだpr作成
    - [ ] gui実装
- ドキュメント: (できれば)TTSの設定や注意点をまとめた資料の作成 [参考pr](https://github.com/o108minmin/halberd/pull/29)
    - [ ] ドキュメント整備

**実装に必要な情報**

下記が分かっているなら追記

* 既存のTTSを同じ設定で出力できるか
* (できない場合)txtファイルのエンコーディング(utf-8, Shift-JIS)
* (txtファイルが出力されない場合)どうすればセリフを読み込みできるか
* wavファイルとtxtファイルが1:1で出力されるか
    * 下記のように出力されているかどうか(拡張子以外が同じ名前のファイル)

```
01.wav
02.wav
01.txt
02.txt
```
