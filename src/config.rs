use uuid::Uuid;

pub struct Config {
    pub uuid: Uuid,
    pub host: String,
    pub proxy_addr: String,
    pub proxy_port: u16,

    pub main_page_url: String,
    pub sublink_page_url: String,
    pub weblink_page_url: String,
    pub vmess_page_url: String,
    pub vless_page_url: String,
    pub trojan_page_url: String,
    pub converter_page_url: String,
    pub ss_page_url: String,
    
}
