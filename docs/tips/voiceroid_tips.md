# halberd with VOICEROID

halberdをVOICEROID2で利用するための設定やtipsです

## テキスト書き出し設定

- ツール ➡ オプション ➡ 音声保存 ➡ ファイル分割 ➡ 「1文毎に区切って複数のファイルに書き出す」に✅
- ツール ➡ オプション ➡ 音声保存 ➡ その他 ➡ 「テキストファイルを音声ファイルと一緒に保存する」に✅

## 複数話者対応

VOICEROID2では複数話者書き出し時に `話者＞わあ` みたいな文章としてtxtファイルに保存される  
この時、 `話者＞` を字幕生成から取り除くのは優先度低めだけど検討中

https://github.com/o108minmin/halberd/issues/28

## VOICEROID+, VOICEROID+exで動く？

未検証。たぶん動く？  
(txtファイルがShift_JISならたぶん動く)

VOICEROID2にインポートしたものなら問題なく動くはず
