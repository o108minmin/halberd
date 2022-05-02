.\halberd.exe --version
set /p tts=TTSの種類を入力してください(1: voiceroid, 2: coefont, 3: voicevox, 4: softalk)
set tts=%tts%
if %tts% == 1 (
    set tts="voiceroid"
)
if %tts% == 2 (
    set tts="coefont"
)
if %tts% == 3 (
    set tts="voicevox"
)
if %tts% == 4 (
    set tts="softalk"
)

set input="%~f1"
if %input%=="" (
    set /p input=ディレクトリのパスを入力してください(1: 今のディレクトリで実行, それ以外: 入力)
)
if %input% == 1 (
    set input=".\"
)

set /p format=フォーマットを指定してください(1: srt, 2: xml)
if %format% == 1 (
    set format="srt"
)
if %format% == 2 (
    set format="xml"
)

set /p file_name=ファイル名を指定してください(1: 標準出力 それ以外: そのファイル名で出力)
if %file_name% == 1 (
    set file_name="stdout"
)
.\halberd.exe %tts% %input% -f %format% -o %file_name% -u
pause
