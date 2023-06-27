use super::wearable::Wearable;

pub fn dgtble_headspace() -> Vec<Wearable> {
    vec![
        Wearable::new(
            String::from("dgtble_rainbow_grid_shoes_feet"),
            String::from("The Headspace Rainbow Grid Shoes"),
            String::from("Rainbow Grid is a licensed artwork collab between The Headspace and Daniel Prust whose asymmetric patterns and abstract designs deliver an experimental style that transforms outlandish dreams into pleasing aesthetics."),
            String::from("feet"),
            String::from("legendary"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("dgtble_shorts_folife_lower_body"),
            String::from("The Headspace Flower of Life Shorts"),
            String::from("We are the divine, and together we make a web of divine life on this planet and beyond that is infinite and unbreakable."),
            String::from("lower_body"),
            String::from("legendary"),
            vec![String::from("BaseMale")],
        ),
        Wearable::new(
            String::from("dgtble_protection_mask_dala_mask"),
            String::from("The Headspace Mandala Love Mask"),
            String::from("Mandala Love is a licensed artwork collab between The Headspace and Cameron Grey, an Aria nominated Melbourne Artist who believes you must teach people how to empower themselves from within their true self, at their core level of being, and the rest will always follow."),
            String::from("mask"),
            String::from("legendary"),
            vec![String::from("BaseMale"), String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("dgtble_dress_folife_upper_body"),
            String::from("The Headspace Flower of Life Dress"),
            String::from("In your mind's eye witness The Flower of Life in this space, floating before you in golden light. Take a moment to absorb the divine wisdom contained in this sacred figure."),
            String::from("upper_body"),
            String::from("legendary"),
            vec![String::from("BaseFemale")],
        ),
        Wearable::new(
            String::from("dgtble_hoodi_linetang_upper_body"),
            String::from("The Headspace Linetangles Hoodie"),
            String::from("Linetangles is a licensed artwork from Daniel Prust whose asymmetric patterns and abstract designs deliver an experimental style that transforms outlandish dreams into pleasing aesthetics."),
            String::from("upper_body"),
            String::from("legendary"),
            vec![String::from("BaseMale")],
        ),
        Wearable::new(
            String::from("dgtble_tshirt_message_upper_body"),
            String::from("The Headspace The Messanger T-Shirt"),
            String::from("The Messenger is a licensed artwork from Grokko whose hand-drawn works are laid with the intention of stirring memories within our subconscious, to return us to the origins of humanity and discover our purpose going forward. It is through the connection to our timeless selves that our dreams can materialize."),
            String::from("upper_body"),
            String::from("legendary"),
            vec![String::from("BaseMale")],
        )
    ]
}
