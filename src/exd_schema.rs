use serde::Deserialize;
use serde_yml;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Schema {
    fields: Vec<Field>,
    pending_fields: Option<Vec<Field>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Field {
    name: Option<String>, // Name is optional for array fields
    pending_name: Option<String>,

    #[serde(rename = "type", default)]
    kind: FieldKind,

    count: Option<u32>,
    fields: Option<Vec<Field>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
enum FieldKind {
    Scalar,
    Array,
    Icon,
    ModelId,
    Color,
    Link,
}

impl Default for FieldKind {
    fn default() -> Self {
        Self::Scalar
    }
}

/// Retrieve a list of field names from EXDSchema for the given sheet
pub fn field_names(sheet_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let path = format!("schemas/{}.yml", sheet_name);
    let file = File::open(&path).expect(format!("Could not find schema: {}", &path).as_str());
    let schema: Schema =
        serde_yml::from_reader(file).expect(format!("Failed to parse schema: {}", &path).as_str());

    // Prefer the pending field list when available
    let names: Vec<String> = match schema.pending_fields {
        Some(pending) => parse_field_names(&pending),
        None => parse_field_names(&schema.fields),
    };

    return Ok(names);
}

fn parse_field_names(fields: &Vec<Field>) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();

    // Add the ID field
    names.push(String::from("#"));

    for field in fields.iter() {
        let name = latest_name(&field);

        match field.kind {
            FieldKind::Array => {
                parse_array(&field, name, &mut names);
            }
            _ => {
                names.push(name);
            }
        }
    }

    return names;
}

// Prefer the pending field name when available
fn latest_name(field: &Field) -> String {
    return match &field.pending_name {
        Some(pending) => pending.clone(),
        None => field
            .name
            .clone()
            .expect("Schema must provide a name for all fields."),
    };
}

fn parse_array(field: &Field, name: String, names: &mut Vec<String>) {
    match &field.count {
        Some(count) => {
            for i in 0..*count {
                // Append an index to the given name
                let name = format!("{}[{}]", name, i);

                match &field.fields {
                    Some(fields) => {
                        if fields.len() > 1 {
                            // If the array has more than one field, we need to traverse them and parse the nested fields
                            for field in fields {
                                // Technically we should re-check the field kind, but only arrays are countable at the moment
                                parse_array(
                                    &field,
                                    format!("{}.{}", name, latest_name(field)),
                                    names,
                                );
                            }
                        } else {
                            // Otherwise, we can just push the new field name
                            names.push(name);
                        }
                    }
                    None => {
                        // There are no nested fields to deal with - just push the field name
                        names.push(name);
                    }
                }
            }
        }
        None => {
            // An uncountable field found within a nested array - just push the field name
            names.push(name);
        }
    }
}
