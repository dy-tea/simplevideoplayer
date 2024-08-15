use adw::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};

use ffmpeg_next::format;

use crate::AppMsg;

#[derive(Debug)]
struct Metadata {
    key: String,
    value: String,
}

#[relm4::factory]
impl FactoryComponent for Metadata {
    type Init = Metadata;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = adw::PreferencesGroup;

    view! {
        #[root]
        root = adw::ActionRow {
            #[watch]
            set_title: &self.key,
            add_suffix = &gtk::Label {
                #[watch]
                set_text: &self.value
            }
        },
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            key: init.key,
            value: init.value,
        }
    }
}

pub struct MediaInfoWindow {
    format: Option<String>,
    duration: Option<String>,
    bitrate: Option<String>,
    metadata: FactoryVecDeque<Metadata>,
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
                    #[local_ref]
                    metadata_view -> adw::PreferencesGroup {
                        set_title: "Metadata"
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
        let metadata = FactoryVecDeque::builder()
            .launch(adw::PreferencesGroup::new())
            .detach();

        let model = Self {
            format: None,
            duration: None,
            bitrate: None,
            metadata,
        };

        let metadata_view = model.metadata.widget();
        let widgets = view_output!();
        widgets.window.set_transient_for(Some(&init));

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        let mut metadata_guard = self.metadata.guard();

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

                for (a, b) in context.metadata().iter() {
                    let first_char = a.chars().next().unwrap_or(' ').to_uppercase().to_string();
                    metadata_guard.push_back(Metadata {
                        key: format! {"{}{}", first_char, &a[1..].to_lowercase()},
                        value: b.to_string(),
                    });
                }
            }
        }
    }
}
