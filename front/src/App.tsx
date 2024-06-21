import { useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import { useNavigate } from "react-router-dom";
import { NewFoodPayload, TrackedFood, UpdateFoodPayload } from "./types/itemType";
import { deleteFoodApi, getAllFoodsApi, postFoodApi, updateFoodApi } from "./api/callApi";
import { signOut } from "./api/auth";
import { InputItem } from "./components/InputItem";
import { ItemList } from "./components/ItemList";
import { Header } from "./components/Header";

export const App = () => {
  const [foods, setFoods] = useState<TrackedFood[]>([]);

  const onSubmitHandle = async (payload: NewFoodPayload) => {
    await postFoodApi(payload);

    const modifiedFoods = await getAllFoodsApi();
    setFoods(modifiedFoods);
  };

  const onUpdateHandle = async (id: number, payload: UpdateFoodPayload) => {
    await updateFoodApi(id, payload);

    const modifiedFoods = await getAllFoodsApi();
    setFoods(modifiedFoods);
  };

  const onDeleteHandle = async (id: number) => {
    await deleteFoodApi(id);

    const modifiedFoods = await getAllFoodsApi();
    setFoods(modifiedFoods);
  };

  // 初回バインド時にデータ取得
  useEffect(() => {
    (async () => {
      const getFoods = await getAllFoodsApi();
      setFoods(getFoods);
    })();
  }, []);


  const removeCookie = useCookies(["session_id"])[2];
  const navigate = useNavigate();
  
  const onClickRmSession = async () => {
    try {
      await signOut();
      removeCookie("session_id");
      navigate("/sign_in");
    } catch {
      alert("サインアウトできませんでした");
    }
  }

  return(
    <> 
      <Header signOutHandle={onClickRmSession}/>
      <InputItem onSubmitHandle={onSubmitHandle} />
      <ItemList foods={foods} onUpdateHandle={onUpdateHandle} onDeleteHandle={onDeleteHandle}></ItemList>
    </>
  );
};
