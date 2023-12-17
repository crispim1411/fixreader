use serde::Deserialize;

#[derive(Deserialize)]
pub struct FixSchema {
    pub header: Header,
    pub trailer: Trailer,
    pub messages: Messages,
    pub components: Components,
    pub fields: Fields,
}

#[derive(Deserialize)]
pub struct Header {
    #[serde(rename = "$value")]
    pub values: Vec<FieldHeader>,
}

#[derive(Deserialize)]
pub struct Trailer {
    #[serde(rename = "$value")]
    pub values: Vec<FieldHeader>,
}

#[derive(Deserialize)]
pub struct Messages {
    #[serde(rename = "$value")]
    pub values: Vec<Message>,
}

#[derive(Deserialize)]
pub struct Components {
    #[serde(rename = "$value")]
    pub values: Vec<Component>,
}

#[derive(Deserialize)]
pub struct Fields {
    #[serde(rename = "$value")]
    pub values: Vec<Field>,
}

#[derive(Deserialize, Clone)]
pub struct FieldHeader {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@required")]
    pub required: String,
    #[serde(default, rename = "$value")]
    pub group: Vec<FieldHeader>,
}

#[derive(Deserialize)]
pub struct Message {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@msgtype")]
    pub msgtype: String,
    #[serde(rename = "@msgcat")]
    pub msgcat: String,
    #[serde(default, rename = "$value")]
    pub fields: Vec<FieldHeader>,
}

#[derive(Deserialize)]
pub struct Component {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(default, rename = "$value")]
    pub fields: Vec<FieldHeader>,
}

#[derive(Deserialize)]
pub struct Field {
    #[serde(rename = "@number")]
    pub number: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub field_type: String,
    #[serde(default, rename = "$value")]
    pub values: Vec<FieldValues>,
}

#[derive(Deserialize)]
pub struct FieldValues {
    #[serde(rename = "@enum")]
    pub value: String,
    #[serde(rename = "@description")]
    pub description: String,
}

type ParsedTag = Vec<(String, String, String)>;

impl FixSchema {
    pub fn parse_tags<'a, I>(&self, mut tag_values: I) -> Result<ParsedTag, &'static str> 
    where 
        I: Iterator<Item=(&'a str, &'a str)> 
    {
        let msgtype_value = tag_values.find(|x| x.0 == "35").expect("Mensagem fix inválida").1;
        let msg_schema = self.find_msgtype(msgtype_value).expect("Tipo de mensagem não suportado");

        let mut values = Vec::new();
        for (tag, value) in tag_values {
            let Some(field) = self.find_field_by_tag(tag) else { 
                values.push((tag.to_string(), value.to_string(), String::new()));
                continue;
            };
            let description: &str = {
                if let Some(field) = field.values.iter().find(|item| &item.value == value) {
                    &field.description
                } else { value }
            };
            let required = 
                if let Some(body_field) = msg_schema.fields.iter().find(|x| x.name == field.name) {
                    body_field.required.clone()
                } else if let Some(header_field) = self.header.values.iter().find(|x| x.name == field.name) {
                    header_field.required.clone()
                } else if let Some(trailer_field) = self.trailer.values.iter().find(|x| x.name == field.name) {
                    trailer_field.required.clone()
                } else {
                    String::new()
                };
            
            values.push((field.name.to_string(), description.to_string(), required));
            
        }
        return Ok(values);
    }

    fn find_msgtype(&self, msgtype: &str) -> Option<Message> {
        if let Some(msg) = self.messages.values
            .iter()
            .find(|&item| item.msgtype == msgtype) {
                let fields = msg.fields.clone();
                let grouped: Vec<FieldHeader> = fields.iter().flat_map(|i| i.group.clone()).collect();
                return Some(Message {
                    name: msg.name.clone(),
                    msgtype: msg.msgtype.clone(),
                    msgcat: msg.msgcat.clone(),
                    fields: [fields, grouped].concat(),
                });
        }
        None
    }

    fn find_field_by_tag(&self, tag: &str) -> Option<&Field> {
        self.fields.values
            .iter()
            .find(|item| &item.number == tag)
    }
}
