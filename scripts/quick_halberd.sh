#!/bin/sh
echo "TTSの種類を入力してください(1: voiceroid, 2: coefont, 3: voicevox, 4: softalk, 5: talqu)"
read tts;
if [ $tts = "1" ]
then
    tts="voiceroid"
fi
if [ $tts = "2" ]
then
    tts="coefont"
fi
if [ $tts = "3" ]
then
    tts="voicevox"
fi
if [ $tts = "4" ]
then
    tts="softalk"
fi
if [ $tts = "5" ]
then
    tts="talqu"
fi

echo "ディレクトリのパスを入力してください(1: 今のディレクトリで実行, それ以外: 入力)"
read input
if [ $input = "1" ]
then
    input="./"
fi

echo "フォーマットを指定してください(1: srt, 2: xml)"
read format
if [ $format = "1" ]
then
    format="srt"
fi
if [ $format = "2" ]
then
    format="xml"
fi
echo "ファイル名を指定してください(1: 標準出力 それ以外: そのファイル名で出力)"
read file_name
if [ $file_name = "1" ]
then
    file_name="stdout"
fi
./halberd $tts $input -f $format -u -o $file_name
