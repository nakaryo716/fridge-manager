import { useEffect, useState } from "react";
import { isSession } from "./api/session";
import { FoodApp } from "./components/FoodApp"
import { SignIn } from "./components/SignIn"

export const App = () => {
  const [haveSession, setHaveSession] = useState(false);

  useEffect(() => {
    (
      async () => {
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
