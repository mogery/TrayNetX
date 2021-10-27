use terminal_clipboard;
use ksni;
use local_ip_address::local_ip;
use mac_address;
use reqwest::blocking::get;

#[derive(Debug)]
struct TrayNet {
    local_ip: String,
    mac_addr: String,
    public_ip: String
}

impl ksni::Tray for TrayNet {
    fn icon_name(&self) -> String {
        "applications-internet".into()
    }
    fn title(&self) -> String {
        "TrayNetX".into()
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            StandardItem {
                label: "Public IP: ".to_string() + &self.public_ip,
                activate: Box::new(|this: &mut Self| {
                    terminal_clipboard::set_string(this.public_ip.clone());
                }),
                ..Default::default()
            }.into(),
            StandardItem {
                label: "Local IP: ".to_string() + &self.local_ip,
                activate: Box::new(|this: &mut Self| {
                    terminal_clipboard::set_string(this.local_ip.clone());
                }),
                ..Default::default()
            }.into(),
            MenuItem::Separator,
            StandardItem {
                label: "MAC Address: ".to_string() + &self.mac_addr,
                activate: Box::new(|this: &mut Self| {
                    terminal_clipboard::set_string(this.mac_addr.clone());
                }),
                ..Default::default()
            }.into(),
            MenuItem::Separator,
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }.into(),
        ]
    }
}

fn generate_data() -> TrayNet {
    let public_ip = match match get("https://api.ipify.org/") {
        Ok(x) => x.text(),
        Err(_x) => Ok("Failed".to_string())
    } {
        Ok(x) => x,
        Err(_x) => "Failed".to_string()
    };

    TrayNet {
        local_ip: local_ip().unwrap().to_string(),
        mac_addr: mac_address::get_mac_address().unwrap().unwrap().to_string(),
        public_ip
    }
}

fn main() {
    let service = ksni::TrayService::new(generate_data());
    let handle = service.handle();
    service.spawn();

    loop {
        handle.update(|tray | {
            *tray = generate_data();
        });
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}