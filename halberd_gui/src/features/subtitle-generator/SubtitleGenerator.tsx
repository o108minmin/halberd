import { useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { homeDir } from "@tauri-apps/api/path";
import { open, save } from "@tauri-apps/plugin-dialog";
import ArrowForwardRoundedIcon from "@mui/icons-material/ArrowForwardRounded";
import AudioFileRoundedIcon from "@mui/icons-material/AudioFileRounded";
import AutoAwesomeRoundedIcon from "@mui/icons-material/AutoAwesomeRounded";
import BoltRoundedIcon from "@mui/icons-material/BoltRounded";
import DescriptionRoundedIcon from "@mui/icons-material/DescriptionRounded";
import PlayArrowRoundedIcon from "@mui/icons-material/PlayArrowRounded";
import TuneRoundedIcon from "@mui/icons-material/TuneRounded";
import {
  Box,
  Button,
  Chip,
  MenuItem,
  Select,
  Stack,
  ToggleButton,
  ToggleButtonGroup,
  Typography,
} from "@mui/material";
import { PathSelector } from "../../components/PathSelector";
import { StatusBanner } from "../../components/StatusBanner";
import { ttsProfiles } from "../../data/ttsProfiles";
import type {
  GenerationStatus,
  OutputFormat,
  TtsId,
} from "../../types";

const initialStatus: GenerationStatus = {
  kind: "idle",
  message: "TTS、入力フォルダ、出力先を順に指定してください。",
};

function directoryName(path: string) {
  const normalized = path.replace(/[\\/]+$/, "");
  return normalized.split(/[\\/]/).pop() || "subtitle";
}

function errorMessage(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}

export function SubtitleGenerator() {
  const [tts, setTts] = useState<TtsId | "">("");
  const [input, setInput] = useState("");
  const [output, setOutput] = useState("");
  const [format, setFormat] = useState<OutputFormat>("srt");
  const [status, setStatus] = useState<GenerationStatus>(initialStatus);

  const selectedProfile = useMemo(
    () => ttsProfiles.find((profile) => profile.id === tts),
    [tts],
  );
  const isRunning = status.kind === "running";
  const canGenerate = Boolean(tts && input && output) && !isRunning;

  async function selectInputDirectory() {
    try {
      const defaultPath = await homeDir();
      const selected = await open({ directory: true, defaultPath });
      if (typeof selected === "string") {
        setInput(selected);
        setOutput("");
        setStatus(initialStatus);
      }
    } catch (error) {
      setStatus({ kind: "error", message: `入力先を開けませんでした: ${errorMessage(error)}` });
    }
  }

  async function selectOutputFile() {
    try {
      const baseName = input ? directoryName(input) : "subtitle";
      const selected = await save({
        defaultPath: `${baseName}.${format}`,
        filters: [
          {
            name: format === "srt" ? "SubRip subtitle" : "Final Cut Pro XML",
            extensions: [format],
          },
        ],
      });
      if (selected) {
        setOutput(selected);
        setStatus(initialStatus);
      }
    } catch (error) {
      setStatus({ kind: "error", message: `出力先を開けませんでした: ${errorMessage(error)}` });
    }
  }

  function changeFormat(nextFormat: OutputFormat | null) {
    if (!nextFormat) return;
    setFormat(nextFormat);
    setOutput("");
    setStatus(initialStatus);
  }

  async function generateSubtitle() {
    if (!tts || !input || !output) {
      setStatus({ kind: "error", message: "未入力の項目があります。すべての項目を指定してください。" });
      return;
    }

    setStatus({ kind: "running", message: "音声の長さを解析し、字幕データを書き出しています…" });
    try {
      const result = await invoke<string>("halberd_run", { input, output, tts });
      setStatus({ kind: "success", message: result });
    } catch (error) {
      setStatus({ kind: "error", message: errorMessage(error) });
    }
  }

  return (
    <Box className="workspace">
      <Stack className="workspace-header" direction="row" justifyContent="space-between">
        <Box>
          <Stack direction="row" spacing={1} alignItems="center">
            <Chip className="workspace-header__chip" icon={<BoltRoundedIcon />} label="SUBTITLE GENERATOR" />
            <Typography className="workspace-header__version">Tauri desktop</Typography>
          </Stack>
          <Typography component="h1" className="workspace-header__title">
            音声から、編集しやすい字幕へ。
          </Typography>
          <Typography className="workspace-header__lead">
            TTSが出力したテキストと音声を読み込み、タイミングを揃えた字幕ファイルを生成します。
          </Typography>
        </Box>
        <Box className="workspace-header__decoration" aria-hidden="true">
          <AutoAwesomeRoundedIcon />
        </Box>
      </Stack>

      <Box className="generator-grid">
        <Box className="generator-panel">
          <Box className="step-section">
            <Stack className="step-heading" direction="row" spacing={1.5} alignItems="center">
              <Box className="step-heading__number">01</Box>
              <Box>
                <Typography className="step-heading__title">TTSプロファイル</Typography>
                <Typography className="step-heading__description">音声を作成したソフトウェアを選択</Typography>
              </Box>
            </Stack>
            <Select
              className="tts-select"
              displayEmpty
              fullWidth
              onChange={(event) => {
                setTts(event.target.value as TtsId);
                setStatus(initialStatus);
              }}
              renderValue={(value) => {
                const profile = ttsProfiles.find((item) => item.id === value);
                return profile ? (
                  <Stack direction="row" spacing={1.5} alignItems="center">
                    <Box className="tts-dot" sx={{ backgroundColor: profile.accent }} />
                    <Typography className="tts-select__name">{profile.name}</Typography>
                    <Chip label={profile.encoding} size="small" />
                  </Stack>
                ) : (
                  <Typography className="tts-select__placeholder">TTSを選択してください</Typography>
                );
              }}
              value={tts}
            >
              {ttsProfiles.map((profile) => (
                <MenuItem key={profile.id} value={profile.id}>
                  <Stack direction="row" spacing={1.5} alignItems="center">
                    <Box className="tts-dot" sx={{ backgroundColor: profile.accent }} />
                    <Box>
                      <Typography>{profile.name}</Typography>
                      <Typography className="tts-menu__encoding">{profile.encoding}</Typography>
                    </Box>
                  </Stack>
                </MenuItem>
              ))}
            </Select>
          </Box>

          <Box className="step-section">
            <Stack className="step-heading" direction="row" spacing={1.5} alignItems="center">
              <Box className="step-heading__number">02</Box>
              <Box>
                <Typography className="step-heading__title">入力フォルダ</Typography>
                <Typography className="step-heading__description">同名の音声とテキストを含むフォルダ</Typography>
              </Box>
            </Stack>
            <PathSelector
              actionLabel="フォルダを選択"
              description=".wav と .txt が格納されたフォルダ"
              emptyLabel="フォルダが選択されていません"
              icon={<AudioFileRoundedIcon />}
              label="SOURCE DIRECTORY"
              onSelect={selectInputDirectory}
              value={input}
            />
          </Box>

          <Box className="step-section">
            <Stack className="step-heading" direction="row" spacing={1.5} alignItems="center">
              <Box className="step-heading__number">03</Box>
              <Box>
                <Typography className="step-heading__title">出力ファイル</Typography>
                <Typography className="step-heading__description">用途に合わせて形式と保存先を指定</Typography>
              </Box>
            </Stack>
            <ToggleButtonGroup
              className="format-switch"
              exclusive
              fullWidth
              onChange={(_, value: OutputFormat | null) => changeFormat(value)}
              value={format}
            >
              <ToggleButton value="srt">
                <DescriptionRoundedIcon />
                <Box>
                  <Typography className="format-switch__title">SRT</Typography>
                  <Typography className="format-switch__caption">汎用字幕</Typography>
                </Box>
              </ToggleButton>
              <ToggleButton value="fcpxml">
                <TuneRoundedIcon />
                <Box>
                  <Typography className="format-switch__title">FCPXML</Typography>
                  <Typography className="format-switch__caption">Final Cut Pro</Typography>
                </Box>
              </ToggleButton>
            </ToggleButtonGroup>
            <PathSelector
              actionLabel="保存先を選択"
              description={`.${format} ファイルとして保存`}
              emptyLabel="保存先が選択されていません"
              icon={<DescriptionRoundedIcon />}
              label="OUTPUT FILE"
              onSelect={selectOutputFile}
              value={output}
            />
          </Box>
        </Box>

        <Box component="aside" className="summary-panel">
          <Stack spacing={3}>
            <Box>
              <Typography className="summary-panel__eyebrow">GENERATION SUMMARY</Typography>
              <Typography className="summary-panel__title">書き出し設定</Typography>
            </Box>

            <Stack spacing={1.25}>
              <Box className="summary-row">
                <Typography>プロファイル</Typography>
                <Typography>{selectedProfile?.name ?? "未選択"}</Typography>
              </Box>
              <Box className="summary-row">
                <Typography>文字コード</Typography>
                <Typography>{selectedProfile?.encoding ?? "—"}</Typography>
              </Box>
              <Box className="summary-row">
                <Typography>出力形式</Typography>
                <Typography>{format.toUpperCase()}</Typography>
              </Box>
            </Stack>

            <Box className="summary-flow">
              <Stack direction="row" alignItems="center" justifyContent="space-between">
                <Box className="summary-flow__node">
                  <AudioFileRoundedIcon />
                  <Typography>WAV + TXT</Typography>
                </Box>
                <ArrowForwardRoundedIcon className="summary-flow__arrow" />
                <Box className="summary-flow__node summary-flow__node--accent">
                  <DescriptionRoundedIcon />
                  <Typography>{format.toUpperCase()}</Typography>
                </Box>
              </Stack>
            </Box>

            <StatusBanner status={status} />

            <Button
              className="generate-button"
              disabled={!canGenerate}
              fullWidth
              onClick={generateSubtitle}
              size="large"
              startIcon={isRunning ? <AutoAwesomeRoundedIcon /> : <PlayArrowRoundedIcon />}
              variant="contained"
            >
              {isRunning ? "生成しています…" : "字幕を生成"}
            </Button>
            <Typography className="summary-panel__hint">
              実行すると選択した出力先へ直接保存されます。
            </Typography>
          </Stack>
        </Box>
      </Box>
    </Box>
  );
}
