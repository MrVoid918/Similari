use crate::utils::bbox::Universal2DBox;
use crate::utils::nms::nms;
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(
    name = "nms",
    text_signature = "(detections, nms_threshold, score_threshold)"
)]
pub fn nms_py(
    detections: Vec<(Universal2DBox, Option<f32>)>,
    nms_threshold: f32,
    score_threshold: Option<f32>,
) -> Vec<Universal2DBox> {
    nms(&detections, nms_threshold, score_threshold)
        .into_iter()
        .cloned()
        .collect()
}

//
// #[pyfunction]
// #[pyo3(
//     name = "parallel_nms",
//     text_signature = "(detections, nms_threshold, score_threshold)"
// )]
// pub fn parallel_nms_py(
//     detections: Vec<(Universal2DBox, Option<f32>)>,
//     nms_threshold: f32,
//     score_threshold: Option<f32>,
// ) -> Vec<Universal2DBox> {
//     parallel_nms(&detections, nms_threshold, score_threshold)
//         .into_iter()
//         .cloned()
//         .collect()
// }
