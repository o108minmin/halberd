#!/bin/sh
echo "TTSの種類を入力してください(1: voiceroid, 2: coefont)"
read tts;
if [ $tts = "1" ]
then
    tts="voiceroid"
fi
if [ $tts = "2" ]
then
    tts="coefont"
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
