import React from 'react'
import ReactDOM from 'react-dom/client'
import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import { App } from './App';
import "./index.css";

// 18時以降は自動でダークモードになるように設定する関数
const isDark = () => {
  const currentHour = new Date().getHours();

  if (currentHour > 18) {
    return createTheme({
      palette: {
        mode: 'dark',
      },
    })
  } else {
    return createTheme({
      palette: {
        mode: 'light',
      },
    })
  }
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <ThemeProvider theme={isDark()}>
      <CssBaseline />
        <App />
    </ThemeProvider>
  </React.StrictMode>
)
