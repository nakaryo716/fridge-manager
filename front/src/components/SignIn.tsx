import { Container, Card, TextField, Button, Typography } from '@mui/material';
import { signIn } from '../api/auth';
import { Credentials } from '../types/middleware';
import React, { useState } from 'react';

export const SignIn = () => {
  const [mail, setMailText] = useState("");
  const [password, setPassWord] = useState("");
  
  const onChengeMail = (e: React.ChangeEvent<HTMLInputElement>) => {
    setMailText(e.target.value);
  };
  
  const onChengePass = (e: React.ChangeEvent<HTMLInputElement>) => {
    setPassWord(e.target.value);
  };

  const onClickSignIn = async () => {
    const cred: Credentials = {
      mail: mail,
      password: password,
    };

    await signIn(cred);

    // この後に失敗したら
    // パスワードが違いますの表示
    // Okだったらアプリケーションページにリダイレクトする
  };

  return (
    <Container maxWidth="sm"  sx={{textAlign: "center"}}>
      <Card sx={{ mt: 8 }}>
        <Typography variant="h2" sx={{ mb: 2, marginTop: 3 }}>
          サインイン
        </Typography>
        <TextField
          id="email"
          label="メールアドレス" 
          variant="outlined"
          margin="normal"
          sx={{ width: 450 }}
          onChange={(e: React.ChangeEvent<HTMLInputElement>) => onChengeMail(e)}
        />
        <TextField
          id="password"
          label="パスワード"
          type="password"
          variant="outlined"
          margin="normal"
          sx={{ width: 450 }}
          onChange={(e: React.ChangeEvent<HTMLInputElement>) => onChengePass(e)}
        />
        <Button variant="contained" type="submit" sx={{ mt: 3, marginBlock: 3 }} onClick={onClickSignIn}>
          サインイン
        </Button>
      </Card>
    </Container>
  );
};
