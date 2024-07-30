use bevy::utils::all_tuples;
use crate::layout::SDUILayouts;
use crate::traits::UIElement;

pub fn stack(
    children: impl ElementTuple
) -> Stack<impl ElementTuple> {
    Stack::new(children)
}

pub struct Stack<K> where K:ElementTuple {
   children: K,
}

impl<K:ElementTuple> Stack<K> {
    pub fn new(children: K) -> Self {
        Self {
            children,
        }
    }
    
    pub fn push_to_layout(&mut self, layout: &mut SDUILayouts) {
        self.children.foreach_view(&mut |element| {
            layout.push_element(element);
        });
    }
}

pub trait ElementTuple {
    fn foreach_view<F: FnMut(Box<dyn UIElement>)>(&self, f: &mut F);
    fn is_empty(&self) -> bool {
        false
    }
}

macro_rules! impl_view_tuples{
    ($($element:ident),*) => {
        impl<$($element),*> ElementTuple for ($($element,)*) 
        where 
            $($element: UIElement + Clone + 'static),*
        {
            #[allow(non_snake_case, unused_variables)]
            #[track_caller]
            fn foreach_view<FN: FnMut(Box<dyn UIElement>)>(&self, f: &mut FN) {
                let ($($element,)*) = self;
                $(f(Box::new($element.clone()) as Box<dyn UIElement>);)*
            }
        }
    }
}

all_tuples!(impl_view_tuples, 0, 128, T);