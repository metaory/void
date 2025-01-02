use serde::{Deserialize, Serialize};

use crate::{Meta, Node, Screen};

// [key, value]
pub type JsonTag = (String, String);

#[derive(Serialize, Deserialize)]
pub struct JsonMeta {
    pub ctime: u64,
    pub mtime: u64,
    pub finish_time: Option<u64>,
    pub tags: Vec<JsonTag>,
    pub due: Option<u64>,
}

// [id, meta, text, children, collapsed, stricken, hide_stricken, parent_id, free_text]
pub type JsonNode = (
    u64,           // id
    JsonMeta,      // meta
    String,        // text
    Vec<u64>,      // children
    bool,          // collapsed
    bool,          // stricken
    bool,          // hide_stricken
    u64,           // parent_id
    Option<String>, // free_text
);

// [nodes, max_id, arrows]
pub type JsonScreen = (Vec<JsonNode>, u64, Vec<(u64, u64)>);

pub fn serialize_screen(screen: &Screen) -> Vec<u8> {
    let screen_json: JsonScreen = (
        screen
            .nodes
            .iter()
            .map(|(_, node)| serialize_node(node))
            .collect(),
        screen.max_id,
        screen
            .arrows
            .iter()
            .map(|&(from, to, _)| (from, to))
            .collect(),
    );

    serde_json::to_vec(&screen_json).unwrap()
}

fn serialize_meta(meta: &Meta) -> JsonMeta {
    JsonMeta {
        ctime: meta.ctime,
        mtime: meta.mtime,
        finish_time: meta.finish_time,
        tags: meta
            .tags
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
        due: meta.due,
    }
}

fn serialize_node(node: &Node) -> JsonNode {
    (
        node.id,
        serialize_meta(&node.meta),
        node.content.clone(),
        node.children.clone(),
        node.collapsed,
        node.stricken,
        node.hide_stricken,
        node.parent_id,
        node.free_text.clone(),
    )
}

pub fn deserialize_screen(data: Vec<u8>) -> Result<Screen, serde_json::Error> {
    let screen_json: JsonScreen = serde_json::from_slice(&data)?;
    let mut screen = Screen::default();
    screen.max_id = screen_json.1;

    screen.nodes = screen_json.0
        .iter()
        .map(|node_json| {
            let node = deserialize_node(node_json);
            screen.tag_db.reindex(node.id, node.content.clone());
            (node.id, node)
        })
        .collect();

    screen.arrows = screen_json.2
        .iter()
        .map(|&(from, to)| (from, to, crate::random_fg_color()))
        .collect();

    Ok(screen)
}

fn deserialize_meta(meta_json: &JsonMeta) -> Meta {
    Meta {
        ctime: meta_json.ctime,
        mtime: meta_json.mtime,
        finish_time: meta_json.finish_time,
        due: meta_json.due,
        tags: meta_json
            .tags
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
    }
}

fn deserialize_node(node_json: &JsonNode) -> Node {
    Node {
        id: node_json.0,
        meta: deserialize_meta(&node_json.1),
        content: node_json.2.clone(),
        children: node_json.3.clone(),
        collapsed: node_json.4,
        stricken: node_json.5,
        hide_stricken: node_json.6,
        parent_id: node_json.7,
        free_text: node_json.8.clone(),
        rooted_coords: (1, 2),
        selected: false,
        color: crate::random_fg_color(),
        auto_arrange: true,
    }
} 