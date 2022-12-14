# Contribution Guide

このリポジトリへのコントリビューティングガイドです

## 最初に

- このプロジェクトはo108minminが個人でやっているものなので、対応が遅れる場合があります
- なるべく下記を重視します
    - マルチプラットフォームで動作する
        - windows, mac, linux
    - 特定の動画編集ソフトへの依存を避ける
        - 特定の動画ソフトだけで起きている不具合向けの修正はあり
    - いつ手放しても問題ないツールにする
- 長期的に反応がない場合、issueやprをcloseすることがあります
    - 1か月程度は待つ予定です。必要であれば再作成、reopenしてください

## Issues

issueはここから作成してください: https://github.com/o108minmin/halberd/issues/new/choose

- 対応して欲しいTTSがある場合、 `new TTS support` にて作成をお願いします。

## Pull Request

Pull Requestはいつでも歓迎です。  
ただし、新機能の追加やリファクタリングが大きそうな場合、issue作成をお願いします。

- ブランチの命名規則
    - `add-` 機能追加
    - `fix-` バグ修正
    - `change-` 仕様変更
    - `release-` リリース直前
- commitメッセージ
    - `add-` 機能追加
    - `fix-` バグ修正
    - `change-` 仕様変更

squash mergeしても問題ないように、prの先頭のcommitについてはpr全体の修正を表すようなメッセージだと嬉しいです。  
改行に関しては特に規定はありません。

## テスト

`cargo test`  
動かない場合は `.github/workflows/release.yml` を参考にしてみてください

## lint

`cargo clippy`, `cargo check` の二つは通してください

## ディレクトリ構造

```
.
├── Cargo.toml
├── docs # ロゴや設定など
│   └── tips
├── halberd_cli # cliゲートウェイ(clap)
│   ├── Cargo.toml
│   ├── src
│   └── tests # 結合テスト
├── halberd_core # 字幕生成ロジックなど
│   ├── Cargo.toml
│   └── src
├── halberd_gui # guiゲートウェイ(Tauri)
│   ├── src
│   └── src-tauri
└── scripts # cliをさらに簡単に使うためのスクリプト
```

## リリース

- バージョン情報などを更新したpr作成
    - 参考commit https://github.com/o108minmin/halberd/commit/9cc67c2163283e5a82c8cb560cfaf1e715669ba7
- `git tag v2.x.x`
- `git push origin v2.x.x`

あとはCIがなんとかします
