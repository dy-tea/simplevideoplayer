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
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { visible: false };

        let group1 = gtk::ShortcutsGroup::builder()
            .title("Player")
            .name("player")
            .build();

        group1.append(
            &gtk::ShortcutsShortcut::builder()
                .title("Play/Pause")
                .name("playpause")
                .action_name("playpause")
                .accelerator("space")
                .build(),
        );
        group1.append(
            &gtk::ShortcutsShortcut::builder()
                .title("Fullscreen")
                .name("fullscreen")
                .action_name("fullscreen")
                .accelerator("F")
                .build(),
        );
        group1.append(
            &gtk::ShortcutsShortcut::builder()
                .title("Seek +10s")
                .name("seekforwards")
                .action_name("seekforwards")
                .accelerator("Right")
                .build(),
        );
        group1.append(
            &gtk::ShortcutsShortcut::builder()
                .title("Seek -10s")
                .name("seekbackwards")
                .action_name("seekbackwards")
                .accelerator("Left")
                .build(),
        );

        let group2 = gtk::ShortcutsGroup::builder()
            .title("General")
            .name("general")
            .build();

        group2.append(
            &gtk::ShortcutsShortcut::builder()
                .title("Open")
                .name("open")
                .action_name("open")
                .accelerator("<Ctrl>O")
                .build(),
        );
        group2.append(
            &gtk::ShortcutsShortcut::builder()
                .title("Media Info")
                .name("info")
                .action_name("info")
                .accelerator("<Ctrl>I")
                .build(),
        );
        group2.append(
            &gtk::ShortcutsShortcut::builder()
                .title("About")
                .name("about")
                .action_name("about")
                .accelerator("<Ctrl>A")
                .build(),
        );
        group2.append(
            &gtk::ShortcutsShortcut::builder()
                .title("Shortcuts")
                .name("shortcuts")
                .action_name("shortcuts")
                .accelerator("<Ctrl>question")
                .build(),
        );

        let container = gtk::ShortcutsSection::builder()
            .orientation(gtk::Orientation::Horizontal)
            .section_name("shortcuts")
            .build();

        container.append(&group1);
        container.append(&group2);

        root.set_child(Some(&container));

        let widgets = ShortcutsWidgets {
            window: root.to_owned(),
        };

        root.connect_close_request(move |_| {
            sender.input(ShortcutsMsg::Hide);
            gtk::glib::Propagation::Proceed
        });

        ComponentParts { model, widgets }
    }

    fn init_root() -> Self::Root {
        gtk::ShortcutsWindow::builder().modal(true).build()
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
