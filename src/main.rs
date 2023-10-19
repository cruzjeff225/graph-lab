use graph_lab::Graph;
fn main() {
    let mut graph = Graph::new();

    graph.add_node("A");
    graph.add_node("B");
    graph.add_node("C");
    graph.add_node("D");

    let a = 0;
    let b = 1;
    let c = 2;
    let d = 3;

    graph.add_edge(a, b);
    graph.add_edge(b, c);
    graph.add_edge(b, d);

    println!("Graph contains 'C': {}", graph.contains("C"));
    println!("Graph contains 'E': {}", graph.contains("E"));

    if let Some(path) = graph.find_path(a, d) {
        println!("Path from A to D: {:?}", path);
    } else {
        println!("No path found from A to D.");
    }
}