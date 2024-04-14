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
        const {id, name, expiration_date, used} = Updates;

        const payload: UpdateFoodPayload = {
            name,
            expiration_date,
            used: !used
        };  
        await onUpdateHandle(id, payload);
    };

    const onClickDelete = async (id: number) => {
        await onDeleteHandle(id);
    };

    const [modalFlag, setModalFlag] = useState(false);
    const modalOpen = (id: number) => {
        console.log(id);
        setModalFlag(true);
    }
    const modalClose = () => setModalFlag(false);

    return(
        <TableRow key={food.id}>
        <TableCell align="left"><Checkbox onChange={() => onClickCheckBox(food)} checked={food.used}/>{food.name}</TableCell>
        <TableCell align="left">{food.expiration_date}</TableCell>
        <TableCell align="left">
            <Button key={food.id} variant="contained" sx={{backgroundColor: "mediumturquoise", "&:hover":{backgroundColor: "darkcyan"}}} onClick={() => modalOpen(food.id)}>編集</Button>
            <Modal open={modalFlag} onClose={modalClose}>
                <Box sx={style}>
                    <EditItem food={food} onUpdateHandle={onUpdateHandle} setModalFlag={setModalFlag}></EditItem>
                </Box>
            </Modal>
        </TableCell>
        <TableCell align="left">
            <Button variant="contained" sx={{backgroundColor: "pink", "&:hover":{backgroundColor: "hotpink"}}} onClick={() => onClickDelete(food.id)}>削除</Button>
        </TableCell>
    </TableRow>
    )
};

const style = {
    position: 'absolute',
    top: '50%',
    left: '50%',
    transform: 'translate(-50%, -50%)',
    width: 1200,
    bgcolor: 'background.paper',
    border: '2px solid #000',
    boxShadow: 24,
    p: 4,
};