//! TV Studio в графическом окне — запуск среды.
//!
//! `tvstudio-gui [проект.tvsp | каталог проекта | файлы...]`. Без
//! аргументов открывается последний проект.
//!
//! Вся среда — обычная программа Turbo Vision (tvision-rs); окно ОС,
//! шрифт, темы и мышь даёт tvision-gui. Собранная без особенности `gui`
//! (`cargo run --no-default-features`), та же среда работает в терминале.

// В выпускной сборке на Windows — без окна консоли.
#![cfg_attr(all(feature = "gui", not(debug_assertions)), windows_subsystem = "windows")]

use tvision::*;
use tvstudio::dialogs::Config;
use tvstudio::ide::{menu_bar, status_line, Ide};

/// Среда в окне: шрифт и тема — стандартным диалогом tvision-gui,
/// заголовок — в заголовке окна ОС.
#[cfg(feature = "gui")]
struct GuiHost;

#[cfg(feature = "gui")]
impl tvstudio::host::Host for GuiHost {
    fn graphical(&self) -> bool {
        true
    }
    fn settings_dialog(&mut self, app: &mut Program) -> bool {
        tvision_gui::settings_dialog(app)
    }
    fn set_title(&mut self, title: &str) {
        tvision_gui::set_title(title);
    }
}

/// Сама среда (в окне — в отдельном потоке, окно живёт в главном).
fn app_main() -> std::io::Result<()> {
    // Язык надписей хранится отдельно для каждого потока — ставим в потоке программы.
    locale::set_language(locale::Language::Ru);
    #[cfg(feature = "gui")]
    let mut app = {
        use tvision_gui::ProgramExt;
        Program::with_window(Some(menu_bar()), Some(status_line()))?
    };
    #[cfg(not(feature = "gui"))]
    let mut app = Program::with_terminal(Some(menu_bar()), Some(status_line()))?;
    let mut ide = Ide::new(Config::load(), false);
    #[cfg(feature = "gui")]
    {
        ide.host = Box::new(GuiHost);
    }
    ide.startup(&mut app, std::env::args().skip(1).collect());
    ide.update_title();
    app.run(&mut ide);
    ide.shutdown(&mut app);
    Ok(())
}

#[cfg(feature = "gui")]
fn main() -> std::io::Result<()> {
    locale::set_language(locale::Language::Ru);
    // Окно 132×43 символа; тема, шрифт, размер и положение окна
    // запоминаются между запусками (файл tvstudio-gui.conf).
    let cfg = tvision_gui::GuiConfig::new("TV Studio").size(132, 43).remember_settings(true);
    tvision_gui::run(cfg, app_main)
}

#[cfg(not(feature = "gui"))]
fn main() -> std::io::Result<()> {
    app_main()
}
