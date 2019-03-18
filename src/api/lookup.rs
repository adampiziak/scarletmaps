use phf;

static BUSCH: &'static str = "Busch";
static COLLEGE_AVE: &'static str = "College Ave";
static NEWARK: &'static str = "Newark";
static COOK_DOUGLAS: &'static str = "Cook Douglas";
static GEORGE_STREET: &'static str = "George Street";
static LIVINGSTON: &'static str = "Livingston";


static STOP_INFO: phf::Map<i32, (&'static str, &'static str)> = phf_map! {
    4229620i32 => ("scott_hall", COLLEGE_AVE),
    4229524i32 => ("busch_student_center", BUSCH),
    4229520i32 => ("busch_suites", BUSCH),
    4229634i32 => ("college_ave_student_center", NEWARK),
    4229578i32 => ("allison_road_classrooms", BUSCH),
    4229636i32 => ("bergn_building", NEWARK),
    4229600i32 => ("biel_road", COOK_DOUGLAS),
    4229638i32 => ("blumenthal_hall", NEWARK),
    4229640i32 => ("boyden_hall", NEWARK),
    4229582i32 => ("bravo_supermarket", GEORGE_STREET),
    4229690i32 => ("broad_st", NEWARK),
    4229528i32 => ("buell_apartments", BUSCH),
    4229644i32 => ("clj", NEWARK),
    4229492i32 => ("college_ave_student_center", COLLEGE_AVE),
    4229550i32 => ("college_hall", COOK_DOUGLAS),
    4229566i32 => ("davidson_hall", BUSCH),
    4229648i32 => ("dental_school", NEWARK),
    4229650i32 => ("ecc", NEWARK),
    4229542i32 => ("food_sciences_building", COOK_DOUGLAS),
    4229612i32 => ("george_street_northbound", GEORGE_STREET),
    4229616i32 => ("george_street_northbound_paterson", GEORGE_STREET),
    4229698i32 => ("george_street_southbound", GEORGE_STREET),
    4229608i32 => ("gibbons", COOK_DOUGLAS),
    4229604i32 => ("henderson", COOK_DOUGLAS),
    4229508i32 => ("busch", BUSCH),
    4229646i32 => ("clinical_academic_building", COLLEGE_AVE),
    4229652i32 => ("frank_rodgers_blvd_and_cleveland", NEWARK),
    4229592i32 => ("public_safety_building", GEORGE_STREET),
    4229500i32 => ("stadium", BUSCH),
    4229496i32 => ("student_activities_center_northbound", COLLEGE_AVE),
    4229686i32 => ("university_north", NEWARK),
    4229688i32 => ("washington_park", NEWARK),
    4229504i32 => ("werblin_back_entrance", BUSCH),
    4229562i32 => ("werblin_main_entrance", BUSCH),
    4229558i32 => ("zimmerli_arts_museum", COLLEGE_AVE),
    4229656i32 => ("harrison_passaic", NEWARK),
    4229654i32 => ("hospital", NEWARK),
    4229684i32 => ("shoprite", NEWARK),
    4229512i32 => ("science_building", BUSCH),
    4229588i32 => ("rockoff_hall", COOK_DOUGLAS),
    4229538i32 => ("red_oak_lane", COOK_DOUGLAS),
    4229576i32 => ("quads", LIVINGSTON),
    4229678i32 => ("physical_plant", NEWARK),
    4229676i32 => ("penn_station", NEWARK),
    4229624i32 => ("nursing_school", COLLEGE_AVE),
    4229554i32 => ("northbound_public_safety", GEORGE_STREET),
    4229532i32 => ("train_station_george_street", COLLEGE_AVE),
    4229536i32 => ("train_station_somerset", COLLEGE_AVE),
    4229674i32 => ("NJIT", NEWARK),
    4229672i32 => ("medical_school_arrival", NEWARK),
    4229670i32 => ("medical_school", NEWARK),
    4229574i32 => ("livingston_student_center", LIVINGSTON),
    4229570i32 => ("livingston_plaza", LIVINGSTON),
    4229658i32 => ("ICPH", NEWARK),
    4229546i32 => ("katzenbach", COOK_DOUGLAS),
    4229660i32 => ("kearny_ave_dukes", NEWARK),
    4229662i32 => ("kearny_ave_bergen", NEWARK),
    4229664i32 => ("kearny_ave_midland", NEWARK),
    4229666i32 => ("kearny_ave_quincy", NEWARK),
    4229668i32 => ("kmart", NEWARK),
    4229516i32 => ("library_of_science", BUSCH),
    4229596i32 => ("lipman_hall", COOK_DOUGLAS),
    4230628i32 => ("livingston_health_center", LIVINGSTON),
};

pub fn get_stop_area(id: &i32) -> String {
    STOP_INFO.get(id).unwrap_or(&("", "unknown")).1.to_string()
}

pub fn get_route_areas(ids: &Vec<i32>) -> Vec<String> {
    let mut areas = Vec::new();
    for id in ids {
        let area = get_stop_area(&id);
        if (area == "unknown") {
            continue;
        }

        if !areas.contains(&area) {
            areas.push(area);
        }
    }
    areas
}
