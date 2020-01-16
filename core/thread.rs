use std::thread;
use std::time::Duration;


enum execution_mode
{
    OFI_GLOBAL,
    OFI_THREAD_LOCAL,

}

enum config
{
    MAX_STACK_SIZE
}

pub fn set_max_stack_size(stack_size:&i32)
{
    config.MAX_STACK_SIZE = stack_size;

}

fn titan_create_thread(name:&String, stack_size:&i32, builder:&Builder) -> Result:i32
{

    let builder = thread::Builder::new()
    .name(name)
    .stack_size(stack_size);
  
    return 1


}


fn titan_join_thread()
{

}


fn titan_detach_thread()
{
    
}
