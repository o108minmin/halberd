import type { ReactNode } from "react";
import CheckCircleRoundedIcon from "@mui/icons-material/CheckCircleRounded";
import FolderOpenRoundedIcon from "@mui/icons-material/FolderOpenRounded";
import {
  Box,
  Button,
  Stack,
  Typography,
} from "@mui/material";

type PathSelectorProps = {
  actionLabel: string;
  description: string;
  emptyLabel: string;
  icon: ReactNode;
  label: string;
  onSelect: () => void;
  value: string;
};

function pathName(path: string) {
  const normalized = path.replace(/[\\/]+$/, "");
  return normalized.split(/[\\/]/).pop() ?? path;
}

export function PathSelector({
  actionLabel,
  description,
  emptyLabel,
  icon,
  label,
  onSelect,
  value,
}: PathSelectorProps) {
  return (
    <Box className={`path-selector${value ? " path-selector--selected" : ""}`}>
      <Stack direction="row" spacing={2} alignItems="center">
        <Box className="path-selector__icon">{icon}</Box>
        <Box className="path-selector__body">
          <Stack direction="row" spacing={1} alignItems="center">
            <Typography className="path-selector__label">{label}</Typography>
            {value && <CheckCircleRoundedIcon className="path-selector__check" />}
          </Stack>
          <Typography className="path-selector__value" title={value || emptyLabel}>
            {value ? pathName(value) : emptyLabel}
          </Typography>
          <Typography className="path-selector__description" title={value || description}>
            {value || description}
          </Typography>
        </Box>
        <Button
          className="path-selector__button"
          onClick={onSelect}
          startIcon={<FolderOpenRoundedIcon />}
          variant="outlined"
        >
          {actionLabel}
        </Button>
      </Stack>
    </Box>
  );
}
