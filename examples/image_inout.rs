fn main() -> titan::titanResult {
    let context = titan::Context::createContext();

    let node = context.create_node("image_inout")?;

    node.input_port("<File-name>");
    node.output_port();

    let result = node.run_graph();
}
