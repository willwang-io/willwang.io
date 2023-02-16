import { useState } from "react";
import IconButton from "@mui/material/IconButton";
import CheckIcon from "@mui/icons-material/Check";
import Snackbar from "@mui/material/Snackbar";
import ContentPasteIcon from '@mui/icons-material/ContentPaste';

export default function CopyToClipboard({ content }) {
  const [open, setOpen] = useState(false);

  const handleTooltipClose = () => {
    setOpen(false);
  };

  const handleTooltipOpen = () => {
    setOpen(true);
  };

  return (
    <>
      <IconButton
        sx={{
          top: 0,
          right: 0,
          position: "absolute",
          opacity: 1,
        }}
        onClick={() => {
          setOpen(true);
          navigator.clipboard.writeText(content);
        }}
      >
        {!open ? (
          <ContentPasteIcon fontSize="small" />
        ) : (
          <CheckIcon sx={{}} color="success" fontSize="small" />
        )}
      </IconButton>
      <Snackbar
        message="Copied to clipboard"
        autoHideDuration={1500}
        onClose={() => setOpen(false)}
        open={open}
      />
    </>
  );
}
