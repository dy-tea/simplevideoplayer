use adw::prelude::*;
use gtk::glib::clone;
use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    prelude::*,
};

pub mod player;
use player::{Player, PlayerMsg};

pub mod media_info;
use media_info::{MediaInfoMsg, MediaInfoWindow};

pub mod about;
use about::{AboutDialog, AboutDialogMsg};

pub mod shortcuts;
use shortcuts::{Shortcuts, ShortcutsMsg};

struct App {
    file: Option<String>,
    player: Controller<Player>,
    media_info_window: AsyncController<MediaInfoWindow>,
    about_dialog: Controller<AboutDialog>,
    shortcuts_window: Controller<Shortcuts>,
}

#[derive(Debug)]
pub enum AppMsg {
    SelectFile,
    OpenMediaInfo,
    PlayPause,
    Fullscreen,
    SeekForwards,
    SeekBackwards,
    Quit,
}

relm4::new_action_group!(WindowActionGroup, "win");

relm4::new_stateless_action!(Open, WindowActionGroup, "open");
relm4::new_stateless_action!(About, WindowActionGroup, "about");
relm4::new_stateless_action!(Info, WindowActionGroup, "mediainfo");
relm4::new_stateless_action!(Shortcut, WindowActionGroup, "shortcuts");
relm4::new_stateless_action!(Quit, WindowActionGroup, "quit");
relm4::new_stateless_action!(PlayPause, WindowActionGroup, "playpause");
relm4::new_stateless_action!(Fullscreen, WindowActionGroup, "fullscreen");
relm4::new_stateless_action!(SeekForwards, WindowActionGroup, "seekforwards");
relm4::new_stateless_action!(SeekBackwards, WindowActionGroup, "seekbackwards");

#[relm4::component(async)]
impl AsyncComponent for App {
    type Input = AppMsg;
    type Output = ();
    type Init = u8;
    type CommandOutput = ();

    menu! {
        main_menu: {
            section! {
                "About" => About,
                "Keyboard Shortcuts" => Shortcut
            },
        }
    }

    view! {
        window = adw::Window {
            set_title: Some("Simple Video Player"),
            set_default_width: 800,
            set_default_height: 450,
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                adw::HeaderBar {
                    pack_start = &gtk::Button {
                        set_label: "Open",
                        connect_clicked => AppMsg::SelectFile,
                    },
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&main_menu),
                    },
                    pack_end = &gtk::Button {
                        set_icon_name: "documentinfo-symbolic",
                        set_tooltip_text: Some("Media Info"),
                        #[watch]
                        set_sensitive: model.file.is_some(),
                        connect_clicked => AppMsg::OpenMediaInfo,
                    },
                },
                #[local]
                player_box -> gtk::Box {},
            }
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let media_info_broker: relm4::MessageBroker<MediaInfoMsg> = relm4::MessageBroker::new();
        let about_dialog_broker: relm4::MessageBroker<AboutDialogMsg> = relm4::MessageBroker::new();
        let shortcuts_broker: relm4::MessageBroker<ShortcutsMsg> = relm4::MessageBroker::new();

        let model = App {
            file: None,
            player: Player::builder().launch(()).detach(),
            media_info_window: MediaInfoWindow::builder()
                .transient_for(root.clone())
                .launch_with_broker(root.clone(), &media_info_broker)
                .detach(),
            about_dialog: AboutDialog::builder()
                .transient_for(root.clone())
                .launch_with_broker((), &about_dialog_broker)
                .detach(),
            shortcuts_window: Shortcuts::builder()
                .transient_for(root.clone())
                .launch_with_broker((), &shortcuts_broker)
                .detach(),
        };

        let player_box = model.player.widget().to_owned();
        let widgets = view_output!();

        let app = relm4::main_application();
        let mut group = RelmActionGroup::<WindowActionGroup>::new();

        app.set_accelerators_for_action::<Open>(&["<Ctrl>O"]);
        app.set_accelerators_for_action::<About>(&["<Ctrl>A"]);
        app.set_accelerators_for_action::<Shortcut>(&["<Ctrl>question"]);
        app.set_accelerators_for_action::<Info>(&["<Ctrl>I"]);
        app.set_accelerators_for_action::<Quit>(&["<Ctrl>Q"]);
        app.set_accelerators_for_action::<PlayPause>(&["space"]);
        app.set_accelerators_for_action::<Fullscreen>(&["F"]);
        app.set_accelerators_for_action::<SeekForwards>(&["Right"]);
        app.set_accelerators_for_action::<SeekBackwards>(&["Left"]);

        group.add_action::<Open>(RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(AppMsg::SelectFile);
            }
        )));

        group.add_action::<About>(RelmAction::new_stateless(move |_| {
            about_dialog_broker.send(AboutDialogMsg::Show);
        }));

        group.add_action::<Info>(RelmAction::new_stateless(move |_| {
            media_info_broker.send(MediaInfoMsg::Show);
        }));

        group.add_action::<Shortcut>(RelmAction::new_stateless(move |_| {
            shortcuts_broker.send(ShortcutsMsg::Show);
        }));

        group.add_action::<Quit>(RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(AppMsg::Quit);
            }
        )));

        group.add_action::<PlayPause>(RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(AppMsg::PlayPause);
            }
        )));
        group.add_action::<Fullscreen>(RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(AppMsg::Fullscreen);
            }
        )));
        group.add_action::<SeekForwards>(RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(AppMsg::SeekForwards);
            }
        )));
        group.add_action::<SeekBackwards>(RelmAction::new_stateless(move |_| {
            sender.input(AppMsg::SeekBackwards);
        }));
        widgets
            .window
            .insert_action_group("win", Some(&group.into_action_group()));

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: AppMsg,
        _sender: AsyncComponentSender<Self>,
        root: &Self::Root,
    ) {
        match msg {
            AppMsg::SelectFile => {
                let dialog = rfd::AsyncFileDialog::new()
                    .set_title("Select a Video")
                    .add_filter(
                        "Video",
                        &[
                            "mp4", "mkv", "mka", "mk3d", "mks", "mov", "avi", "wmv", "flv", "f4v",
                            "webm", "ogv",
                        ],
                    )
                    .pick_file();
                if let Some(path) = dialog.await {
                    self.file = Some(path.path().display().to_string());
                    self.player
                        .sender()
                        .send(PlayerMsg::SetVideo(path.path().to_path_buf()))
                        .unwrap();
                    self.media_info_window
                        .sender()
                        .send(MediaInfoMsg::GetInfo(path.path().to_path_buf()))
                        .unwrap();
                }
            }
            AppMsg::OpenMediaInfo => {
                self.media_info_window
                    .sender()
                    .send(MediaInfoMsg::Show)
                    .unwrap();
            }
            AppMsg::PlayPause => {
                self.player.sender().send(PlayerMsg::PlayPause).unwrap();
            }
            AppMsg::Fullscreen => {
                root.set_fullscreened(!root.is_fullscreen());
            }
            AppMsg::SeekForwards => {
                self.player.sender().send(PlayerMsg::SeekForwards).unwrap();
            }
            AppMsg::SeekBackwards => {
                self.player.sender().send(PlayerMsg::SeekBackwards).unwrap();
            }
            AppMsg::Quit => {
                relm4::main_application().quit();
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("dy-tea.simplevideo.player");
    app.run_async::<App>(0);
}
