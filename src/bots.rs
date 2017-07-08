use rocket_contrib::Value;

pub trait Bot {
    fn say_hello(&self) -> Value;
}

struct EqxBot {
    pub name: String
}

impl Bot for EqxBot {
    fn say_hello(&self) -> Value {
        return json!({
            "name": self.name,
        });
    }
}

// TESTS

#[test]
fn bot_says_hello() {
    let eqxbot = EqxBot {
        name: "EqxBot".to_string(),
    };

    assert_eq!(eqxbot.say_hello(), json!({
        "name": "EqxBot",
    }));
}
