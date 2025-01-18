#[derive(Clone, Default, Debug, PartialEq)]
pub struct FrequencyStruct {
    pub freq1: f32,
    pub freq2: f32,
    pub freq3: Vec<f32>,
}

// impl FrequencyModel {
//     pub fn freq3_csv(&self) -> String {
//         float_list_to_string(&self.freq3)
//     }
//     pub fn freq3_from_csv(float_list: &str) -> Vec<f32> {
//         string_to_float_list(float_list)
//     }
// }

