import {styled} from "@mui/material/styles/";
// import { Typography } from "@mui/material";

import { Typography } from "@mui/material"

const CustomTypography = styled(Typography)({
  fontSize: 20,
  // color: 'red',
})

export default function Para({children, ...props}) {
  console.log(children)
  return (
    <CustomTypography>{children}</CustomTypography>
  )
  // return <CustomTypography props={props}>hello, world</CustomTypography> 
}