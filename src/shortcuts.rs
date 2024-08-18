use adw::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
pub struct Shortcuts {
    visible: bool,
}

#[derive(Debug)]
pub enum ShortcutsMsg {
    Show,
    Hide,
}

pub struct ShortcutsWidgets {
    window: gtk::ShortcutsWindow,
}

impl SimpleComponent for Shortcuts {
    type Init = ();
    type Input = ShortcutsMsg;
    type Output = ();
    type Root = gtk::ShortcutsWindow;
    type Widgets = ShortcutsWidgets;

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { visible: false };

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .margin_start(20)
            .margin_end(20)
            .margin_top(5)
            .margin_bottom(5)
            .build();

        let group1 = gtk::ShortcutsGroup::builder()
            .title("Video")
            .name("video")
            .build();
        group1.append(
            &gtk::ShortcutsShortcut::builder()
                .title("Play/Pause")
                .name("playpause")
                .action_name("playpause")
                .accelerator("space")
                .build(),
        );

        container.append(&group1);

        root.set_child(Some(&container));

        let widgets = ShortcutsWidgets { window: root };

        ComponentParts { model, widgets }
    }

    fn init_root() -> Self::Root {
        gtk::ShortcutsWindow::builder()
            .default_width(600)
            .default_height(450)
            .build()
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        widgets.window.set_visible(self.visible);
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ShortcutsMsg::Show => {
                self.visible = true;
            }
            ShortcutsMsg::Hide => {
                self.visible = false;
            }
        }
    }
}
