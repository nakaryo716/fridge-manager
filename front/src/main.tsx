import React from 'react'
import ReactDOM from 'react-dom/client'
import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import { App } from './App';
import "./index.css";
import { CookiesProvider } from 'react-cookie';
import { RouterProvider, createBrowserRouter, redirect } from 'react-router-dom';
import { SignIn } from './components/SignIn';
import { SignUp } from './components/SignUp';
import { SessionValue, isSession } from './api/session';

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

const router = createBrowserRouter([
  {
    path: "/app",
    element: <App />,
    loader: async () => {
      const res = await isSession();
      if (res == SessionValue.None) {
        return redirect("/sign_in");
      }
      return null
    }
  },
  {
    path: "/sign_in",
    element: <SignIn />,
  },
  {
    path: "/sign_up",
    element: <SignUp />,
  }
]);

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <ThemeProvider theme={isDark()}>
      <CssBaseline />
        <CookiesProvider>
          <RouterProvider router={router} />
        </CookiesProvider>
    </ThemeProvider>
  </React.StrictMode>
)
