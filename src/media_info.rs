use std::collections::HashMap;

use adw::prelude::*;
use gtk::prelude::*;
use relm4::prelude::*;

use ffmpeg_next::format;

use crate::AppMsg;

pub struct MediaInfoWindow {
    format: Option<String>,
    duration: Option<String>,
    bitrate: Option<String>,
    metadata: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub enum MediaInfoMsg {
    GetInfo(std::path::PathBuf),
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for MediaInfoWindow {
    type Init = adw::Window;
    type Input = MediaInfoMsg;
    type Output = AppMsg;

    view! {
        #[root]
        window = adw::Window {
            set_title: Some("Media Info"),
            set_default_size: (600, 450),
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                adw::HeaderBar,
                adw::PreferencesPage {
                    add = &adw::PreferencesGroup {
                        set_title: "Container",
                        adw::ActionRow {
                            set_title: "Format",
                            add_suffix = &gtk::Label {
                                #[watch]
                                set_text: model.format.clone().unwrap_or(String::from("N/A")).as_ref(),
                            }
                        },
                        adw::ActionRow {
                            set_title: "Duration",
                            add_suffix = &gtk::Label {
                                #[watch]
                                set_text: model.duration.clone().unwrap_or(String::from("N/A")).as_ref(),
                            }
                        },
                        adw::ActionRow {
                            set_title: "Bitrate",
                            add_suffix = &gtk::Label {
                                #[watch]
                                set_text: model.bitrate.clone().unwrap_or(String::from("N/A")).as_ref(),
                            }
                        }
                    },
                    #[name = "metadata"]
                    add = &adw::PreferencesGroup {
                        set_title: "Metadata",
                    }
                }
            },
            connect_close_request[sender] => move |_̱| {
                let _ = sender.output(AppMsg::CloseMediaInfo);
                gtk::glib::Propagation::Proceed
            }
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = Self {
            format: None,
            duration: None,
            bitrate: None,
            metadata: None,
        };

        let widgets = view_output!();

        widgets.window.set_transient_for(Some(&init));

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            MediaInfoMsg::GetInfo(path) => {
                let _ = ffmpeg_next::init();
                let context = match format::input(&path) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Error: Could not create context ({e})");
                        panic!();
                    }
                };

                self.format = Some(format!("{}", context.format().description()));

                let seconds = context.duration() as f64 / 1_000_000.0;
                let duration = format!(
                    "{:02}:{:02}:{:02}",
                    (seconds / 3600.0) as u64 % 60,
                    (seconds / 60.0) as u64 % 60,
                    seconds as u64 % 60
                );
                self.duration = Some(duration);

                self.bitrate = Some(format!(
                    "{:.2} Mbps",
                    context.bit_rate() as f64 / 1_000_000.0
                ));

                let mut map: HashMap<String, String> = HashMap::new();
                for (a, b) in context.metadata().iter() {
                    map.insert(a.to_string(), b.to_string());
                }
                self.metadata = Some(map);
            }
        }
    }
}
