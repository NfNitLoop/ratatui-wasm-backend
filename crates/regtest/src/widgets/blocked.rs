use ratatui::widgets::{Block, Borders, Widget, WidgetRef};

use super::utils::ref_or_dyn::RefOrDyn;

/// Surrounds any widget with a Block.
/// 
/// Vs. every widget having to accept Block.
pub struct Blocked<'a> {
    pub block: Block<'a>,
    pub widget: RefOrDyn<'a, dyn WidgetRef + 'a>
}


impl <'a> WidgetRef for Blocked<'a> {
    fn render_ref(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.block.render_ref(area, buf);
        let area = self.block.inner(area);
        self.widget.as_ref().render_ref(area, buf);
    }
}