use taffy::NodeId;

use super::element::Element;


fn binding()->Element{
    Element::new()
}

struct BindingObject{
    //data object
    data_object:BindingData,
    data_id:NodeId
    //layout nodeid
    //fn data to element
}

impl BindingObject{
    fn get_element(){}
}

struct BindingCollection {
    parent:NodeId,
    binding_objects: Vec<BindingObject>
}

enum BindingData {
    U32(u32),
    C32(Vec::<u32>)
}

struct Field<T>{
    value: T,
    element:Element
}

impl<T> Field<T> {
   fn get(){}
   fn set(){}
   fn changed(){}
   fn is_none(){}
}