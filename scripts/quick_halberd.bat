.\halberd.exe --version
set /p tts=TTS�̎�ނ���͂��Ă�������(1: voiceroid, 2: coefont, 3: voicevox, 4: softalk, 5: talqu)
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
if %tts% == 5 (
    set tts="talqu"
)

set input="%~f1"
if %input%=="" (
    set /p input=�f�B���N�g���̃p�X����͂��Ă�������(1: ���̃f�B���N�g���Ŏ��s, ����ȊO: ����)
)
if %input% == 1 (
    set input=".\"
)

set /p format=�t�H�[�}�b�g���w�肵�Ă�������(1: srt, 2: xml)
if %format% == 1 (
    set format="srt"
)
if %format% == 2 (
    set format="xml"
)

set /p file_name=�t�@�C�������w�肵�Ă�������(1: �t�H���_�����g�p 2: �W���o�� ����ȊO: ���̃t�@�C�����ŏo��)
if %file_name% == 1 (
    set file_name="dirname"
)
if %file_name% == 2 (
    set file_name="stdout"
)
.\halberd.exe %tts% %input% -f %format% -o %file_name% -t
pause
