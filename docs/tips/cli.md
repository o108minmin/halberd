## cli動作例

WSL2(Ubuntu 20.04)

### srtファイル生成

```bash
$ halberd voiceroid tests/data/tts/vocieroid/ -f srt -o voiceroid.srt

$ cat voiceroid.srt
1
00:00:00,000 --> 00:00:01,269
テスト

2
00:00:01,270 --> 00:00:06,295
こんにちは、だみーです。17歳です
```

### xmlファイル生成

```bash
$ halberd voiceroid tests/data/tts/vocieroid/ -f xml -o voiceroid.xml
$ cat voiceroid.xml
```

```xml
<?xml version="1.0" encoding="utf-8"?>
<fcpxml version="1.8">
  <resources>
    <format id="r1" name="FFVideoFormat1080p2398" frameDuration="1001/24000s" width="1920" height="1080" colorSpace="1-1-1 (Rec. 709)" />
    <effect id="r2" name="text" />
  </resources>
  <library>
    <event name="event">
      <project name="event">
        <sequence format="r1">
          <spine>
            <text-style-def id="ts1">
              <text-style font="Helvetica" fontSize="72" fontFace="Regular" fontColor="1 0.999974 0.999991 1" alignment="center" />
            </text-style-def>
            <title ref="r2" offset="0/1000s" name="1" start="0/1000s" duration="1269/1000s">
              <text>
                <text-style ref="ts1">テスト</text-style>
              </text>
            </title>
            <title ref="r2" offset="1270/1000s" name="2" start="1270/1000s" duration="5024/1000s">
              <text>
                <text-style ref="ts1">こんにちは、だみーです。17歳です</text-style>
              </text>
            </title>
          </spine>
        </sequence>
      </project>
    </event>
  </library>
</fcpxml>
```
