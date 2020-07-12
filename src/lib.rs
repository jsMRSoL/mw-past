use serde_json::Value;
use std::env;

#[derive(Debug)]
pub struct Verb {
    pub present: String,
    pub past_simple: String,
    pub past_part: String,
    pub asterisked: String,
}

#[tokio::main]
pub async fn lookup(s: &str) -> Verb {
    // split up phrasal verb. We want to look up the head, not the tail
    let head_and_tail = s.splitn(2, " ").collect::<Vec<_>>();
    let head = head_and_tail[0].to_string();
    let mut tail: String = String::new();
    if head_and_tail.len() > 1 {
        tail = format!(" {}", head_and_tail[1]);
    }

    // make an api request with the head
    let api_key;
    let key = "MW_SCHOOL_KEY";
    match env::var(key) {
        Ok(val) => api_key = val,
        Err(e) => {
            eprintln!("Couldn't find API key {}: {}", key, e);
            return default_verb(s, &head, &tail);
        }
    }

    let url = format!(
        "https://www.dictionaryapi.com/api/v3/references/sd4/json/{}?key={}",
        head, api_key
    );

    let v: Value = match reqwest::get(&url).await {
        Ok(content) => match content.json().await {
            Ok(v) => v,
            _ => Value::Null,
        },
        _ => Value::Null,
    };

    match v {
        Value::Null => default_verb(s, &head, &tail),
        _ => {
            // let inflections = v[0]["ins"].as_array().unwrap();
            let index = match &v[0]["fl"].as_str() {
                Some("verb") => 0,
                _ => 1,
            };
            let inflections = v[index]["ins"].as_array();
            match inflections {
                Some(inflections) => {
                    let past = inflections[0]["if"].as_str().unwrap().replace("*", "");
                    let past_simple = format!("{}{}", past, tail);
                    let participle = inflections[1]["if"].as_str().unwrap().replace("*", "");
                    let past_part = match &participle {
                        pt if pt.ends_with("ing") => past_simple.clone(),
                        _ => format!("{}{}", participle, tail),
                    };
                    return Verb {
                        present: s.to_string(),
                        past_simple,
                        past_part,
                        asterisked: format!("*{}*{}", head, tail),
                    };
                }
                _ => default_verb(s, &head, &tail),
            }
        }
    }
}

fn default_verb(s: &str, head: &str, tail: &str) -> Verb {
    let past_simple = match &head {
        hd if hd.ends_with("e") => format!("{}d{}", hd, tail),
        _ => format!("{}ed{}", head, tail),
    };
    let past_part = past_simple.clone();
    return Verb {
        present: s.to_string(),
        past_simple,
        past_part,
        asterisked: format!("*{}*{}", head, tail),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn record_works() {
        let res: Verb = lookup("give");
        println!("Give:\n{:#?}", res);
        // let res2: Verb = lookup("give up");
        // println!("Give: up\n{:#?}", res2);
        // let res3: Verb = lookup("take");
        // println!("Take:\n{:#?}", res3);
        // let res4: Verb = lookup("take off");
        // println!("Take off:\n{:#?}", res4);
        // let res5: Verb = lookup("have");
        // println!("Have:\n{:#?}", res5);
        // let res6: Verb = lookup("have at");
        // println!("Have at:\n{:#?}", res6);
        // let res7: Verb = lookup("get");
        // println!("Get:\n{:#?}", res7);
        // let res8: Verb = lookup("get in");
        // println!("Get in:\n{:#?}", res8);
        // let res9: Verb = lookup("wug");
        // println!("Wug:\n{:#?}", res9);
        // let res10: Verb = lookup("wug up");
        // println!("Wug up:\n{:#?}", res10);
        // let res11: Verb = lookup("wuge");
        // println!("Wuge:\n{:#?}", res11);
        // let res12: Verb = lookup("wuge up");
        // println!("Wuge up:\n{:#?}", res12);
        let res13: Verb = lookup("plan");
        println!("Plan:\n{:#?}", res13);
    }
}
