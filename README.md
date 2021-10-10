# halberd

halberd is Software Talk Subtitle Generator.

## poc

### VOICEROID

```bash
$ ls voiceroid/
01.txt  01.wav  02.txt  02.wav
$ iconv -f SJIS voiceroid/01.txt
テスト
$ iconv -f SJIS voiceroid/02.txt
こんにちは、桜乃そらです。17歳です

$ cargo run voiceroid voiceroid/
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/halberd voiceroid voiceroid/`
1
00:00:00,000 --> 00:00:01,269
テスト

2
00:00:01,269 --> 00:00:06,294
こんにちは、桜乃そらです。17歳です
```

```bash
# Generate srt file

$ cargo run voiceroid voiceroid/ > voiceroid.srt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/halberd voiceroid voiceroid/`
$ cat voiceroid.srt
1
00:00:00,000 --> 00:00:01,269
テスト

2
00:00:01,269 --> 00:00:06,294
こんにちは、桜乃そらです。17歳です
```

### coefont studio

```bash
$ ls coefontstudio/
20210815-0108_Allial_わたしわあ.txt  20210815-0108_Allial_わたしわあ.wav  20210815-0118_Allial_こんにちわ.txt  20210815-0118_Allial_こんにちわ.wav
$ cat coefontstudio/20210815-0108_Allial_わたしわあ.txt
私はアリアルです。よろしくお願いします。
$ cat coefontstudio/20210815-0118_Allial_こんにちわ.txt
こんにちわ
$ cargo run coefontstudio coefontstudio/
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/halberd coefontstudio coefontstudio/`
1
00:00:00,000 --> 00:00:00,975
こんにちわ

2
00:00:00,975 --> 00:00:04,474
私はアリアルです。よろしくお願いします。
```

```bash
# Generate srt file

$ cargo run coefontstudio coefontstudio/ > coefontstudio.srt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/halberd coefontstudio coefontstudio/`
$ cat coefontstudio.srt
1
00:00:00,000 --> 00:00:00,975
こんにちわ

2
00:00:00,975 --> 00:00:04,474
私はアリアルです。よろしくお願いします。
```

## TODO for 1.0.0

- [x] Use cli library([clap](https://github.com/clap-rs/clap))
- [x] Use logger
- [x] Refactor Error handling
- [x] Refactor names
- [x] Write unit tests
- [ ] Write integration tests
- [ ] Write documents
