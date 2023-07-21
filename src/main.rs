mod graph;

use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::node::element::Text;
use svg::node::Text as SvgText;
use svg::Document;

pub use crate::graph::{layout, Color, Edge, Graph, Node};

fn main() {
    let mut graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };
    graph.nodes.push(Node {
        id: String::from("node_a"),
        color: Color { r: 255, g: 0, b: 0 },
        name: String::from("A"),
        x: 0.0,
        y: 0.0,
    });
    graph.nodes.push(Node {
        id: String::from("node_b"),
        color: Color { r: 0, g: 255, b: 0 },
        name: String::from("B"),
        x: 0.1,
        y: 0.1,
    });
    graph.nodes.push(Node {
        id: String::from("node_c"),
        color: Color { r: 0, g: 255, b: 0 },
        name: String::from("C"),
        x: 0.2,
        y: 0.2,
    });
    graph.edges.push(Edge {
        source_id: String::from("node_a"),
        dest_id: String::from("node_b"),
    });
    graph.edges.push(Edge {
        source_id: String::from("node_a"),
        dest_id: String::from("node_c"),
    });
    let laid_out_graph = layout(graph, 10.0, 10.0, 1.0);

    // let data = Data::new()
    //     .move_to((10, 10))
    //     .line_by((0, 50))
    //     .line_by((50, 0))
    //     .line_by((0, -50))
    //     .close();

    // let path = Path::new()
    //     .set("fill", "none")
    //     .set("stroke", "black")
    //     .set("stroke-width", 3)
    //     .set("d", data);

    let mut document = Document::new().set("viewBox", (-1, -1, 1, 1));

    for node in laid_out_graph.nodes.iter() {
        let text = Text::new()
            .set("x", node.x)
            .set("y", node.y)
            .set("fill", "black")
            .add(SvgText::new(node.name.clone()));
       document.clone().add(text);
    }

    svg::save("image.svg", &document).unwrap();
}
