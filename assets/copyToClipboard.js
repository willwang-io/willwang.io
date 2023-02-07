import { useState } from "react";
import ContentCopyIcon from "@mui/icons-material/ContentCopy";
import IconButton from "@mui/material/IconButton";
import Tooltip from "@mui/material/Tooltip";
import ClickAwayListener from "@mui/material/ClickAwayListener";

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
    <ClickAwayListener onClickAway={handleTooltipClose}>
      <Tooltip
        PopperProps={{ disablePortal: true }}
        onClose={handleTooltipClose}
        open={open}
        disableFocusListener
        disableHoverListener
        disableTouchListener
        leaveTouchDelay={1000}
        title="Copied"
      >
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
          <ContentCopyIcon fontSize="small" />
        </IconButton>
      </Tooltip>
      {/* <Snackbar
        message="Copied to clipboard"
        autoHideDuration={1500}
        onClose={() => setOpen(false)}
        open={open}
      /> */}

    </ClickAwayListener>
    </>
  );
}
