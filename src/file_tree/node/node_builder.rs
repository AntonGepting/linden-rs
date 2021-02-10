

#[derive(Debug, Default)]
pub struct NodeBuilder {
    parent: Option<Node>,
    file: OsString,
}

impl NodeBuilder {
    pub fn new() -> Self {
        NodeBuilder::default()
    }

    pub fn file(&mut self, file: OsString) -> &mut Self {
        self.file = file;
        self
    }

    pub fn build(self) -> Node {
        Node {
            file: self.file,
            ..Default::default()
        }
    }
}
