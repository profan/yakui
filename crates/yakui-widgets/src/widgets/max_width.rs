use std::f32::INFINITY;

use yakui_core::geometry::{Constraints, Vec2};
use yakui_core::widget::{LayoutContext, Widget};
use yakui_core::Response;

use crate::util::widget_children;

/**
A box that enforces a maximum width upon its children.

Responds with [MaxWidthResponse].
*/
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MaxWidth {
    pub max_width: f32,
}

impl MaxWidth {
    pub fn new(max_width: f32) -> Self {
        Self { max_width }
    }

    pub fn show<F: FnOnce()>(self, children: F) -> Response<MaxWidthWidget> {
        widget_children::<MaxWidthWidget, F>(children, self)
    }
}

#[derive(Debug)]
pub struct MaxWidthWidget {
    props: MaxWidth,
}

pub type MaxWidthResponse = ();

impl Widget for MaxWidthWidget {
    type Props = MaxWidth;
    type Response = MaxWidthResponse;

    fn new() -> Self {
        Self {
            props: MaxWidth {
                max_width: INFINITY,
            },
        }
    }

    fn update(&mut self, props: Self::Props) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, mut ctx: LayoutContext<'_>, mut constraints: Constraints) -> Vec2 {
        let node = ctx.dom.get_current();
        let mut size = Vec2::ZERO;
        constraints.max.x = constraints.max.x.min(self.props.max_width);

        for &child in &node.children {
            let child_size = ctx.calculate_layout(child, constraints);
            size = size.max(child_size);
        }

        size
    }
}
