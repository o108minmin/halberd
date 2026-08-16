import ContentCopyRoundedIcon from "@mui/icons-material/ContentCopyRounded";
import FolderOpenRoundedIcon from "@mui/icons-material/FolderOpenRounded";
import ReplayRoundedIcon from "@mui/icons-material/ReplayRounded";
import { Button, Stack, Typography } from "@mui/material";

type SuccessActionsProps = {
  disabled: boolean;
  onCopyPath: () => void;
  onRegenerate: () => void;
  onRevealOutput: () => void;
};

export function SuccessActions({
  disabled,
  onCopyPath,
  onRegenerate,
  onRevealOutput,
}: SuccessActionsProps) {
  return (
    <Stack className="success-actions" spacing={1.25}>
      <Typography className="success-actions__label">NEXT ACTION</Typography>
      <Button
        disabled={disabled}
        fullWidth
        onClick={onRevealOutput}
        startIcon={<FolderOpenRoundedIcon />}
        variant="outlined"
      >
        出力先を表示
      </Button>
      <Stack direction="row" spacing={1.25}>
        <Button
          disabled={disabled}
          fullWidth
          onClick={onCopyPath}
          startIcon={<ContentCopyRoundedIcon />}
          variant="text"
        >
          パスをコピー
        </Button>
        <Button
          disabled={disabled}
          fullWidth
          onClick={onRegenerate}
          startIcon={<ReplayRoundedIcon />}
          variant="text"
        >
          もう一度生成
        </Button>
      </Stack>
    </Stack>
  );
}
