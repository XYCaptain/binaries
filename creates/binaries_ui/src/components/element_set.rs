use bevy::utils::all_tuples;

use super::element::Element;
use crate::traits::UIElement;
use crate::components::stacks::Stack;

pub trait ElementSet {
    fn foreach_view<F: FnMut(Box<dyn UIElement>)>(&self, f: &mut F);
    fn is_empty(&self) -> bool {
        false
    }
}

impl ElementSet for Element
{
    fn foreach_view<FN: FnMut(Box<dyn UIElement>)>(&self, f: &mut FN) {
        f(Box::new(self.clone()) as Box<dyn UIElement>);
    }
}

impl<K> ElementSet for Stack<K>
where  K: ElementSet+ Clone + Send + Sync + 'static 
{
    fn foreach_view<FN: FnMut(Box<dyn UIElement>)>(&self, f: &mut FN) {
        f(Box::new(self.clone()) as Box<dyn UIElement>);
    }
}

impl<T> ElementSet for Vec<T> 
where T: UIElement + Clone
{
    fn foreach_view<FN: FnMut(Box<dyn UIElement>)>(&self, f: &mut FN) {
        for element in self{
            f(Box::new(element.clone()) as Box<dyn UIElement>);
        }
    }
}

macro_rules! impl_view_tuples{
    ($($element:ident),*) => {
        impl<$($element),*> ElementSet for ($($element,)*)
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