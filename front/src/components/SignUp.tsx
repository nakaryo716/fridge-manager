import { Container, Card, TextField, Button, Typography } from '@mui/material';
import React, { useState } from 'react';
import { NewUser } from '../types/middleware';
import { signUP } from '../api/auth';
import { useNavigate } from 'react-router-dom';

export const SignUp = () => {
  const [userName, setUserName] = useState("");
  const [mail, setMailText] = useState("");
  const [password, setPassWord] = useState("");

  const onChengeUserName = (e: React.ChangeEvent<HTMLInputElement>) => {
    setUserName(e.target.value);
  };
  
  const onChengeMail = (e: React.ChangeEvent<HTMLInputElement>) => {
    setMailText(e.target.value);
  };
  
  const onChengePass = (e: React.ChangeEvent<HTMLInputElement>) => {
    setPassWord(e.target.value);
  };

  const navigate = useNavigate();
  const onClickSignUp = async () => {
    if (password.length < 8) {
      alert("パスワードは8文字以上にしてください");
      return;
    }

    const payload: NewUser = {
      user_name: userName,
      mail,
      password,
    };

    try {
      const res = await signUP(payload);
      if (!res.ok) {
        switch (res.status) {
          case 400:
            alert("既に存在するユーザーか無効なアドレスです");
            break;
          default:
            alert("予期せぬエラーが発生しました");
            break;
        }
      } else {
        navigate("/sign_in");
      }
    } catch {
      alert("予期せぬエラーが発生しました");
    }
  };


  return (
    <Container maxWidth="sm" sx={{ textAlign: 'center' }}>
      <Card sx={{ mt: 8 }}>
        <Typography variant="h2" sx={{ mb: 2, marginTop: 3 }}>
          サインアップ
        </Typography>
        <TextField
          id="username"
          label="ユーザー名"
          variant="outlined"
          margin="normal"
          sx={{ width: 450 }}
          onChange={(e: React.ChangeEvent<HTMLInputElement>) => onChengeUserName(e)}
        />
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
        <div>
            <Button variant="contained" type="submit" sx={ buttonStyle } onClick={onClickSignUp}>
            登録
            </Button>
        </div>
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
