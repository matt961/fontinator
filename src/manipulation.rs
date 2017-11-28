use std::cell::{
    Cell,
    RefCell
};

use std::rc::Rc;

use html5ever::{
    parse_document,
    ParseOpts,
    rcdom,
    tree_builder,
    tendril
};

use html5ever::tendril::TendrilSink;
use html5ever::rcdom::NodeData;

use markup5ever;

use rand::{
    thread_rng,
    Rng
};

const FONT_CLASSES: &'static [&'static str] = &[
    "font-a",
    "font-b",
    "font-c",
    "font-d",
    "font-e",
    "font-f",
    "font-g",
    "font-h",
    "font-i",
    "font-k",
    "font-l",
    "font-m",
    "font-n",
    "font-o",
    "font-p",
    "font-q",
];

pub fn to_dom(html: &[u8]) -> rcdom::RcDom {
    let opts = ParseOpts {
        tree_builder: tree_builder::TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };
    parse_document(rcdom::RcDom::default(), opts)
        .from_utf8()
        .one(html)
}

pub fn walk_and_randomize(handle: rcdom::Handle, scheme: &str, origin: &str) {
    let node = handle;

    match node.data {
        NodeData::Element { ref attrs, .. } => {
            let ref mut attrs = attrs.borrow_mut();

            let rn = thread_rng().gen_range(0, FONT_CLASSES.len());
            let mut font_str = " ".to_string();
            font_str.push_str(FONT_CLASSES[rn]);

            let mut has_class_attr = false;

            if let Some(class_attr) = attrs.iter_mut()
                .find(|ref attr| attr.name.local.contains("class")) {
                class_attr.value.push_slice(&font_str);
                has_class_attr = true;
            }

            if !has_class_attr {
                let new_class_attr = tree_builder::Attribute {
                    name: markup5ever::QualName::new(None, ns!(), local_name!("class")),
                    value: tendril::StrTendril::from(font_str.as_str())
                };
                attrs.push(new_class_attr);
            }

            // fix domain prefix for loading foreign content
            if let Some(obj_path) = attrs.iter_mut()
                .find(|ref attr| attr.name.local.contains("href")
                    || attr.name.local.contains("src")) {
                if obj_path.value.starts_with("/") {
                    let mut domain = tendril::StrTendril::new();
                    let prefix = format!("{}://{}", scheme, origin);
                    domain.push_slice(&prefix);
                    domain.push_tendril(&obj_path.value);
                    obj_path.value = domain;
                }
            }
        }
        _ => (),
    };
    for child in node.children.borrow().iter() {
        walk_and_randomize(child.clone(), scheme, origin);
    }
}

pub fn push_style(document: rcdom::Handle) -> Result<(), String> {
    let ref mut sink = rcdom::RcDom::default();

    let style_elem = tree_builder::create_element(
        sink,
        markup5ever::QualName::new(None, ns!(html), local_name!("style")),
        Vec::new()
    );

    let mut s = tendril::StrTendril::new();
    s.push_slice(include_str!("../static/random-style.css"));

    let text_css = Rc::new(rcdom::Node {
        parent: Cell::new(None),
        children: RefCell::new(Vec::new()),
        data: NodeData::Text {
            contents: RefCell::new(s)
        }
    });

    style_elem.children.borrow_mut().push(text_css);

    document.children.borrow().iter()
        .find(|ref node| {
            match node.data {
                NodeData::Element { ref name, .. } => {
                    name.local.contains("html")
                }
                _ => false
            }
        })
        // returns Err(String) if not successful in finding `<html>`
        .ok_or("Malformed html document, there's no <html> tag".to_string())
        // The Some(node) has been mapped to a Result::Ok(node)
        .and_then(move |html_tag| {
            let mut children = html_tag.children.borrow_mut();
            let head_elem = children.iter_mut()
                .find(|ref node| {
                    if let NodeData::Element { ref name, .. } = node.data {
                        name.local.contains("head")
                    } else {
                        false
                    }
                });
            if let Some(head_elem) = head_elem {
                head_elem.children.borrow_mut().push(style_elem);
                Ok(())
            } else {
                Err("Couldn't find a head element :(".to_string())
            }
        })
}

