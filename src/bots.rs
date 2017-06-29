trait Bot {
    fn say_hello(&self) {
        return json!({
            "name": self.name,
        });
    }
}

struct EqxBot {
    pub name: String
}

impl Bot for EqxBot {
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
