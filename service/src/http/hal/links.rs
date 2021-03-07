use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Link {
    pub href: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl<S> From<S> for Link
where
    S: Into<String>,
{
    fn from(href: S) -> Self {
        Self {
            href: href.into(),
            name: None,
        }
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Links {
    Single(Link),
    Multiple(Vec<Link>),
}

impl Links {
    pub fn append(self, link: Link) -> Self {
        let links = match self {
            Links::Single(previous) => {
                vec![previous, link]
            }
            Links::Multiple(mut previous) => {
                previous.push(link);
                previous
            }
        };

        Links::Multiple(links)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};

    #[test]
    fn convert_str_to_link() {
        let link = Link::from("/test");

        check!(link.href == "/test");
        check!(link.name == None);
    }

    #[test]
    fn append_to_single_link() {
        let link_1 = Link::from("/abc");
        let link_2 = Link::from("/def");

        let links = Links::Single(link_1.clone());
        let result = links.append(link_2.clone());

        let_assert!(Links::Multiple(list) = result);
        check!(list == vec![link_1, link_2]);
    }

    #[test]
    fn append_to_multiple_links() {
        let link_1 = Link::from("/abc");
        let link_2 = Link::from("/def");
        let link_3 = Link::from("/ghi");

        let links = Links::Multiple(vec![link_1.clone(), link_3.clone()]);
        let result = links.append(link_2.clone());

        let_assert!(Links::Multiple(list) = result);
        check!(list == vec![link_1, link_3, link_2]);
    }
}
