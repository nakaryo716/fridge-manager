import { Button, FormControl, InputLabel, MenuItem, Select, SelectChangeEvent, TextField } from "@mui/material"
import React, { useState } from "react";
import { NewFoodPayload } from "../types/itemType";

type Props = {
    onSubmitHandle: (payload: NewFoodPayload) => Promise<void>;
};

export const InputItem = (props: Props) => {
    const { onSubmitHandle } = props;

    // デフォルト値を設定するために現在の日付を取得
    const currentYear = new Date().getFullYear();
    const currentMonth = new Date().getMonth();
    const currentDate = new Date().getDate();

    // セレクトバーに表示するための連番生成
    const sequentialYear = Array.from({length: 15}, (_, i) => (currentYear - 1) + i + 1);
    const sequentialMonth = Array.from({length: 12}, (_, i) => i + 1);
    const sequentialDate = Array.from({length: 31}, (_, i) => i + 1);

    const [text, setText] = useState("");

    const [selectedYear, setSelectedYear] = useState(currentYear);
    const [selectedMonth, setSelectedMonth] = useState(currentMonth + 1);
    const [selectedDate, setSelectedDate] = useState(currentDate);

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

    // 追加ボタンを押した際のAPI Call + Inputコンポーネント初期化
    const onClickSubmit = async () => {
        const ymd = `${selectedYear}-${selectedMonth}-${selectedDate}`;
        const payload: NewFoodPayload = {
            food_name: text,
            expiration: ymd
        };
        
        if (!payload.food_name) {
            setSelectedYear(currentYear);
            setSelectedMonth(currentMonth + 1);
            setSelectedDate(currentDate);
            return;
        }
        await onSubmitHandle(payload);

        setText("");
        setSelectedYear(currentYear);
        setSelectedMonth(currentMonth + 1);
        setSelectedDate(currentDate);
    };
    
    return(
        <>
            <div style={{textAlign: "center", margin: 40}}>
                <TextField sx={{width: 250}}id="filled-basic" label="商品名" variant="filled" onChange={(e:  React.ChangeEvent<HTMLInputElement>) =>followTextHandle(e)} value={text}/>
                <FormControl sx={{minWidth: 40, marginLeft: 2}}>
                    <InputLabel>年</InputLabel>
                    <Select value={selectedYear} onChange={(e) => onChengeYear(e)}>
                        {sequentialYear.map((year) => {
                            return(
                                <MenuItem key={year} value={year}>{year}</MenuItem>
                            );
                        })}
                    </Select>
                </FormControl>
                <FormControl sx={{minWidth: 80, marginLeft: 1}}>
                    <InputLabel>月</InputLabel>
                    <Select value={selectedMonth} onChange={(e) => onChengeMonth(e)}>
                        {sequentialMonth.map((month) => {
                            return(
                                <MenuItem key={month} value={month}>{month}</MenuItem>
                            );
                        })}
                    </Select>
                </FormControl>
                <FormControl sx={{minWidth: 80, marginLeft: 1}}>
                    <InputLabel>日</InputLabel>
                    <Select value={selectedDate} onChange={(e) => onChengeDate(e)}>
                        {sequentialDate.map((date) => {
                            return(
                                <MenuItem key={date}value={date}>{date}</MenuItem>
                            );
                        })}
                    </Select>
                </FormControl>
                <Button size="large" variant="contained" sx={{height: 56, width: 100, marginLeft: 2}} onClick={() => onClickSubmit()}>追加</Button>
            </div>
        </>
    );
};
