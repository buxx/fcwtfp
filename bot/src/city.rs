use common::session::city::CitiesState;

pub struct CitiesStateMarkdown(pub String);

impl From<CitiesState> for CitiesStateMarkdown {
    fn from(state: CitiesState) -> Self {
        let mut value = "# 🌆 Cities\n\n".to_string();

        for (member, cities) in state.cities() {
            value.push_str(&format!("## {}\n\n", member.name().0));
            for city in cities {
                value.push_str(&format!("* {}\n", city.city_name()));
            }
        }

        Self(value.to_string())
    }
}
