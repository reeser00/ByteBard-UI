use egui::{menu, Button, CentralPanel, Context, Key, Label, RawInput, ScrollArea, SidePanel, TopBottomPanel, Ui, Vec2, Visuals, Widget, Window};
use tokio::sync::mpsc;
use std::{process, sync::{Arc, Mutex}};
use proxy::Proxy;
pub struct App {
    packets: Vec<Vec<u8>>,
    packet_transmitter: Arc<Mutex<mpsc::Sender<Vec<u8>>>>,
    packet_receiver: Arc<Mutex<mpsc::Receiver<Vec<u8>>>>,
    home_addr: String,
    server_addr: String,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        let (tx, rx) = mpsc::channel::<Vec<u8>>(100);
        Self {
            packets: vec![],
            packet_transmitter: Arc::new(Mutex::new(tx)),
            packet_receiver: Arc::new(Mutex::new(rx)),
            home_addr: String::from("127.0.0.1:15201"),
            server_addr: String::from("85.17.202.49:15201"),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel_0").min_height(100.0).show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("Proxy", |_ui| {
                    
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Cut").clicked() {
                        println!("Cut clicked");
                    }
                    if ui.button("Copy").clicked() {
                        println!("Copy clicked");
                    }
                    if ui.button("Paste").clicked() {
                        println!("Paste clicked");
                    }
                });
            });
            let _home_addr = ui.add_enabled(true, |ui: &mut Ui| {
                ui.add_sized([150.0, 15.0],egui::TextEdit::singleline(&mut self.home_addr)
                .hint_text("Enter Home Adress"))
            });
            let _server_addr = ui.add_enabled(true, |ui: &mut Ui| {
                ui.add_sized([150.0, 15.0],egui::TextEdit::singleline(&mut self.server_addr)
                .hint_text("Enter Server Adress"))
            });
            if ui.add(Button::new("Start Proxy")).clicked() {
                let tx = Arc::clone(&self.packet_transmitter);
                let home_addr_string = self.home_addr.clone();
                let server_addr_string = self.server_addr.clone();
                tokio::spawn( async move {
                    new_proxy(home_addr_string, server_addr_string, tx).await;
                });
            }
            
            
        });
        SidePanel::left("left_panel_0").min_width(300.0).show(ctx, |ui| {
            ui.label("Side Panel Left #1");
        });
        SidePanel::right("right_panel_0").min_width(300.0).show(ctx, |ui| {
            ui.label("Side Panel Right #1");
        });
        TopBottomPanel::bottom("bottom_panel_0").show(ctx, |ui| {
            ui.label("Bottom Panel #1");
        });
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            
            match self.packet_receiver.lock() {
                Ok(mut sender) => {
                    match sender.try_recv() {
                        Ok(buffer) => {
                            self.packets.push(buffer);
    
                        },
                        Err(e) => {
                            if e == mpsc::error::TryRecvError::Empty {
                            } else {
                                eprintln!("Error reading from additional client rx: {}", e);
                            }
                        }
                    }
                }
                Err(_) => {
                    eprintln!("Failed to acquire lock on packet_transmitter");
                }
            }

            ui.ctx().request_repaint();
            ScrollArea::vertical().auto_shrink(false).stick_to_bottom(true).show(ui, |ui| {
                for packet in &self.packets {
                    let string: String = packet.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" ");
                    ui.label(string);
                }
            });
            if ctx.input(|i| i.key_released(Key::C)) {
                self.packets.clear();
            } else if ctx.input(|i| i.key_down(Key::S)) {
                self.packets.push(vec![48, 75, 44, 33, 47, 88, 95, 18, 23]);
            }
        });
    }

}

impl App {
    fn new_proxy_window(&mut self, ctx: &Context) {
        Window::new("Window").show(&ctx, |ui| {
            ui.add(Label::new("A small window"));
        });
    }
}

async fn new_proxy(home_addr: String, server_addr: String, packet_transmitter: Arc<Mutex<mpsc::Sender<Vec<u8>>>>) {
    let mut my_proxy = Proxy::from(home_addr, server_addr, packet_transmitter).await.unwrap();
    my_proxy.start().await;
}




