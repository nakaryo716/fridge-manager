import { Box, Button, Checkbox, Modal, TableCell, TableRow } from "@mui/material";
import { EditItem } from "./EditItem";
import { TrackedFood, UpdateFoodPayload } from "../types/itemType";
import { useState } from "react";

type Props = {
    food: TrackedFood,
    onUpdateHandle: (id: number, payload: UpdateFoodPayload) => Promise<void>,
    onDeleteHandle: (id: number) => Promise<void>,
};

export const FoodItem = (props: Props) => {
    const {food, onUpdateHandle, onDeleteHandle} = props;

    const onClickCheckBox = async (Updates: TrackedFood) => {
        const {food_id, food_name, expiration, used} = Updates;

        const payload: UpdateFoodPayload = {
            food_name,
            expiration,
            used: !used
        };  
        await onUpdateHandle(food_id, payload);
    };

    const onClickDelete = async (id: number) => {
        await onDeleteHandle(id);
    };

    const [modalFlag, setModalFlag] = useState(false);
    const modalOpen = () => setModalFlag(true);
    const modalClose = () => setModalFlag(false);

    return(
        <TableRow key={food.food_id}>
        <TableCell align="left"><Checkbox onChange={() => onClickCheckBox(food)} checked={food.used} sx={ checkBoxStyle }/>{food.food_name}</TableCell>
        <TableCell align="left">{food.expiration}</TableCell>
        <TableCell align="left">
            <Button key={food.food_id} variant="contained" sx={{backgroundColor: "#007d7d", "&:hover":{backgroundColor: "#005757"}}} onClick={modalOpen}>編集</Button>
            <Modal open={modalFlag} onClose={modalClose}>
                <Box sx={style}>
                    <EditItem food={food} onUpdateHandle={onUpdateHandle} setModalFlag={setModalFlag}></EditItem>
                </Box>
            </Modal>
        </TableCell>
        <TableCell align="left">
            <Button variant="contained" sx={{backgroundColor: "#c22755", "&:hover":{backgroundColor: "#7f1830"}}} onClick={() => onClickDelete(food.food_id)}>削除</Button>
        </TableCell>
    </TableRow>
    )
};

const style = {
    position: 'absolute',
    top: '50%',
    left: '50%',
    transform: 'translate(-50%, -50%)',
    width: 750,
    bgcolor: 'background.paper',
    border: '2px solid #000',
    boxShadow: 24,
    p: 4,
};

const checkBoxStyle = {
    '&.Mui-checked': {
      color: "#1196ab"
    },
}
