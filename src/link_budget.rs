use crate::types::{LinkBudgetParams, LinkBudgetResult};

pub fn calculate_link_budget(path_loss_db: f64, params: &LinkBudgetParams) -> LinkBudgetResult {
    let eirp_dbm = params.tx_power_dbm + params.tx_gain_dbi - params.tx_feeder_loss_db;
    let received_power_dbm =
        eirp_dbm - path_loss_db + params.rx_gain_dbi - params.rx_feeder_loss_db;
    let margin_db = received_power_dbm - params.rx_sensitivity_dbm;

    LinkBudgetResult {
        eirp_dbm,
        path_loss_db,
        received_power_dbm,
        margin_db,
    }
}

pub fn dbm_to_watts(dbm: f64) -> f64 {
    10_f64.powf((dbm - 30.0) / 10.0)
}

pub fn watts_to_dbm(watts: f64) -> f64 {
    10.0 * watts.log10() + 30.0
}

pub fn dbm_to_dbuvm(dbm: f64, frequency_ghz: f64) -> f64 {
    dbm + 77.2 + 20.0 * frequency_ghz.log10()
}

pub fn dbuvm_to_dbm(dbuvm: f64, frequency_ghz: f64) -> f64 {
    dbuvm - 77.2 - 20.0 * frequency_ghz.log10()
}
