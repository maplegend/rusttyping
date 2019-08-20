use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::Entity;
use orbtk::{prelude::*, render::RenderContext2D, tree::Tree};
use orbtk::Layout;

use crate::attributed_text::attributed_text::*;

/// Fixed size layout is defined by fixed bounds like the size of an image or the size of a text.
#[derive(Default)]
pub struct AttributedTextLayout {
    desired_size: RefCell<DirtySize>,
    old_alignment: Cell<(Alignment, Alignment)>,
}

impl AttributedTextLayout {
    pub fn new() -> Self {
        AttributedTextLayout::default()
    }
}

impl Layout for AttributedTextLayout {
    fn measure(
        &self,
        render_context_2_d: &mut RenderContext2D,
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree>,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &ThemeValue,
    ) -> DirtySize {
        if Visibility::get(entity, ecm.component_store()) == VisibilityValue::Collapsed {
            self.desired_size.borrow_mut().set_size(0.0, 0.0);
            return self.desired_size.borrow().clone();
        }

        let horizontal_alignment = HorizontalAlignment::get(entity, ecm.component_store());
        let vertical_alignment = VerticalAlignment::get(entity, ecm.component_store());

        if horizontal_alignment != self.old_alignment.get().1
            || vertical_alignment != self.old_alignment.get().0
        {
            self.desired_size.borrow_mut().set_dirty(true);
        }

        let widget = WidgetContainer::new(entity, ecm);

        let size = widget.try_get::<AttributedText>().and_then(|text| {
                    let font = widget.get::<Font>();
                    let font_size = widget.get::<FontSize>();
                    render_context_2_d.set_font_size(font_size.0);
                    render_context_2_d.set_font_family(&font.0[..]);

                    let text_metrics =
                        render_context_2_d.measure_text(text.to_string().as_str());

                    let mut size = (text_metrics.width, text_metrics.height);

                    if text.to_string().ends_with(" ") {
                        size.0 += render_context_2_d
                            .measure_text(&format!("{}a", text.to_string()))
                            .width
                            - render_context_2_d.measure_text("a").width;
                    }
                    Some(size)

                });

        if let Some(size) = size {
            if let Ok(constraint) = ecm
                .component_store_mut()
                .borrow_mut_component::<Constraint>(entity)
            {
                constraint.set_width(size.0 as f64);
                constraint.set_height(size.1 as f64);
            }
        }

        // -- todo will be removed after orbgl merge --

        let constraint = Constraint::get(entity, ecm.component_store());

        if constraint.width() > 0.0 {
            self.desired_size.borrow_mut().set_width(constraint.width());
        }

        if constraint.height() > 0.0 {
            self.desired_size
                .borrow_mut()
                .set_height(constraint.height());
        }

        if ecm.entity_store().children[&entity].len() > 0 {
            let mut index = 0;

            loop {
                let child = ecm.entity_store().children[&entity][index];
                if let Some(child_layout) = layouts.borrow().get(&child) {
                    let dirty = child_layout
                        .measure(render_context_2_d, child, ecm, layouts, theme)
                        .dirty()
                        || self.desired_size.borrow().dirty();

                    self.desired_size.borrow_mut().set_dirty(dirty);
                }

                if index + 1 < ecm.entity_store().children[&entity].len() {
                    index += 1;
                } else {
                    break;
                }
            }
        }

        self.desired_size.borrow().clone()
    }

    fn arrange(
        &self,
        render_context_2_d: &mut RenderContext2D,
        _parent_size: (f64, f64),
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree>,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &ThemeValue,
    ) -> (f64, f64) {
        if !self.desired_size.borrow().dirty() {
            return self.desired_size.borrow().size();
        }
        println!("parent size {:?} desired size {} {}", _parent_size, self.desired_size.borrow().width(), self.desired_size.borrow().height());
        println!("calculated lines {} ", (self.desired_size.borrow().width()/_parent_size.0));
        if let Ok(bounds) = ecm
            .component_store_mut()
            .borrow_mut_component::<Bounds>(entity)
        {
            bounds.set_width(self.desired_size.borrow().width().min(_parent_size.0));
            bounds.set_height(self.desired_size.borrow().height()*(self.desired_size.borrow().width()/_parent_size.0).ceil());
        }

        if ecm.entity_store().children[&entity].len() > 0 {
            let mut index = 0;

            loop {
                let child = ecm.entity_store().children[&entity][index];
                if let Some(child_layout) = layouts.borrow().get(&child) {
                    child_layout.arrange(
                        render_context_2_d,
                        (
                            self.desired_size.borrow().width(),
                            self.desired_size.borrow().height(),
                        ),
                        child,
                        ecm,
                        layouts,
                        theme,
                    );
                }

                if index + 1 < ecm.entity_store().children[&entity].len() {
                    index += 1;
                } else {
                    break;
                }
            }
        }

        self.desired_size.borrow_mut().set_dirty(false);
        self.desired_size.borrow().size()
    }
}

impl Into<Box<dyn Layout>> for AttributedTextLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}
