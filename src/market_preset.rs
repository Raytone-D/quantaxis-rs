use std::collections::HashMap;

use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CodePreset {
    pub name: String,
    pub unit_table: i32,
    pub price_tick: f64,
    pub buy_frozen_coeff: f64,
    pub sell_frozen_coeff: f64,
    pub exchange: String,
    pub commission_coeff_peramount: f64,
    pub commission_coeff_pervol: f64,
    pub commission_coeff_today_peramount: f64,
    pub commission_coeff_today_pervol: f64,
}

impl CodePreset {
    pub fn calc_marketvalue(&mut self, price: f64, volume: f64) -> f64 {
        volume * price * self.unit_table as f64
    }

    pub fn calc_frozenmoney(&mut self, price: f64, volume: f64) -> f64 {
        self.calc_marketvalue(price, volume) * self.buy_frozen_coeff
    }

    pub fn calc_commission(&mut self, price: f64, volume: f64) -> f64 {
        self.commission_coeff_pervol * volume +
            self.commission_coeff_peramount * self.calc_marketvalue(price, volume)
    }

    pub fn calc_commission_today(&mut self, price: f64, volume: f64) -> f64 {
        self.commission_coeff_today_pervol * volume +
            self.commission_coeff_today_peramount * self.calc_marketvalue(price, volume)
    }
    pub fn calc_coeff(&mut self) -> f64 {
        self.buy_frozen_coeff * self.unit_table as f64
    }
}

pub struct MarketPreset {
    preset: HashMap<String, CodePreset>
}


impl MarketPreset {
    pub fn new() -> Self {
        let mut market_preset: HashMap<String, CodePreset> = HashMap::new();
        market_preset.insert("AG".to_string(), CodePreset {
            name: "白银".to_string(),
            unit_table: 15,
            price_tick: 1.0,
            buy_frozen_coeff: 0.06,
            sell_frozen_coeff: 0.06,
            exchange: "SHFE".to_string(),
            commission_coeff_peramount: 0.00005,
            commission_coeff_pervol: 0.0,
            commission_coeff_today_peramount: 0.00005,
            commission_coeff_today_pervol: 0.0,
        });
        market_preset.insert(
            "AG".to_string(), CodePreset {
                name: "白银".to_string(),
                unit_table: 15,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "SHFE".to_string(),
                commission_coeff_peramount: 5e-05,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 5e-05,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "AL".to_string(), CodePreset {
                name: "铝".to_string(),
                unit_table: 5,
                price_tick: 5.0,
                buy_frozen_coeff: 5.0,
                sell_frozen_coeff: 5.0,
                exchange: "SHFE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 3.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "AU".to_string(), CodePreset {
                name: "黄金".to_string(),
                unit_table: 1000,
                price_tick: 0.05,
                buy_frozen_coeff: 0.05,
                sell_frozen_coeff: 0.05,
                exchange: "SHFE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 10.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "BU".to_string(), CodePreset {
                name: "石油沥青".to_string(),
                unit_table: 10,
                price_tick: 2.0,
                buy_frozen_coeff: 2.0,
                sell_frozen_coeff: 2.0,
                exchange: "SHFE".to_string(),
                commission_coeff_peramount: 0.0001,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.0003,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "CU".to_string(), CodePreset {
                name: "铜".to_string(),
                unit_table: 5,
                price_tick: 10.0,
                buy_frozen_coeff: 10.0,
                sell_frozen_coeff: 10.0,
                exchange: "SHFE".to_string(),
                commission_coeff_peramount: 5e-05,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "FU".to_string(), CodePreset {
                name: "燃料油".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "SHFE".to_string(),
                commission_coeff_peramount: 5e-05,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "HC".to_string(), CodePreset {
                name: "热轧卷板".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "SHFE".to_string(),
                commission_coeff_peramount: 0.0001,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.0001,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "NI".to_string(), CodePreset {
                name: "镍".to_string(),
                unit_table: 1,
                price_tick: 10.0,
                buy_frozen_coeff: 10.0,
                sell_frozen_coeff: 10.0,
                exchange: "SHFE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 1.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "PB".to_string(), CodePreset {
                name: "铅".to_string(),
                unit_table: 5,
                price_tick: 5.0,
                buy_frozen_coeff: 5.0,
                sell_frozen_coeff: 5.0,
                exchange: "SHFE".to_string(),
                commission_coeff_peramount: 4e-05,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "RB".to_string(), CodePreset {
                name: "螺纹钢".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 0.09,
                sell_frozen_coeff: 0.09,
                exchange: "SHFE".to_string(),
                commission_coeff_peramount: 0.0001,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.0001,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "RU".to_string(), CodePreset {
                name: "天然橡胶".to_string(),
                unit_table: 10,
                price_tick: 5.0,
                buy_frozen_coeff: 0.09,
                sell_frozen_coeff: 0.09,
                exchange: "SHFE".to_string(),
                commission_coeff_peramount: 4.5e-05,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 4.5e-05,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "SN".to_string(), CodePreset {
                name: "锡".to_string(),
                unit_table: 1,
                price_tick: 10.0,
                buy_frozen_coeff: 10.0,
                sell_frozen_coeff: 10.0,
                exchange: "SHFE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 1.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "SP".to_string(), CodePreset {
                name: "漂针浆".to_string(),
                unit_table: 10,
                price_tick: 2.0,
                buy_frozen_coeff: 2.0,
                sell_frozen_coeff: 2.0,
                exchange: "SHFE".to_string(),
                commission_coeff_peramount: 5e-05,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "WR".to_string(), CodePreset {
                name: "线材".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "SHFE".to_string(),
                commission_coeff_peramount: 4e-05,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "ZN".to_string(), CodePreset {
                name: "锌".to_string(),
                unit_table: 5,
                price_tick: 5.0,
                buy_frozen_coeff: 5.0,
                sell_frozen_coeff: 5.0,
                exchange: "SHFE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 3.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "A".to_string(), CodePreset {
                name: "黄大豆".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 2.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 2.0,
            });
        market_preset.insert(
            "B".to_string(), CodePreset {
                name: "黄大豆".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 1.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 1.0,
            });
        market_preset.insert(
            "BB".to_string(), CodePreset {
                name: "细木工板".to_string(),
                unit_table: 500,
                price_tick: 0.05,
                buy_frozen_coeff: 0.05,
                sell_frozen_coeff: 0.05,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.0001,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 5e-05,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "C".to_string(), CodePreset {
                name: "黄玉米".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 1.2,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "CS".to_string(), CodePreset {
                name: "玉米淀粉".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 1.5,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "EG".to_string(), CodePreset {
                name: "乙二醇".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 4.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "FB".to_string(), CodePreset {
                name: "中密度纤维板".to_string(),
                unit_table: 500,
                price_tick: 0.05,
                buy_frozen_coeff: 0.05,
                sell_frozen_coeff: 0.05,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.0001,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 5e-05,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "I".to_string(), CodePreset {
                name: "铁矿石".to_string(),
                unit_table: 100,
                price_tick: 0.5,
                buy_frozen_coeff: 0.5,
                sell_frozen_coeff: 0.5,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 6e-05,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 6e-05,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "J".to_string(), CodePreset {
                name: "冶金焦炭".to_string(),
                unit_table: 100,
                price_tick: 0.5,
                buy_frozen_coeff: 0.5,
                sell_frozen_coeff: 0.5,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.00018,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.00018,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "JD".to_string(), CodePreset {
                name: "鲜鸡蛋".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.00015,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.00015,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "JM".to_string(), CodePreset {
                name: "焦煤".to_string(),
                unit_table: 60,
                price_tick: 0.5,
                buy_frozen_coeff: 0.5,
                sell_frozen_coeff: 0.5,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.00018,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.00018,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "L".to_string(), CodePreset {
                name: "线型低密度聚乙烯".to_string(),
                unit_table: 5,
                price_tick: 5.0,
                buy_frozen_coeff: 5.0,
                sell_frozen_coeff: 5.0,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 2.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "M".to_string(), CodePreset {
                name: "豆粕".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 1.5,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "P".to_string(), CodePreset {
                name: "棕榈油".to_string(),
                unit_table: 10,
                price_tick: 2.0,
                buy_frozen_coeff: 2.0,
                sell_frozen_coeff: 2.0,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 2.5,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "PP".to_string(), CodePreset {
                name: "聚丙烯".to_string(),
                unit_table: 5,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 6e-05,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 3e-05,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "V".to_string(), CodePreset {
                name: "聚氯乙烯".to_string(),
                unit_table: 5,
                price_tick: 5.0,
                buy_frozen_coeff: 5.0,
                sell_frozen_coeff: 5.0,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 2.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "Y".to_string(), CodePreset {
                name: "豆油".to_string(),
                unit_table: 10,
                price_tick: 2.0,
                buy_frozen_coeff: 2.0,
                sell_frozen_coeff: 2.0,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 2.5,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "AP".to_string(), CodePreset {
                name: "鲜苹果".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 5.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 5.0,
            });
        market_preset.insert(
            "CF".to_string(), CodePreset {
                name: "一号棉花".to_string(),
                unit_table: 5,
                price_tick: 5.0,
                buy_frozen_coeff: 5.0,
                sell_frozen_coeff: 5.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 4.3,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "CY".to_string(), CodePreset {
                name: "棉纱".to_string(),
                unit_table: 5,
                price_tick: 5.0,
                buy_frozen_coeff: 5.0,
                sell_frozen_coeff: 5.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 4.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "FG".to_string(), CodePreset {
                name: "玻璃".to_string(),
                unit_table: 20,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 3.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 6.0,
            });
        market_preset.insert(
            "JR".to_string(), CodePreset {
                name: "粳稻".to_string(),
                unit_table: 20,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 3.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 3.0,
            });
        market_preset.insert(
            "LR".to_string(), CodePreset {
                name: "晚籼稻".to_string(),
                unit_table: 20,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 3.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 3.0,
            });
        market_preset.insert(
            "MA".to_string(), CodePreset {
                name: "甲醇MA".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 2.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 6.0,
            });
        market_preset.insert(
            "OI".to_string(), CodePreset {
                name: "菜籽油".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 2.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "PM".to_string(), CodePreset {
                name: "普通小麦".to_string(),
                unit_table: 50,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 5.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 5.0,
            });
        market_preset.insert(
            "RI".to_string(), CodePreset {
                name: "早籼".to_string(),
                unit_table: 20,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 2.5,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 2.5,
            });
        market_preset.insert(
            "RM".to_string(), CodePreset {
                name: "菜籽粕".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 1.5,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "RS".to_string(), CodePreset {
                name: "油菜籽".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 2.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 2.0,
            });
        market_preset.insert(
            "SF".to_string(), CodePreset {
                name: "硅铁".to_string(),
                unit_table: 5,
                price_tick: 2.0,
                buy_frozen_coeff: 2.0,
                sell_frozen_coeff: 2.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 3.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 9.0,
            });
        market_preset.insert(
            "SM".to_string(), CodePreset {
                name: "锰硅".to_string(),
                unit_table: 5,
                price_tick: 2.0,
                buy_frozen_coeff: 2.0,
                sell_frozen_coeff: 2.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 3.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 6.0,
            });
        market_preset.insert(
            "SR".to_string(), CodePreset {
                name: "白砂糖".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 3.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "TA".to_string(), CodePreset {
                name: "精对苯二甲酸".to_string(),
                unit_table: 5,
                price_tick: 2.0,
                buy_frozen_coeff: 2.0,
                sell_frozen_coeff: 2.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 3.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "WH".to_string(), CodePreset {
                name: "优质强筋小麦".to_string(),
                unit_table: 20,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 2.5,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "ZC".to_string(), CodePreset {
                name: "动力煤ZC".to_string(),
                unit_table: 100,
                price_tick: 0.2,
                buy_frozen_coeff: 0.2,
                sell_frozen_coeff: 0.2,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 4.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 4.0,
            });
        market_preset.insert(
            "SC".to_string(), CodePreset {
                name: "原油".to_string(),
                unit_table: 1000,
                price_tick: 0.1,
                buy_frozen_coeff: 0.1,
                sell_frozen_coeff: 0.1,
                exchange: "INE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 20.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "EB".to_string(), CodePreset {
                name: "苯乙烯".to_string(),
                unit_table: 5,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.0001,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.0001,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "RR".to_string(), CodePreset {
                name: "粳米".to_string(),
                unit_table: 10,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "DCE".to_string(),
                commission_coeff_peramount: 0.0001,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.0001,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "NR".to_string(), CodePreset {
                name: "20号胶".to_string(),
                unit_table: 10,
                price_tick: 5.0,
                buy_frozen_coeff: 5.0,
                sell_frozen_coeff: 5.0,
                exchange: "INE".to_string(),
                commission_coeff_peramount: 0.0001,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.0001,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "SS".to_string(), CodePreset {
                name: "不锈钢".to_string(),
                unit_table: 5,
                price_tick: 5.0,
                buy_frozen_coeff: 5.0,
                sell_frozen_coeff: 5.0,
                exchange: "SHFE".to_string(),
                commission_coeff_peramount: 0.0001,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.0001,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "CJ".to_string(), CodePreset {
                name: "红枣".to_string(),
                unit_table: 5,
                price_tick: 5.0,
                buy_frozen_coeff: 5.0,
                sell_frozen_coeff: 5.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0,
                commission_coeff_pervol: 3.0,
                commission_coeff_today_peramount: 0.0,
                commission_coeff_today_pervol: 3.0,
            });
        market_preset.insert(
            "UR".to_string(), CodePreset {
                name: "尿素".to_string(),
                unit_table: 20,
                price_tick: 1.0,
                buy_frozen_coeff: 1.0,
                sell_frozen_coeff: 1.0,
                exchange: "CZCE".to_string(),
                commission_coeff_peramount: 0.0001,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.0001,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "IC".to_string(), CodePreset {
                name: "中证500指数".to_string(),
                unit_table: 200,
                price_tick: 0.2,
                buy_frozen_coeff: 0.2,
                sell_frozen_coeff: 0.2,
                exchange: "CFFEX".to_string(),
                commission_coeff_peramount: 2.301e-05,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.00034501,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "IF".to_string(), CodePreset {
                name: "沪深300指数".to_string(),
                unit_table: 300,
                price_tick: 0.2,
                buy_frozen_coeff: 0.2,
                sell_frozen_coeff: 0.2,
                exchange: "CFFEX".to_string(),
                commission_coeff_peramount: 2.301e-05,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.0001,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "IH".to_string(), CodePreset {
                name: "上证50指数".to_string(),
                unit_table: 300,
                price_tick: 0.2,
                buy_frozen_coeff: 0.2,
                sell_frozen_coeff: 0.2,
                exchange: "CFFEX".to_string(),
                commission_coeff_peramount: 2.301e-05,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.00034501,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "TF".to_string(), CodePreset {
                name: "5年期国债".to_string(),
                unit_table: 10000,
                price_tick: 0.005,
                buy_frozen_coeff: 0.005,
                sell_frozen_coeff: 0.005,
                exchange: "CFFEX".to_string(),
                commission_coeff_peramount: 0.0001,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.0001,
                commission_coeff_today_pervol: 0.0,
            });
        market_preset.insert(
            "TS".to_string(), CodePreset {
                name: "2年期国债".to_string(),
                unit_table: 20000,
                price_tick: 0.005,
                buy_frozen_coeff: 0.005,
                sell_frozen_coeff: 0.005,
                exchange: "CFFEX".to_string(),
                commission_coeff_peramount: 0.0001,
                commission_coeff_pervol: 0.0,
                commission_coeff_today_peramount: 0.0001,
                commission_coeff_today_pervol: 0.0,
            });

        MarketPreset {
            preset: market_preset
        }
    }


    pub fn get(&mut self, code: &str) -> CodePreset {
        let mut preset = CodePreset {
            name: code.to_string(),
            unit_table: 1,
            price_tick: 0.01,
            buy_frozen_coeff: 1.0,
            sell_frozen_coeff: 0.0,
            exchange: "".to_string(),
            commission_coeff_peramount: 0.0,
            commission_coeff_pervol: 0.0,
            commission_coeff_today_peramount: 0.0,
            commission_coeff_today_pervol: 0.0,
        };


        let re = Regex::new(r"[a-zA-z]+").unwrap();

        let rcode = re.find(code);
        if rcode.is_some() {
            let codename = rcode.unwrap().as_str();

            if self.preset.contains_key(codename) {
                preset = self.preset.get_mut(codename).unwrap().to_owned();
            }
        }

        preset
    }
}

