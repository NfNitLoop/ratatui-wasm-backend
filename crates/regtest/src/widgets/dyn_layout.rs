//! Custom widgets.
//!
//! Ratatui, is surprisngly low-level! Giving myself some nicer primitives to work with.

use std::borrow::Cow;

use ratatui_wasm_backend::ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Widget, WidgetRef},
};

use super::utils::ref_or_dyn::RefOrDyn;

/// Like [Layout], but allows you to dynamically add widgets to it.
/// Then you can .render() the layout itself to render all the added widgets.
pub struct DynLayout<'a> {
    layout: Layout,
    items: Vec<DynLayoutItem<'a>>,
}

impl<'a> From<Layout> for DynLayout<'a> {
    fn from(layout: Layout) -> Self {
        Self {
            layout,
            items: Default::default(),
        }
    }
}

impl<'a> DynLayout<'a> {
    pub fn add(&mut self, constraint: Constraint, widget: impl WidgetRef + Sized + 'a) {
        self.items.push(DynLayoutItem {
            constraint,
            widget: RefOrDyn::Owned(Box::new(widget)),
        });
    }

    #[allow(dead_code)]
    pub fn add_ref(&mut self, constraint: Constraint, widget: &'a dyn WidgetRef) {
        self.items.push(DynLayoutItem {
            constraint,
            widget: RefOrDyn::Ref(widget),
        })
    }
}

struct DynLayoutItem<'a> {
    constraint: Constraint,
    widget: RefOrDyn<'a, dyn WidgetRef + 'a>,
}

impl<'a> WidgetRef for DynLayout<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let constraints = self
            .items
            .iter()
            .map(|it| it.constraint.clone())
            .collect::<Vec<_>>();
        let rects = self.layout.clone().constraints(constraints).split(area);

        for (index, rect) in rects.iter().enumerate() {
            let widget = &self.items[index].widget;
            widget.as_ref().render_ref(*rect, buf);
        }
    }
}

pub trait ToDynLayout<'a> {
    fn dynamic(self) -> DynLayout<'a>;
}

impl<'a> ToDynLayout<'a> for Layout {
    fn dynamic(self) -> DynLayout<'a> {
        DynLayout::from(self)
    }
}
