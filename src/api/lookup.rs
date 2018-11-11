use phf;

static BUSCH: &'static str = "Busch";
static COLLEGE_AVE: &'static str = "College Ave";

static STOP_CAMPUSES: phf::Map<&'static str, &'static str> = phf_map! {
    "scott" => COLLEGE_AVE,
    "hill" => BUSCH
};

pub fn stop_campus(stop: &str) -> String {
    STOP_CAMPUSES.get(stop).unwrap_or(&"Unknown").to_string()
}
