pub enum Allocator<T>
where
    T : datatype,
{
    Stack = 0,
    Heap,
    Custom
}

impl<T> Allocator<T>
where 
    T : datatype,
{
    fn new() {

    }
    fn delete() {
        
    }
}