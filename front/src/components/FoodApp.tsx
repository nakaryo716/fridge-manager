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

// テスト
// 9bf7f092-535e-4fb5-9694-d88024082292 ryo
// 2ae35ec2-a3d6-445d-9287-3dcd95a3bcef akemi
  const [cookie, setCookie, removeCookie] = useCookies(["session_id"]);

  const onClickSession = () => {
    console.log(cookie);
    setCookie("session_id", "9bf7f092-535e-4fb5-9694-d88024082292");
  }

  const onClickRmSession = () => {
    removeCookie("session_id");
  }

  return(
    <>
      <button onClick={onClickSession}>session</button>
      <button onClick={onClickRmSession}>remove session</button>
      <h1 style={{textAlign: "center"}}>賞味・消費期限マネージャー</h1>
      <InputItem onSubmitHandle={onSubmitHandle} />
      <ItemList foods={foods} onUpdateHandle={onUpdateHandle} onDeleteHandle={onDeleteHandle}></ItemList>
    </>
  );
};
