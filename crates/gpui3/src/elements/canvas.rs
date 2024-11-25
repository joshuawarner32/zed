use refineable::Refineable as _;

use crate::{
    AppContext, Bounds, Element, ElementId, GlobalElementId, IntoElement, Pixels, Style,
    StyleRefinement, Styled,
};

/// Construct a canvas element with the given paint callback.
/// Useful for adding short term custom drawing to a view.
pub fn canvas<T>(
    prepaint: impl 'static + FnOnce(Bounds<Pixels>, &mut AppContext) -> T,
    paint: impl 'static + FnOnce(Bounds<Pixels>, T, &mut AppContext),
) -> Canvas<T> {
    Canvas {
        prepaint: Some(Box::new(prepaint)),
        paint: Some(Box::new(paint)),
        style: StyleRefinement::default(),
    }
}

/// A canvas element, meant for accessing the low level paint API without defining a whole
/// custom element
pub struct Canvas<T> {
    prepaint: Option<Box<dyn FnOnce(Bounds<Pixels>, &mut AppContext) -> T>>,
    paint: Option<Box<dyn FnOnce(Bounds<Pixels>, T, &mut AppContext)>>,
    style: StyleRefinement,
}

impl<T: 'static> IntoElement for Canvas<T> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl<T: 'static> Element for Canvas<T> {
    type RequestLayoutState = Style;
    type PrepaintState = Option<T>;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        cx: &mut AppContext,
    ) -> (crate::LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.refine(&self.style);
        let layout_id = cx.request_layout(style.clone(), []);
        (layout_id, style)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Style,
        cx: &mut AppContext,
    ) -> Option<T> {
        Some(self.prepaint.take().unwrap()(bounds, cx))
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        style: &mut Style,
        prepaint: &mut Self::PrepaintState,
        cx: &mut AppContext,
    ) {
        let prepaint = prepaint.take().unwrap();
        style.paint(bounds, cx, |cx| {
            (self.paint.take().unwrap())(bounds, prepaint, cx)
        });
    }
}

impl<T> Styled for Canvas<T> {
    fn style(&mut self) -> &mut crate::StyleRefinement {
        &mut self.style
    }
}
