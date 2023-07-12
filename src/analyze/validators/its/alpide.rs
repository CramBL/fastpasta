//! This module is the parent module for all ALPIDE data validation.
//!
//! It contains some utility functions, and then it publishes modules with more specific ALPIDE related functionality.

pub mod alpide_readout_frame;
pub mod lane_alpide_frame_analyzer;

use alpide_readout_frame::AlpideReadoutFrame;
use itertools::Itertools;
use lane_alpide_frame_analyzer::LaneAlpideFrameAnalyzer;

use crate::util::config::custom_checks::CustomChecksOpt;

// Helper struct to group lanes and bunch counters, used for comparing bunch counters between lanes
struct ValidatedLane {
    lane_id: u8,
    bunch_counter: u8,
}

/// Process ALPIDE data for a readout frame, per lane.
///
/// Returns a tuple of a vector of lane ids with errors, and a vector of error messages.
pub fn check_alpide_data_frame(
    mut alpide_readout_frame: AlpideReadoutFrame,
    custom_checks: &'static impl CustomChecksOpt,
) -> (Vec<u8>, Vec<String>) {
    let mut lane_error_msgs: Vec<String> = Vec::new();
    let mut lane_error_ids: Vec<u8> = Vec::new();
    let mut validated_lanes: Vec<ValidatedLane> = Vec::new();

    let frame_from_layer = alpide_readout_frame.from_layer();

    alpide_readout_frame
        .take_lane_data_frames()
        .into_iter()
        .for_each(|lane_data_frame| {
            // Process data for each lane
            // New decoder for each lane
            let mut analyzer = LaneAlpideFrameAnalyzer::new(
                frame_from_layer,
                custom_checks.chip_orders_ob(),
                custom_checks.chip_count_ob(),
            );
            let lane_number = lane_data_frame.lane_number(frame_from_layer);
            log::trace!("Processing lane #{lane_number}");

            if let Err(mut error_msgs) = analyzer.analyze_alpide_frame(lane_data_frame) {
                error_msgs.insert_str(0, &format!("\n\tLane {lane_number} errors: "));
                lane_error_msgs.push(error_msgs);
                lane_error_ids.push(lane_number);
            } else {
                // If the bunch counter is validated for this lane, add it to the list of validated lanes.
                validated_lanes.push(ValidatedLane {
                    lane_id: lane_number,
                    bunch_counter: analyzer
                        .validated_bc()
                        .expect("No validated bunch counter in lane readout frame with no errors"),
                });
            }
        });

    // Compare all validated bunch counters to each other across lanes
    validate_lane_bcs(&validated_lanes, &mut lane_error_msgs, &mut lane_error_ids);

    (lane_error_ids, lane_error_msgs)
}

/// Compare all validated bunch counters to each other across lanes
fn validate_lane_bcs(
    validated_lanes: &[ValidatedLane],
    lane_error_msgs: &mut Vec<String>, // Just to reduce the amount of copying...
    lane_error_ids: &mut Vec<u8>,      // Just to reduce the amount of copying...
) {
    let unique_bunch_counters: Vec<u8> = validated_lanes
        .iter()
        .map(|lane| lane.bunch_counter)
        .collect::<Vec<u8>>()
        .into_iter()
        .unique()
        .collect();
    if unique_bunch_counters.len() > 1 {
        let mut error_string = format!(
            "\n\tLane {:?} error: Mismatching bunch counters between lanes in same readout frame",
            validated_lanes
                .iter()
                .map(|lane| lane.lane_id)
                .collect::<Vec<u8>>()
        );
        // Find the lanes with each bunch counter
        let mut lanes_to_bunch_counter: Vec<(u8, Vec<u8>)> = Vec::new();
        // Iterate through each unique bunch counter
        unique_bunch_counters.iter().for_each(|bunch_counter| {
            // Collect all lanes with this bunch counter
            lanes_to_bunch_counter.push((
                *bunch_counter,
                validated_lanes
                    .iter()
                    .filter(|lane| lane.bunch_counter == *bunch_counter)
                    .map(|lane| lane.lane_id)
                    .collect::<Vec<u8>>(),
            ));
        });
        // Add the lanes to the error string
        lanes_to_bunch_counter
            .iter()
            .for_each(|(bunch_counter, lanes)| {
                error_string.push_str(&format!(
                    "\n\t\tBunch counter: {bunch_counter:>3?} | Lanes: {lanes:?}",
                    bunch_counter = bunch_counter,
                    lanes = lanes
                ));
            });
        lane_error_msgs.push(error_string);
        lane_error_ids.extend(lanes_to_bunch_counter.iter().flat_map(|(_, lanes)| lanes));
    }
}
