import { useEffect, useState } from "react";
import { isSession } from "./api/session";
import { FoodApp } from "./components/FoodApp"
import { SignIn } from "./components/SignIn"
import { SignUp } from "./components/SignUp";

export const App = () => {
  const [haveSession, setHaveSession] = useState(false);

  useEffect(() => {
    (
      async () => {
        // セッションがあるかどうか確認
        // 個の非同期関数の戻り値は列挙型にした方がいいのか？
        // Some(session) / None
        // セッションがあればアプリケーションページにリダイレクト
        // なければログインページにリダイレクト
        const res = await isSession();
        if (res) {
          setHaveSession(true);
        }
      }
    )();
  }, []);

  return(
      haveSession ? <FoodApp /> : <SignIn />
  )
}
