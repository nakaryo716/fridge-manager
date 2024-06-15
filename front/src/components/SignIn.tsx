import { Container, Card, TextField, Button, Typography } from '@mui/material';

export const SignIn = () => {
  return (
    <Container maxWidth="sm"  sx={{textAlign: "center"}}>
      <Card sx={{ mt: 8 }}>
        <Typography variant="h2" sx={{ mb: 2 }}>
          ログイン
        </Typography>
        <TextField id="email" label="メールアドレス" variant="outlined" margin="normal" fullWidth />
        <TextField
          id="password"
          label="パスワード"
          type="password"
          variant="outlined"
          margin="normal"
          fullWidth
        />
        <Button variant="contained" type="submit" sx={{ mt: 3} }>
          サインイン
        </Button>
      </Card>
    </Container>
  );
};
