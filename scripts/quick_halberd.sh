#!/bin/sh
echo 'TTSの種類を入力してください(1: voiceroid, 2: coefont)'
read tts_temp;
tts=$tts_temp;
if [ $tts_temp = "1" ]
then
    tts="voiceroid"
fi
if [ $tts_temp = "2" ]
then
    tts="coefont"
fi
echo 'フォーマットを指定してください(1: srt, 2: xml)'
read f_temp
f=$f_temp
if [ $f_temp = "1" ]
then
    f="srt"
fi
if [ $f_temp = "2" ]
then
    f="xml"
fi
./halberd $tts ./ -f $f -u
