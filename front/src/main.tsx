import React from 'react'
import ReactDOM from 'react-dom/client'
import CssBaseline from '@mui/material/CssBaseline';
import { App } from './App';
import "./index.css";
import { RouterProvider, createBrowserRouter, redirect } from 'react-router-dom';
import { SignIn } from './components/SignIn';
import { SignUp } from './components/SignUp';
import { isSession } from './api/session';

const router = createBrowserRouter([
  {
    path: "/app",
    element: <App />,
    loader: async () => {
      try {
        const res = await isSession();
        if (!res.ok) {
          return redirect("/sign_in");
        }
        return null
      } catch {
        console.error("unexpected error");
        return redirect("/sign_in");
      }
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
    <CssBaseline />
      <RouterProvider router={router} />
  </React.StrictMode>
)
