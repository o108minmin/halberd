import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Button from '@mui/material/Button';
import { FormControl, Grid, InputLabel, MenuItem, Paper, Select, SelectChangeEvent, styled, Typography } from "@mui/material";
import { createTheme, ThemeProvider } from '@mui/material/styles';
import { open, save } from '@tauri-apps/api/dialog'
import { homeDir } from '@tauri-apps/api/path';

const home = await homeDir();

function App() {
  const [logMsg, setLogMsg] = useState("");
  const [tts, setTts] = useState("");
  const [input, setInput] = useState("");
  const [output, setOutput] = useState("");

  function openDir () {
    open({
      directory: true,
      defaultPath: home,
    }).then(files => setInput(convertFilePath(files)));
  }

  function saveDir () {
    const filters = [{
      name: "fcpxml",
      extensions: ['xml']
    },
    {
      name: "subtitle",
      extensions: ['srt']
    }];
    // Note: input前に開くと落ちることを防ぐ
    if (input == "") {
      save({
        defaultPath: "output",
        filters: filters,
      }).then(files => setOutput(convertFilePath(files)))
    } else {
      save({
        defaultPath: parseDirName(input),
        filters: filters,
      }).then(files => setOutput(convertFilePath(files)))
    }
}

  // inputDirArrayがstringの時だけその値を返す
  function convertFilePath (inputDirArray: string | string[] | null): string {
    if (inputDirArray == null) {
      return ""
    } else if (typeof inputDirArray === "string") {
      return inputDirArray
    } else {
      return ""
    }
  }

  // C:\mnt\dir から dirだけ取り出す
  function parseDirName (inputDir: string): string {
    console.info("parseDirName")
    console.info(inputDir)
    var split_unix = inputDir.split('/')
    split_unix.reverse()
    console.info(split_unix)
    var split_dos = split_unix[0].split('\\')
    split_dos.reverse()
    console.info(split_dos)
    return split_dos[0]
  }

  async function halberd_run() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    console.info("input: %s", input)
    console.info("output: %s", output)
    console.info("tts: %s", tts)
    if (input == "") {
      setLogMsg("inputが指定されていません")
    } else if (output == "") {
      setLogMsg("outputが指定されていません")
    } else if (tts == "") {
      setLogMsg("ttsが指定されていません")
    } else {
      setLogMsg(await invoke("halberd_run", { input, output, tts }));
    }
  }
  const handleTtsChange = (e: SelectChangeEvent) => {
    // https://mui.com/material-ui/react-select/
    console.info("change TTS");
    setTts(e.target.value as string);
  };
  // https://mui.com/material-ui/react-grid/
  const Item = styled(Paper)(({ theme }) => ({
    backgroundColor: theme.palette.mode === 'dark' ? '#1A2027' : '#fff',
    ...theme.typography.body2,
    padding: theme.spacing(1),
    textAlign: 'center',
    color: theme.palette.text.secondary,
  }));
  const theme = createTheme({
    palette: {
      mode: "dark"
    },
  });

  return (
    <div className="container">
      <ThemeProvider theme={theme}>
      <Typography align="center" variant="h2">halberd</Typography>
      <Grid container spacing={2}>
          <Grid item xs={10}>
            <Typography align="left" variant="h5">input: {input}</Typography>
          </Grid>
          <Grid item xs={2}>
            <Button 
              value={input}
              variant="outlined"
              fullWidth
              onClick={openDir}
              >開く</Button>
          </Grid>
          <Grid item xs={10}>
            <Typography align="left" variant="h5">output: {output}</Typography>
          </Grid>
          <Grid item xs={2}>
            <Button 
              variant="outlined"
              fullWidth
              defaultValue={"output"}
              onClick={saveDir}
              >開く</Button>
          </Grid>
          <Grid item xs={10}>
            <FormControl fullWidth>
              <InputLabel id="select-label-tts">TTS</InputLabel>
                <Select
                  labelId="select-label-tts"
                  id="halberd_run-input"
                  value={tts}
                  variant={"outlined"}
                  onChange={handleTtsChange}
                >
                  <MenuItem value={"voiceroid"}>VOICEROID(Shift_JIS)</MenuItem>
                  <MenuItem value={"coefont"}>CoeFont(utf-8)</MenuItem>
                  <MenuItem value={"voicevox"}>VOICEVOX(utf-8)</MenuItem>
                  <MenuItem value={"softalk"}>SofTalk(utf-16le)</MenuItem>
                  <MenuItem value={"talque"}>TALQu(Shift_JIS)</MenuItem>
                </Select>
              </FormControl>
          </Grid>
          <Grid item xs={10}>
              <Button 
                variant="outlined"
                onClick={() => halberd_run()}
              >実行</Button>
          </Grid>
        </Grid>
        </ThemeProvider>
      <p>{logMsg}</p>
    </div>
  );
}

export default App;
