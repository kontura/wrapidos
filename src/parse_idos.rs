
pub struct Connection {
    pub name: String,
    pub departure_time: String,
    pub departure_station: String,
    pub destination_time: String,
    pub destination_station: String,
}


pub fn parse_idos(html: &String) -> Option<Vec<Vec<Connection>>> {
    let dom = html_parser::Dom::parse(&html).unwrap();
    let connection_list = recusively_find_element_by_class(&dom.children, "connection-list");
    let cn = match connection_list {
        Some(ref element) => element,
        None => return None
    };

    let mut con_cons: Vec<Vec<Connection>> = Vec::new();

    for node in &cn.children {
        match node {
            html_parser::Node::Element(ref element) => {
                if element.classes.contains(&String::from("connection")) {
                    let connections = parse_connections(&element);
                    con_cons.push(connections);
                }
            }
            _ => (),
        };
    };

    return Some(con_cons);
}

fn parse_connections(element: &html_parser::Element) -> Vec<Connection> {
    let mut cons = Vec::new();


    let children = &element.children;
    let connection_details_node = &children[1];
    if let html_parser::Node::Element(connection_details) = connection_details_node {
        assert!(connection_details.classes.contains(&String::from("connection-details")));

        let line_item_node = &connection_details.children[0];
        if let html_parser::Node::Element(line_item) = line_item_node {
            assert!(line_item.classes.contains(&String::from("line-item")));
            for inner_line_item_node in &line_item.children {
                if let html_parser::Node::Element(inner_line_item) = inner_line_item_node {
                    assert!(inner_line_item.classes.contains(&String::from("outside-of-popup")));
                    let con = parse_connection(&inner_line_item);
                    cons.push(con);
                }
            }
        }

    }

    return cons;
}

fn parse_connection(element: &html_parser::Element) -> Connection {
    let title = recusively_find_element_by_tag(&element.children, "h3");
    let test = recusively_gather_all_text(&title.unwrap().children);
    let stations = recusively_find_element_by_class(&element.children, "stations");

    let mut c = Connection {
        name: test.unwrap().trim().to_string(),
        departure_time: "".to_string(),
        departure_station: "".to_string(),
        destination_time: "".to_string(),
        destination_station: "".to_string(),
    };

    let departure_node = &stations.unwrap().children[0];
    if let html_parser::Node::Element(departure) = departure_node {
        let time = recusively_find_element_by_class(&departure.children, "time");
        let dep_time = recusively_gather_all_text(&time.unwrap().children);
        c.departure_time = dep_time.unwrap();

        let station = recusively_find_element_by_class(&departure.children, "station");
        let dep_station = recusively_gather_all_text(&station.unwrap().children);
        c.departure_station = dep_station.unwrap();
    }

    let destination_node = &stations.unwrap().children[1];
    if let html_parser::Node::Element(destination) = destination_node {
        let time = recusively_find_element_by_class(&destination.children, "time");
        let des_time = recusively_gather_all_text(&time.unwrap().children);
        c.destination_time = des_time.unwrap();

        let station = recusively_find_element_by_class(&destination.children, "station");
        let des_station = recusively_gather_all_text(&station.unwrap().children);
        c.destination_station = des_station.unwrap();
    }

    return c;
}

fn recusively_find_element_by_class<'a>(nodes: &'a Vec<html_parser::Node>, class: &str) -> Option<&'a html_parser::Element> {
    for node in nodes {
        let ret = match node {
            html_parser::Node::Element(ref element) if element.classes.contains(&String::from(class)) => Some(element),
            html_parser::Node::Text(ref _text) => None,
            html_parser::Node::Comment(ref _comment) => None,
            html_parser::Node::Element(ref element) => recusively_find_element_by_class(&element.children, class),
        };
        if ret != None {
            return ret;
        }
    }

    return None;
}

fn recusively_find_element_by_tag<'a>(nodes: &'a Vec<html_parser::Node>, tag: &str) -> Option<&'a html_parser::Element> {
    for node in nodes {
        let ret = match node {
            html_parser::Node::Element(ref element) if element.name.contains(&String::from(tag)) => Some(element),
            html_parser::Node::Text(ref _text) => None,
            html_parser::Node::Comment(ref _comment) => None,
            html_parser::Node::Element(ref element) => recusively_find_element_by_tag(&element.children, tag),
        };
        if ret != None {
            return ret;
        }
    }

    return None;
}

fn recusively_gather_all_text(nodes: & Vec<html_parser::Node>) -> Option<String> {
    let mut gathered = String::new();
    for node in nodes {
        let ret = match node {
            html_parser::Node::Text(text) => Some(text.clone()),
            html_parser::Node::Comment(_comment) => None,
            html_parser::Node::Element(element) => recusively_gather_all_text(&element.children),
        };
        if ret != None {
            gathered.push(' ');
            gathered.push_str(ret?.as_str());
        }
    }

    return Some(gathered);
}

