import { Button, FormControl, InputLabel, MenuItem, Select, SelectChangeEvent, TextField } from "@mui/material";
import React, { useState } from "react";
import { TrackedFood, UpdateFoodPayload } from "../types/itemType";

type Props = {
    food: TrackedFood,
    onUpdateHandle: (id: number, payload: UpdateFoodPayload) => Promise<void>,
    setModalFlag: React.Dispatch<React.SetStateAction<boolean>>,
}

export const EditItem = (props: Props) => {
    const { food, onUpdateHandle, setModalFlag } = props;

    // foodの賞味期限をセレクトバーに表示するためにnumberに変換
    const settingYmd = food.expiration_date.split("-");
    const settingYear = parseInt(settingYmd[0]);
    const settingmonth = parseInt(settingYmd[1]);
    const settingDate = parseInt(settingYmd[2]);

    // セレクトバーに表示するための連番生成
    const sequentialYear = Array.from({length: 15}, (_, i) => (settingYear - 1) + i + 1);
    const sequentialMonth = Array.from({length: 12}, (_, i) => i + 1);
    const sequentialDate = Array.from({length: 31}, (_, i) => i + 1);

    const [text, setText] = useState("");
    const [selectedYear, setSelectedYear] = useState(settingYear);
    const [selectedMonth, setSelectedMonth] = useState(settingmonth);
    const [selectedDate, setSelectedDate] = useState(settingDate);

    const followTextHandle = (e: React.ChangeEvent<HTMLInputElement>) => {
        setText(e.target.value);
    };

    const onChengeYear = (e: SelectChangeEvent<number>) => {
        setSelectedYear(e.target.value as number);
    };

    const onChengeMonth = (e: SelectChangeEvent<number>) => {
        setSelectedMonth(e.target.value as number);
    };

    const onChengeDate = (e: SelectChangeEvent<number>) => {
        setSelectedDate(e.target.value as number);
    };

    const onClickEdit = async () => {
        const ymd = `${selectedYear}-${selectedMonth}-${selectedDate}`;
        const payload: UpdateFoodPayload = {
            name: text,
            expiration_date: ymd,
            used: food.used,
        };
        
        if (!payload.name) {
            payload.name = food.name;
        }

        await onUpdateHandle(food.id, payload);
        setModalFlag(false);
    };
    
    return(
            <div style={{textAlign: "center", margin: 40}}>
                <TextField sx={{width: 400}}id="filled-basic" label="商品名" variant="filled" onChange={(e:  React.ChangeEvent<HTMLInputElement>) =>followTextHandle(e)} defaultValue={food.name}/>
                <FormControl sx={{minWidth: 120, marginLeft: 2}}>
                    <InputLabel>Year</InputLabel>
                    <Select value={selectedYear} onChange={(e) => onChengeYear(e)}>
                        {sequentialYear.map((year) => {
                            return(
                                <MenuItem key={year} value={year}>{year}</MenuItem>
                            );
                        })}
                    </Select>
                </FormControl>
                <FormControl sx={{minWidth: 120, marginLeft: 1}}>
                    <InputLabel>Month</InputLabel>
                    <Select value={selectedMonth} onChange={(e) => onChengeMonth(e)}>
                        {sequentialMonth.map((month) => {
                            return(
                                <MenuItem key={month} value={month}>{month}</MenuItem>
                            );
                        })}
                    </Select>
                </FormControl>
                <FormControl sx={{minWidth: 120, marginLeft: 1}}>
                    <InputLabel>Date</InputLabel>
                    <Select value={selectedDate} onChange={(e) => onChengeDate(e)}>
                        {sequentialDate.map((date) => {
                            return(
                                <MenuItem key={date}value={date}>{date}</MenuItem>
                            );
                        })}
                    </Select>
                </FormControl>
                <Button size="large" variant="contained" sx={{height: 56, width: 180, marginLeft: 4}} onClick={() => onClickEdit()}>変更</Button>
            </div>
    );
};
