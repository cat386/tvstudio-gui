//! Где работает сама среда: в терминале или в графическом окне.
//!
//! Вся логика среды (`ide`, дизайнер, редактор, LSP...) одинакова и не
//! зависит от tvision-gui — её можно проверять тестами на «невидимом»
//! терминале. То немногое, что умеет только окно (выбор шрифта и темы,
//! заголовок окна ОС), среда просит у [`Host`]; оконную реализацию даёт
//! `main.rs` (с tvision-gui), терминальную — [`TerminalHost`].

use tvision::Program;

/// Возможности «хозяина» среды.
pub trait Host {
    /// Среда в графическом окне (своего терминала нет — программы для
    /// терминала запускаются во внешнем окне терминала ОС).
    fn graphical(&self) -> bool;
    /// Диалог «Шрифт и оформление» (тема, шрифт, размер). `true` — выбор принят.
    fn settings_dialog(&mut self, app: &mut Program) -> bool;
    /// Заголовок окна ОС.
    fn set_title(&mut self, title: &str);
}

/// Среда в терминале: шрифта и заголовка окна нет.
pub struct TerminalHost;

impl Host for TerminalHost {
    fn graphical(&self) -> bool {
        false
    }
    fn settings_dialog(&mut self, _app: &mut Program) -> bool {
        false
    }
    fn set_title(&mut self, _title: &str) {}
}
