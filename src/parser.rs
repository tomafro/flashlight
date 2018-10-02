use regex::Regex;

#[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Eq)]
pub enum Context {
    Web,
    Job,
    Cable,
    Asset,
    Unknown,
}

impl<'a> From<&'a str> for Context {
    fn from(context: &'a str) -> Self {
        match context {
            "ActiveJob" => Context::Job,
            "ActionCable" => Context::Cable,
            "assets" => Context::Asset,
            "web" => Context::Web,
            _ => Context::Unknown,
        }
    }
}

impl Context {
    #[cfg(test)]
    fn string(&self) -> &'static str {
        match *self {
            Context::Job => "jobs",
            Context::Cable => "cable",
            Context::Asset => "assets",
            Context::Web => "web",
            _ => "unknown",
        }
    }
}

impl From<String> for Context {
    fn from(context: String) -> Self {
        Context::from(context.as_str())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Line {
    content: String,
    context: Context,
    request_id: Option<String>,
}

impl Line {
    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
    }
}

impl From<String> for Line {
    fn from(content: String) -> Self {
        lazy_static! {
            static ref SPLITTER: Regex =
                Regex::new(r"^\[([^\]]+)\](?: \[[^\]]+\])? \[([a-z0-9]+…|[a-f0-9-]+)\]").unwrap();
        }

        let mut context = Context::Unknown;
        let mut request_id = None;

        if let Some(captures) = SPLITTER.captures(&content) {
            context = Context::from(captures.get(1).unwrap().as_str());
            request_id = Some(captures.get(2).unwrap().as_str().to_string());
        }

        Line {
            content,
            request_id,
            context,
        }
    }
}

impl<'a> From<&'a String> for Line {
    fn from(content: &'a String) -> Self {
        Line::from(content.to_string())
    }
}

impl<'a> From<&'a str> for Line {
    fn from(content: &'a str) -> Self {
        Line::from(content.to_string())
    }
}

#[cfg(test)]
mod test;
