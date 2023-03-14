use crate::ui::gui::GuiState;
use crate::ui::gui::Theme::{Classic, Dark, Light};
use crate::ui::open_in_app;
use crate::ui::popup::PopupMessage;
use crate::ui::window::Tool;

use imgui::{im_str, ImString, Io, MenuItem, Ui};

pub struct HelpTool {
    text: &'static str,
}

impl HelpTool {
    pub fn new(text: &'static str) -> HelpTool {
        HelpTool { text }
    }
}

impl Tool for HelpTool {
    fn draw(&mut self, ui: &Ui, _io: &Io, state: &mut GuiState) {
        ui.menu_bar(|| {
            ui.menu(im_str!("Полезные ссылочки"), true, || {
                for (name, url) in [
                    (im_str!("GitHub"), "https://github.com/JustAGod1/bevm"),
                    (im_str!("Telegram"), "https://t.me/notsofunnyhere"),
                    (im_str!("Методичка"), "https://yadi.sk/i/brIICpYtcb3LMg"),
                    (im_str!("Моя телега"), "https://t.me/JustAG0d"),
                ] {
                    if MenuItem::new(name).build(ui) {
                        if let Err(e) = open_in_app(url) {
                            state.popup_manager.open(PopupMessage::new(
                                "Ошибочка",
                                format!("Не смог открыть ссылку: {}", e),
                            ))
                        }
                        return;
                    }
                }
                if ui.is_item_hovered() {
                    ui.tooltip_text(
                        "Мне желательно писать по поводу идей для новых фич для этой БЭВМ.\n\n\
                    Желательно придерживаться правил общения описанных на nometa.xyz.",
                    )
                }
            });
            ui.menu(im_str!("Оформление"), true, || {
                if MenuItem::new(im_str!("Темное")).build(ui) {
                    state.theme_requested = Some(Dark)
                }
                if MenuItem::new(im_str!("Светлое")).build(ui) {
                    state.theme_requested = Some(Light)
                }
                if MenuItem::new(im_str!("Классическое")).build(ui) {
                    state.theme_requested = Some(Classic)
                }
                if MenuItem::new(im_str!("Редактор"))
                    .selected(state.editor_enabled)
                    .build(ui)
                {
                    state.editor_enabled = !state.editor_enabled
                }
            })
        });
        ui.text_wrapped(ImString::new(self.text).as_ref());
    }
}
