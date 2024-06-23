import { Container, Card, TextField, Button, Typography } from '@mui/material';
import { signIn } from '../api/auth';
import { Credentials } from '../types/middleware';
import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';

export const SignIn = () => {
  const [mail, setMailText] = useState("");
  const [password, setPassWord] = useState("");
  
  const onChengeMail = (e: React.ChangeEvent<HTMLInputElement>) => {
    setMailText(e.target.value);
  };
  
  const onChengePass = (e: React.ChangeEvent<HTMLInputElement>) => {
    setPassWord(e.target.value);
  };

  const navigate = useNavigate();

  const onClickSignIn = async () => {
    const cred: Credentials = {
      mail: mail,
      password: password,
    };

    try {
      const res = await signIn(cred);
      
      if (!res) {
        alert("パスワードが違います");
      }
      navigate("/app");

    } catch {
      alert("パスワードが違います");
    }
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
        <Button variant="contained" type="submit" sx={ buttonStyle } onClick={onClickSignIn}>
          サインイン
        </Button>
      </Card>
    </Container>
  );
};

const buttonStyle = {
  mt: 3,
  marginBlock: 3,
  color: "black",
  backgroundColor: "#FF9900",
  "&:hover": {
      backgroundColor: "#FF6600",
  },
};
