use super::wearable::Wearable;

pub fn dg_summer_2020() -> Vec<Wearable> {
    vec![
    Wearable::new(
        String::from("dg_flip_up_spectacles_eyewear"),
        String::from("DG Flip Up Spectacles"),
        String::from("Swaggy flip up spectacles for any occasion, inside or outside, featuring a chain to keep them secure #ice"),
        String::from("eyewear"),
        String::from("legendary"),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
    Wearable::new(
        String::from("dg_deezys_feet"),
        String::from("DG Deezys"),
        String::from("Comfy and elegant sneakers #deezys"),
        String::from("feet"),
        String::from("legendary"),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
    Wearable::new(
        String::from("dg_slides_feet"),
        String::from("DG Slides"),
        String::from("Lazy day designer slides complete with socks to keep your toes warm #cozyslides"),
        String::from("feet"),
        String::from("legendary"),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
    Wearable::new(
        String::from("dg_tracksuit_bottom_lower_body"),
        String::from("DG Tracksuit Bottom"),
        String::from("The pants of the decadent and elegant DG tracksuit #drip"),
        String::from("lower_body"),
        String::from("legendary"),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
    Wearable::new(
        String::from("dg_mink_fur_coat_upper_body"),
        String::from("DG Fur Mink"),
        String::from("An opulent fur coat made from minks #extrasaus"),
        String::from("upper_body"),
        String::from("legendary"),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
    Wearable::new(
        String::from("dg_tracksuit_top_upper_body"),
        String::from("DG Tracksuit Top"),
        String::from("The jacket of the decadent and elegant DG tracksuit #drip"),
        String::from("upper_body"),
        String::from("legendary"),
        vec![String::from("BaseMale"), String::from("BaseFemale")],
    ),
]
}
