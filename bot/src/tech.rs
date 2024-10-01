use common::session::tech::TechnologiesState;

pub struct TechnologyStateMarkdown(pub String);

impl From<TechnologiesState> for TechnologyStateMarkdown {
    fn from(state: TechnologiesState) -> Self {
        let mut value = "# 🔬 Technology\n\n".to_string();
        value.push_str("## ✅ Almost one nation own\n\n");

        for technology in state.done() {
            let members_str = technology
                .member_names()
                .iter()
                .map(|n| n.0.clone())
                .collect::<Vec<String>>()
                .join(", ");
            value.push_str(&format!(
                "* {} ({})\n",
                technology.technology_name(),
                members_str
            ));
        }

        value.push_str("## 🎯 Almost one nation searching\n\n");

        for technology in state.search() {
            let members_str = technology
                .member_names()
                .iter()
                .map(|n| n.0.clone())
                .collect::<Vec<String>>()
                .join(", ");
            value.push_str(&format!(
                "* {} ({})\n",
                technology.technology_name(),
                members_str
            ));
        }

        Self(value.to_string())
    }
}
