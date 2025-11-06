use phf::phf_map;

/// 服务器配置信息
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub match_history: &'static str,
    pub common: &'static str,
}

/// 服务器配置映射
pub const SGP_SERVERS: phf::Map<&'static str, ServerConfig> = phf_map! {
    "TENCENT_HN1" => ServerConfig {
        match_history: "https://hn1-k8s-sgp.lol.qq.com:21019",
        common: "https://hn1-k8s-sgp.lol.qq.com:21019",
    },
    "TENCENT_HN10" => ServerConfig {
        match_history: "https://hn10-k8s-sgp.lol.qq.com:21019",
        common: "https://hn10-k8s-sgp.lol.qq.com:21019",
    },
    "TENCENT_TJ100" => ServerConfig {
        match_history: "https://tj100-sgp.lol.qq.com:21019",
        common: "https://tj100-sgp.lol.qq.com:21019",
    },
    "TENCENT_TJ101" => ServerConfig {
        match_history: "https://tj101-sgp.lol.qq.com:21019",
        common: "https://tj101-sgp.lol.qq.com:21019",
    },
    "TENCENT_NJ100" => ServerConfig {
        match_history: "https://nj100-sgp.lol.qq.com:21019",
        common: "https://nj100-sgp.lol.qq.com:21019",
    },
    "TENCENT_GZ100" => ServerConfig {
        match_history: "https://gz100-sgp.lol.qq.com:21019",
        common: "https://gz100-sgp.lol.qq.com:21019",
    },
    "TENCENT_CQ100" => ServerConfig {
        match_history: "https://cq100-sgp.lol.qq.com:21019",
        common: "https://cq100-sgp.lol.qq.com:21019",
    },
    "TENCENT_BGP2" => ServerConfig {
        match_history: "https://bgp2-k8s-sgp.lol.qq.com:21019",
        common: "https://bgp2-k8s-sgp.lol.qq.com:21019",
    },
    "TENCENT_PBE" => ServerConfig {
        match_history: "https://pbe-sgp.lol.qq.com:21019",
        common: "https://pbe-sgp.lol.qq.com:21019",
    },
    "TENCENT_PREPBE" => ServerConfig {
        match_history: "https://prepbe-sgp.lol.qq.com:21019",
        common: "https://prepbe-sgp.lol.qq.com:21019",
    },
    "TW2" => ServerConfig {
        match_history: "https://apse1-red.pp.sgp.pvp.net",
        common: "https://tw2-red.lol.sgp.pvp.net",
    },
    "SG2" => ServerConfig {
        match_history: "https://apse1-red.pp.sgp.pvp.net",
        common: "https://sg2-red.lol.sgp.pvp.net",
    },
    "PH2" => ServerConfig {
        match_history: "https://apse1-red.pp.sgp.pvp.net",
        common: "https://ph2-red.lol.sgp.pvp.net",
    },
    "VN2" => ServerConfig {
        match_history: "https://apse1-red.pp.sgp.pvp.net",
        common: "https://vn2-red.lol.sgp.pvp.net",
    },
    "PBE" => ServerConfig {
        match_history: "https://usw2-red.pp.sgp.pvp.net",
        common: "https://pbe-red.lol.sgp.pvp.net",
    },
    "EUW" => ServerConfig {
        match_history: "https://euc1-red.pp.sgp.pvp.net",
        common: "https://euw-red.lol.sgp.pvp.net",
    },
    "JP1" => ServerConfig {
        match_history: "https://apne1-red.pp.sgp.pvp.net",
        common: "https://jp-red.lol.sgp.pvp.net",
    },
    "RU" => ServerConfig {
        match_history: "https://euc1-red.pp.sgp.pvp.net",
        common: "https://ru-red.lol.sgp.pvp.net",
    },
    "BR1" => ServerConfig {
        match_history: "https://usw2-red.pp.sgp.pvp.net",
        common: "https://br-red.lol.sgp.pvp.net",
    },
    "OC1" => ServerConfig {
        match_history: "https://apse1-red.pp.sgp.pvp.net",
        common: "https://oce-red.lol.sgp.pvp.net",
    },
    "TR1" => ServerConfig {
        match_history: "https://euc1-red.pp.sgp.pvp.net",
        common: "https://tr-red.lol.sgp.pvp.net",
    },
    "LA1" => ServerConfig {
        match_history: "https://usw2-red.pp.sgp.pvp.net",
        common: "https://lan-red.lol.sgp.pvp.net",
    },
    "LA2" => ServerConfig {
        match_history: "https://usw2-red.pp.sgp.pvp.net",
        common: "https://las-red.lol.sgp.pvp.net",
    },
    "NA1" => ServerConfig {
        match_history: "https://usw2-red.pp.sgp.pvp.net",
        common: "https://na-red.lol.sgp.pvp.net",
    },
    "TH2" => ServerConfig {
        match_history: "https://apse1-red.pp.sgp.pvp.net",
        common: "https://th2-red.lol.sgp.pvp.net",
    },
    "KR" => ServerConfig {
        match_history: "https://apne1-red.pp.sgp.pvp.net",
        common: "https://kr-red.lol.sgp.pvp.net",
    },
};

/// 腾讯服务器匹配历史互操作性列表
pub const TENCENT_SERVER_MATCH_HISTORY_INTEROPERABILITY: &[&str] = &[
    "TENCENT_HN1",
    "TENCENT_HN10",
    "TENCENT_NJ100",
    "TENCENT_GZ100",
    "TENCENT_CQ100",
    "TENCENT_TJ100",
    "TENCENT_TJ101",
    "TENCENT_BGP2",
    "TENCENT_PBE",
    "TENCENT_PREPBE",
];

/// 腾讯服务器观战互操作性列表
pub const TENCENT_SERVER_SPECTATOR_INTEROPERABILITY: &[&str] = &[
    "TENCENT_HN1",
    "TENCENT_HN10",
    "TENCENT_NJ100",
    "TENCENT_GZ100",
    "TENCENT_CQ100",
    "TENCENT_TJ100",
    "TENCENT_TJ101",
    "TENCENT_BGP2",
    "TENCENT_PBE",
    "TENCENT_PREPBE",
];

/// 腾讯服务器召唤师互操作性列表
pub const TENCENT_SERVER_SUMMONER_INTEROPERABILITY: &[&str] = &[
    "TENCENT_HN1",
    "TENCENT_HN10",
    "TENCENT_NJ100",
    "TENCENT_GZ100",
    "TENCENT_CQ100",
    "TENCENT_TJ100",
    "TENCENT_TJ101",
    "TENCENT_BGP2",
    "TENCENT_PBE",
    "TENCENT_PREPBE",
];

/// 服务器名称映射（英文）
pub const SERVER_NAMES_EN: phf::Map<&'static str, &'static str> = phf_map! {
    "TENCENT_HN1" => "Ionia",
    "TENCENT_HN10" => "Black Rose",
    "TENCENT_TJ100" => "League 4",
    "TENCENT_TJ101" => "League 5",
    "TENCENT_NJ100" => "League 1",
    "TENCENT_GZ100" => "League 2",
    "TENCENT_CQ100" => "League 3",
    "TENCENT_BGP2" => "Rift's Summit",
    "TENCENT_PBE" => "PBE (Tencent)",
    "TENCENT_PREPBE" => "PREPBE (Tencent)",
    "TW2" => "Taiwan",
    "SG2" => "Singapore",
    "PH2" => "Philippines",
    "VN2" => "Vietnam",
    "PBE" => "PBE",
    "EUW" => "EUW",
    "JP1" => "Japan",
    "RU" => "Russia",
    "BR1" => "Brazil",
    "OC1" => "Oceania",
    "TR1" => "Turkey",
    "LA1" => "Latin America North",
    "LA2" => "Latin America South",
    "NA1" => "North America",
    "TH2" => "Thailand",
    "KR" => "Korea",
};

/// 服务器名称映射（中文简体）
pub const SERVER_NAMES_ZH_CN: phf::Map<&'static str, &'static str> = phf_map! {
    "TENCENT_HN1" => "艾欧尼亚",
    "TENCENT_HN10" => "黑色玫瑰",
    "TENCENT_TJ100" => "联盟四区",
    "TENCENT_TJ101" => "联盟五区",
    "TENCENT_NJ100" => "联盟一区",
    "TENCENT_GZ100" => "联盟二区",
    "TENCENT_CQ100" => "联盟三区",
    "TENCENT_BGP2" => "峡谷之巅",
    "TENCENT_PBE" => "PBE (腾讯)",
    "TENCENT_PREPBE" => "PREPBE (腾讯)",
    "TW2" => "台湾",
    "SG2" => "新加坡",
    "PH2" => "菲律宾",
    "VN2" => "越南",
    "PBE" => "PBE",
    "EUW" => "EUW",
    "JP1" => "日本",
    "RU" => "俄罗斯",
    "BR1" => "巴西",
    "OC1" => "大洋洲",
    "TR1" => "土耳其",
    "LA1" => "拉丁美洲北部",
    "LA2" => "拉丁美洲南部",
    "NA1" => "北美",
    "TH2" => "泰国",
    "KR" => "韩国",
};
