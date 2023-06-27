use super::wearable::Wearable;

pub fn ethermon_wearables() -> Vec<Wearable> {
    vec![
  Wearable::new(
    String::from("ethermon_feet"),
    String::from("Ethermon Shoes"),
    String::from("Each piece of the Ethermon Legendary Set provides its own unique buff when battling & capturing Mons. Wearing 4 pieces at once will grant the owner special access."),
    String::from("feet"),
    String::from("legendary"),
    vec![String::from("BaseMale"), String::from("BaseFemale")],
),
Wearable::new(
    String::from("ethermon_hat"),
    String::from("Ethermon Hat"),
    String::from("Each piece of the Ethermon Legendary Set provides its own unique buff when battling & capturing Mons. Wearing 4 pieces at once will grant the owner special access."),
    String::from("hat"),
    String::from("legendary"),
    vec![String::from("BaseMale"), String::from("BaseFemale")],
),
Wearable::new(
    String::from("ethermon_lower_body"),
    String::from("Ethermon Pants"),
    String::from("Each piece of the Ethermon Legendary Set provides its own unique buff when battling & capturing Mons. Wearing 4 pieces at once will grant the owner special access."),
    String::from("lower_body"),
    String::from("legendary"),
    vec![String::from("BaseMale"), String::from("BaseFemale")],
),
Wearable::new(
    String::from("barkindle_top_head"),
    String::from("Barkindle Wearable"),
    String::from("Each piece of the Ethermon Legendary Set provides its own unique buff when battling & capturing Mons. Wearing 4 pieces at once will grant the owner special access."),
    String::from("top_head"),
    String::from("legendary"),
    vec![String::from("BaseMale"), String::from("BaseFemale")],
),
Wearable::new(
    String::from("ruffski_top_head"),
    String::from("Ruffski Wearable"),
    String::from("Each piece of the Ethermon Legendary Set provides its own unique buff when battling & capturing Mons. Wearing 4 pieces at once will grant the owner special access."),
    String::from("top_head"),
    String::from("legendary"),
    vec![String::from("BaseMale"), String::from("BaseFemale")],
),
Wearable::new(
    String::from("ethermon_upper_body"),
    String::from("Ethermon Jacket"),
    String::from("Each piece of the Ethermon Legendary Set provides its own unique buff when battling & capturing Mons. Wearing 4 pieces at once will grant the owner special access."),
    String::from("upper_body"),
    String::from("legendary"),
    vec![String::from("BaseMale"), String::from("BaseFemale")],
)
]
}
