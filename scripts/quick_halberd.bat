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
    set f="srt"
)
if %f_temp% == 2 (
    set f="xml"
)
set /p file_name_temp=�t�@�C�������w�肵�Ă�������(1: �W���o�� ����ȊO: ���̃t�@�C�����ŏo��)
set file_name=%file_name_temp%
if %file_name_temp% == 1 (
    set file_name="stdout"
)
.\halberd.exe %tts% .\ -f %f% -o %file_name% -u
pause
