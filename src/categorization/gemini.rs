use gtk4::prelude::{ComboBoxExt, EditableExt};
use reqwest::*;
use serde::{Serialize, Deserialize};
use std::result::Result;
use crate::ui::add_list_dialog::RowTuple;

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct RequestBody {
    contents: Vec<Content>,
}

#[derive(Deserialize, Debug)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize, Debug)]
struct Candidate {
    content: ContentResponse,
}

#[derive(Deserialize, Debug)]
struct ContentResponse {
    parts: Vec<PartResponse>,
}

#[derive(Deserialize, Debug)]
struct PartResponse {
    text: String,
}

#[derive(Deserialize, Debug)]
struct CategorizedItems {
    #[serde(rename = "Protein")]
    protein: Vec<String>,
    #[serde(rename = "Fruit/Vegetable")]
    fruit_vegetable: Vec<String>,
    #[serde(rename = "Dairy")]
    dairy: Vec<String>,
    #[serde(rename = "Carbohydrate")]
    carbohydrate: Vec<String>,
    #[serde(rename = "Fat/Oil")]
    fat_oil: Vec<String>,
    #[serde(rename = "Unhealthy")]
    unhealthy: Vec<String>,
    #[serde(rename = "Hygiene")]
    hygiene: Vec<String>,
    #[serde(rename = "Miscellaneous")]
    miscellaneous: Vec<String>,
}


// Serializes a request object to json containing the gemini prompt and makes the request. Then
// receiving the response and processing it
pub async fn categorize_new_items_with_gemini(new_items: &RowTuple, categorized_items: RowTuple) -> Result<RowTuple, Box<dyn std::error::Error>> {
    if new_items.borrow().len() == 0 {
        return Ok(categorized_items);
    }

    let client = Client::new();
    let endpoint = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key=GEMINI_API_KEY";

    let mut body_str = "Given the following categories: Protein, Fruit/Vegetable, Dairy, Carbohydrate, Fat/Oil, Unhealthy, Hygiene, Miscellaneous. Categorize these items in the context of nutrition and return the result as JSON with each category being an array, include nothing else except the resulting json in the response and if there are no items given to categorize then return the empty arrays: ".to_string();

    for (name, _price, _category) in new_items.borrow_mut().iter() {
        body_str.push_str(&format!("{}, ", name.text()));
    }

    let body = RequestBody {
        contents: vec![Content {
            parts: vec![Part {
                text: body_str,
            }],
        }],
    };


    let response = client.post(endpoint)
        .json(&body)
        .send()
        .await?
        .json::<GeminiResponse>()
        .await?;

    let mut raw_json_response = String::new();

    for candidate in response.candidates {
        for part in candidate.content.parts {
            raw_json_response.push_str(&part.text);
        }
    }
    raw_json_response = extract_json(&raw_json_response).to_string();
    let categories: CategorizedItems = serde_json::from_str(&raw_json_response)?;

    for (name, _price, category) in new_items.borrow().iter() {
        let mut found = false;
        if !found {
            for protein in categories.protein.iter() {
                if &name.text().to_string() == protein {
                    category.set_active_id(Some("Protein"));
                    found = true;
                    break;
                }
            }
        }

        if !found {
            for fruit_vegetable in categories.fruit_vegetable.iter() {
                if &name.text().to_string() == fruit_vegetable {
                    category.set_active_id(Some("Fruit/Vegetable"));
                    found = true;
                    break;
                }
            }
        }

        if !found {
            for dairy in categories.dairy.iter() {
                if &name.text().to_string() == dairy {
                    category.set_active_id(Some("Dairy"));
                    found = true;
                    break;
                }
            }
        }

        if !found {
            for carbohydrate in categories.carbohydrate.iter() {
                if &name.text().to_string() == carbohydrate {
                    category.set_active_id(Some("Carbohydrate"));
                    found = true;
                    break;
                }
            }
        }

        if !found {
            for fat_oil in categories.fat_oil.iter() {
                if &name.text().to_string() == fat_oil {
                    category.set_active_id(Some("Fat/Oil"));
                    found = true;
                    break;
                }
            }
        }

        if !found {
            for unhealthy in categories.unhealthy.iter() {
                if &name.text().to_string() == unhealthy {
                    category.set_active_id(Some("Unhealthy"));
                    found = true;
                    break;
                }
            }
        }

        if !found {
            for hygiene in categories.hygiene.iter() {
                if &name.text().to_string() == hygiene {
                    category.set_active_id(Some("Hygiene"));
                    found = true;
                    break;
                }
            }
        }

        if !found {
            for miscellaneous in categories.miscellaneous.iter() {
                if &name.text().to_string() == miscellaneous {
                    category.set_active_id(Some("Miscellaneous"));
                    break;
                }
            }
        }
    }

    Ok(categorized_items)
}

fn extract_json(raw: &str) -> &str {
    raw.trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim()
}

