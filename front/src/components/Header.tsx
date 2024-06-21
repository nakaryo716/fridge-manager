import { AppBar, Box, Button, Toolbar, Typography } from "@mui/material"

type Props = {
    signOutHandle: () => Promise<void>
};

export const Header = (props: Props) => {
    const  { signOutHandle }  = props;

    return(
        <Box sx={{ flexGrow: 1 }}>
        <AppBar position="static" sx={{backgroundColor: "#003366"}}>
          <Toolbar>
            <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
                賞味・消費期限マネージャー
            </Typography>
            <Button color="inherit" onClick={signOutHandle}>サインアウト</Button>
          </Toolbar>
        </AppBar>
      </Box>
    )
}
