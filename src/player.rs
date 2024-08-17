use adw::prelude::*;
use relm4::prelude::*;

#[derive(Default)]
pub struct Player {
    path: Option<std::path::PathBuf>,
    playing: bool,
}

pub struct PlayerWidgets {
    player: gtk::Video,
}

#[derive(Debug)]
pub enum PlayerMsg {
    SetVideo(std::path::PathBuf),
    PlayPause,
}

impl SimpleComponent for Player {
    type Init = ();
    type Input = PlayerMsg;
    type Output = ();
    type Root = gtk::Box;
    type Widgets = PlayerWidgets;

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            playing: false,
            path: None,
        };

        let player = gtk::Video::builder().vexpand(true).hexpand(true).build();
        root.append(&player);

        let widgets = PlayerWidgets { player };

        ComponentParts { model, widgets }
    }

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build()
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        if widgets.player.file().is_none() {
            widgets.player.set_filename(self.path.as_ref());
        }
        if let Some(stream) = widgets.player.media_stream() {
            stream.set_playing(self.playing);
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            PlayerMsg::SetVideo(path) => {
                self.path = Some(path);
            }
            PlayerMsg::PlayPause => {
                self.playing = !self.playing;
            }
        }
    }
}
