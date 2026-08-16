import CheckCircleRoundedIcon from "@mui/icons-material/CheckCircleRounded";
import ErrorRoundedIcon from "@mui/icons-material/ErrorRounded";
import InfoRoundedIcon from "@mui/icons-material/InfoRounded";
import SyncRoundedIcon from "@mui/icons-material/SyncRounded";
import { Box, Stack, Typography } from "@mui/material";
import type { GenerationStatus } from "../types";

type StatusBannerProps = {
  status: GenerationStatus;
};

const content = {
  idle: {
    icon: <InfoRoundedIcon />,
    title: "準備ができたら生成を開始できます",
  },
  running: {
    icon: <SyncRoundedIcon className="status-banner__spinner" />,
    title: "字幕ファイルを生成しています",
  },
  success: {
    icon: <CheckCircleRoundedIcon />,
    title: "字幕ファイルを生成しました",
  },
  error: {
    icon: <ErrorRoundedIcon />,
    title: "生成処理を完了できませんでした",
  },
};

export function StatusBanner({ status }: StatusBannerProps) {
  const view = content[status.kind];

  return (
    <Box className={`status-banner status-banner--${status.kind}`}>
      <Stack direction="row" spacing={1.5} alignItems="center">
        <Box className="status-banner__icon">{view.icon}</Box>
        <Box>
          <Typography className="status-banner__title">{view.title}</Typography>
          <Typography className="status-banner__message">{status.message}</Typography>
        </Box>
      </Stack>
    </Box>
  );
}
