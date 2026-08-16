import CheckRoundedIcon from "@mui/icons-material/CheckRounded";
import { Box, Stack, Typography } from "@mui/material";

type StepHeadingProps = {
  complete: boolean;
  description: string;
  number: string;
  title: string;
};

export function StepHeading({
  complete,
  description,
  number,
  title,
}: StepHeadingProps) {
  return (
    <Stack
      className={`step-heading${complete ? " step-heading--complete" : ""}`}
      direction="row"
      spacing={1.5}
      alignItems="center"
    >
      <Box
        aria-label={complete ? `${title}は設定済みです` : `ステップ${number}`}
        className="step-heading__number"
      >
        {complete ? <CheckRoundedIcon /> : number}
      </Box>
      <Box>
        <Stack direction="row" spacing={1} alignItems="center">
          <Typography className="step-heading__title">{title}</Typography>
          {complete && (
            <Typography className="step-heading__complete-label">設定済み</Typography>
          )}
        </Stack>
        <Typography className="step-heading__description">{description}</Typography>
      </Box>
    </Stack>
  );
}
