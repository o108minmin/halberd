.\halberd.exe --version
set /p tts_temp=TTS�̎�ނ���͂��Ă�������(1: voiceroid, 2: coefont)
set tts=%tts_temp%
if %tts_temp% == 1 (
    set tts="voiceroid" 
)
if %tts_temp% == 2 (
    set tts="coefont" 
)
set /p f_temp=�t�H�[�}�b�g���w�肵�Ă�������(1: srt, 2: xml)
set f=%f_temp%
if %f_temp% == 1 (
    set tts="voiceroid" 
)
if %f_temp% == 2 (
    set tts="coefont" 
)
.\halberd.exe %tts% .\ -f %f% -u
pause
