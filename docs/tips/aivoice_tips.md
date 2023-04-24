# halberd with VOICEROID

halberdをA.I.VOICEで利用するための設定やtipsです

## テキスト書き出し設定

音声保存時下記の設定にする

- ファイル分割 -> 「1文毎に区切って複数のファイルに書き出す」
- ファイル形式 -> WAVE 44100Hz PCM ファイルヘッダー有り
- テキストファイル -> テキストファイルを音声ファイルと一緒に保存する
    - 文字コード -> utf-8

## 複数話者対応

A.I.VOICEではリスト書き出し時に `話者＞わあ` みたいな文章としてtxtファイルに保存される  
この時、 `話者＞` を字幕生成から取り除くのは優先度低めだけど検討中

https://github.com/o108minmin/halberd/issues/28