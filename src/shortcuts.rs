use adw::prelude::*;
use gtk::prelude::*;
use relm4::prelude::*;

use crate::*;

#[derive(Debug)]
pub struct Shortcuts {
    visible: bool,
}

#[derive(Debug)]
pub enum ShortcutsMsg {
    Show,
    Hide,
}

#[relm4::component(pub)]
impl SimpleComponent for Shortcuts {
    type Init = ();
    type Input = ShortcutsMsg;
    type Output = ();

    view! {
        adw::Window {
            set_title: Some("Shortcuts"),
            #[watch]
            set_visible: model.visible,
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                gtk::HeaderBar,
            },
            connect_close_request[sender] => move |_| {
                sender.input(ShortcutsMsg::Hide);
                gtk::glib::Propagation::Proceed
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { visible: false };

        let widgets = view_output!();

        ComponentParts { model, widgets }
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
