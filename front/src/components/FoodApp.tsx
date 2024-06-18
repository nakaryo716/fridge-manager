import { useEffect, useState } from "react";
import { useCookies } from "react-cookie";
import { NewFoodPayload, TrackedFood, UpdateFoodPayload } from "../types/itemType";
import { deleteFoodApi, getAllFoodsApi, postFoodApi, updateFoodApi } from "../api/callApi";
import { ItemList } from "./ItemList";
import { InputItem } from "./InputItem";

export const FoodApp = () => {
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


  const [cookie, setCookie, removeCookie] = useCookies(["session_id"]);
  
  const onClickRmSession = () => {
    removeCookie("session_id");
  }

  return(
    <>
      <button onClick={onClickRmSession}>ログアウト</button>
      <h1 style={{textAlign: "center"}}>賞味・消費期限マネージャー</h1>
      <InputItem onSubmitHandle={onSubmitHandle} />
      <ItemList foods={foods} onUpdateHandle={onUpdateHandle} onDeleteHandle={onDeleteHandle}></ItemList>
    </>
  );
};
