.\halberd.exe --version
set /p tts_temp=TTSの種類を入力してください(1: voiceroid, 2: coefont)
set tts=%tts_temp%
if %tts_temp% == 1 (
    set tts="voiceroid"
)
if %tts_temp% == 2 (
    set tts="coefont"
)
set /p f_temp=フォーマットを指定してください(1: srt, 2: xml)
set f=%f_temp%
if %f_temp% == 1 (
    set f="srt"
)
if %f_temp% == 2 (
    set f="xml"
)
set /p file_name_temp=ファイル名を指定してください(1: 標準出力 それ以外: そのファイル名で出力)
set file_name=%file_name_temp%
if %file_name_temp% == 1 (
    set file_name="stdout"
)
.\halberd.exe %tts% .\ -f %f% -o %file_name% -u
pause
