use adw::prelude::*;
use relm4::{
    actions::{RelmAction, RelmActionGroup},
    prelude::*,
};

pub mod about;
use about::{AboutDialog, AboutDialogMsg};

pub static mut ABOUT_DIALOG: Option<Controller<AboutDialog>> = None;

struct App {
    file: Option<String>,
}

#[derive(Debug)]
enum AppMsg {
    SelectFile,
}

relm4::new_action_group!(WindowActionGroup, "win");

relm4::new_stateless_action!(MediaInfo, WindowActionGroup, "media_info");
relm4::new_stateless_action!(About, WindowActionGroup, "about");

#[relm4::component(async)]
impl AsyncComponent for App {
    type Input = AppMsg;
    type Output = ();
    type Init = u8;
    type CommandOutput = ();

    menu! {
        main_menu: {
            section! {
                "Media Info" => MediaInfo,
                "About" => About,
            },
        }
    }

    view! {
        window = adw::Window {
            set_title: Some("Video Player"),
            set_default_width: 800,
            set_default_height: 450,
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                adw::HeaderBar {
                    pack_start = &gtk::Button {
                        set_label: "Open",
                        connect_clicked[sender] => move |_| {
                            sender.input(AppMsg::SelectFile);
                        },
                    },
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&main_menu),
                    }
                },
                gtk::Video {
                    set_autoplay: true,
                    set_hexpand: true,
                    set_vexpand: true,
                    #[watch]
                    set_filename: model.file.as_ref(),
                },
            }
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = App { file: None };

        let widgets = view_output!();

        let about_dialog_broker: relm4::MessageBroker<AboutDialogMsg> = relm4::MessageBroker::new();

        unsafe {
            ABOUT_DIALOG = Some(
                AboutDialog::builder()
                    .transient_for(widgets.window.clone())
                    .launch_with_broker((), &about_dialog_broker)
                    .detach(),
            );
        }

        let mut group = RelmActionGroup::<WindowActionGroup>::new();

        group.add_action::<About>(RelmAction::new_stateless(move |_| {
            about_dialog_broker.send(AboutDialogMsg::Show);
        }));

        widgets
            .window
            .insert_action_group("win", Some(&group.into_action_group()));

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: AppMsg, sender: AsyncComponentSender<Self>, root: &Self::Root) {
        match msg {
            AppMsg::SelectFile => {
                let dialog = rfd::AsyncFileDialog::new()
                    .add_filter("Video", &["mp4", "mkv"])
                    .pick_file();
                if let Some(path) = dialog.await {
                    self.file = Some(path.path().display().to_string());
                }
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.video.player");
    app.run_async::<App>(0);
}
