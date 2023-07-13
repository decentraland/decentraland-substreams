use super::wearables_v1::*;

fn get_all_collections() -> Vec<Vec<wearable::Wearable>> {
    vec![
        atari_launch::atari_launch(),
        binance_us_collection::binance_us_collection(),
        china_flying::china_flying(),
        community_contest::community_contest(),
        cybermike_cybersoldier_set::cybermike_cybersoldier_set(),
        cz_mercenary_mtz::cz_mercenary_mtz(),
        dappcraft_moonminer::dappcraft_moonminer(),
        dc_meta::dc_meta(),
        dc_niftyblocksmith::dc_niftyblocksmith(),
        dcg_collection::dcg_collection(),
        dcl_launch::dcl_launch(),
        dg_atari_dillon_francis::dg_atari_dillon_francis(),
        dg_fall_2020::dg_fall_2020(),
        dg_summer_2020::dg_summer_2020(),
        dgtble_headspace::dgtble_headspace(),
        digital_alchemy::digital_alchemy(),
        ethermon_wearables::ethermon_wearables(),
        exclusive_masks::exclusive_masks(),
        halloween_2019::halloween_2019(),
        halloween_2020::halloween_2020(),
        mch_collection::mch_collection(),
        meme_dontbuythis::meme_dontbuythis(),
        mf_sammichgamer::mf_sammichgamer(),
        ml_liondance::ml_liondance(),
        ml_pekingopera::ml_pekingopera(),
        moonshot_2020::moonshot_2020(),
        pm_dreamverse_eminence::pm_dreamverse_eminence(),
        pm_outtathisworld::pm_outtathisworld(),
        rac_basics::rac_basics(),
        release_the_kraken::release_the_kraken(),
        rtfkt_x_atari::rtfkt_x_atari(),
        stay_safe::stay_safe(),
        sugarclub_yumi::sugarclub_yumi(),
        tech_tribal_marc0matic::tech_tribal_marc0matic(),
        threelau_basics::threelau_basics(),
        wearable_test::wearable_test(),
        winklevoss_capital::winklevoss_capital(),
        wonderzone_meteorchaser::wonderzone_meteorchaser(),
        wonderzone_steampunk::wonderzone_steampunk(),
        wz_wonderbot::wz_wonderbot(),
        xmas_2019::xmas_2019(),
        xmas_2020::xmas_2020(),
        xmash_up_2020::xmash_up_2020(),
    ]
}

pub fn find_wearable(wearable_id: &str) -> Option<wearable::Wearable> {
    let all_collections = get_all_collections();
    for collection in all_collections {
        for wearable in collection {
            if wearable.id == wearable_id {
                substreams::log::info!("wearable found");
                return Some(wearable);
            }
        }
    }
    substreams::log::info!("wearable not found {:?}", wearable_id);
    None
}
