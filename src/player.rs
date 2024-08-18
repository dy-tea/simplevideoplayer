use adw::prelude::*;
use relm4::prelude::*;

#[derive(Default)]
pub struct Player {
    path: Option<std::path::PathBuf>,
    playing: bool,
    seek: i8,
    volume: i8,
}

pub struct PlayerWidgets {
    player: gtk::Video,
}

#[derive(Debug)]
pub enum PlayerMsg {
    SetVideo(std::path::PathBuf),
    PlayPause,
    SeekForwards,
    SeekBackwards,
    VolumeUp,
    VolumeDown,
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
            seek: 1,
            volume: 1,
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
            if self.seek != 1 {
                stream.seek(stream.timestamp() + (10000000 * (self.seek - 1) as i64));
            }
            stream.set_volume(stream.volume() + (10.0 * (self.volume - 1) as f64));
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        self.seek = 1;
        match msg {
            PlayerMsg::SetVideo(path) => {
                self.path = Some(path);
            }
            PlayerMsg::PlayPause => {
                self.playing = !self.playing;
            }
            PlayerMsg::SeekForwards => {
                self.seek = 2;
            }
            PlayerMsg::SeekBackwards => {
                self.seek = 0;
            }
            PlayerMsg::VolumeUp => {
                self.volume = 2;
            }
            PlayerMsg::VolumeDown => {
                self.volume = 0;
            }
        }
    }
}
