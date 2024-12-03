use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Curtois2014Instance {
    pub horizon : i32,
    pub shifts : Vec<Shift>,
    pub staff: Vec<Staff>,
    pub daysoff : HashMap<String, Vec<i32>>,
    pub shiftrequests : Vec<Request>,
    pub offrequests : Vec<Request>,
    pub cover : Vec<Requirement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Shift {
    pub id : String,
    pub length : i32,
    pub forbidden_successors_ids : Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Staff {
    pub id : String,
    pub maxshifts : HashMap<String,i32>,
    pub maxminutes : i32,
    pub minminutes: i32,
    pub maxconsecutiveshifts : i32,
    pub minconsecutiveshifts : i32,
    pub minconsecutiveoff : i32,
    pub maxweekends : i32,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub staffid : String,
    pub day : i32,
    pub shiftid : String,
    pub weight : i32
}

#[derive(Debug, Clone)]
pub struct Requirement {
    pub day : i32,
    pub shift_id : String,
    pub required : i32,
    pub cost_under : i32,
    pub cost_over : i32
}

pub fn parse_curtois2014(filepath : &str) -> Curtois2014Instance {
    let mut instance = Curtois2014Instance {
        horizon: 0,
        shifts: vec![],
        staff: vec![],
        daysoff: Default::default(),
        shiftrequests: vec![],
        offrequests: vec![],
        cover: vec![],
    };
    read_file_line_by_line(filepath, &mut instance).expect("Panicked");

    instance
}

fn read_file_line_by_line(filepath: &str, mut instance: &mut Curtois2014Instance) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);

    read_horizon(&mut reader, &mut instance);
    Ok(())
}

fn read_horizon(mut reader : &mut BufReader<File>, mut instance: &mut Curtois2014Instance) {

    get_next_data_entry(&mut reader);
    let line = get_next_data_entry(&mut reader).0;

    instance.horizon = line.trim().parse::<i32>().unwrap();
    read_shifts(&mut reader, instance);
}

fn read_shifts(mut reader : &mut BufReader<File>, mut instance: &mut Curtois2014Instance) {

    get_next_data_entry(&mut reader);
    let mut line = get_next_data_entry(&mut reader).0;
    let mut shifts : Vec<Shift> = vec![];

    while line.as_str().trim() != "SECTION_STAFF" {
        //println!("{}", line);
        let parts : Vec<&str> = line.trim().split(",").collect();
        let mut forbidden_successors_ids : Vec<String> = vec![];

        if parts.len() > 2 {
            if parts[2] != "".to_string() {
                forbidden_successors_ids = parts[2].trim().split("|").map(|s| s.to_string()).collect();
            }
        }

        let shift = Shift {
            id: parts[0].clone().to_string(),
            length: parts[1].clone().parse().unwrap(),
            forbidden_successors_ids,
        };
        shifts.push(shift);

        line = get_next_data_entry(&mut reader).0;
    }

    instance.shifts = shifts;

    read_staff(&mut reader, &mut instance);
}

fn read_staff(mut reader : &mut BufReader<File>, mut instance : &mut Curtois2014Instance) {
    let mut line = get_next_data_entry(reader).0;
    let mut staff: Vec<Staff> = vec![];
    let mut index = 0;

    while line.as_str().trim() != "SECTION_DAYS_OFF" {
        let parts : Vec<String>= line.split(",").map(|s| s.to_string()).collect();
        let mut maxshifts: HashMap<String, i32> = HashMap::new();
        let tuples : Vec<String> = parts[1].split("|").map(|s| s.to_string()).collect();

        for tuple in tuples {
            let assignment : Vec<String> = tuple.split("=").map(|s| s.to_string()).collect();
            maxshifts.insert(assignment[0].clone(), assignment[1].parse::<i32>().unwrap());
        }

        let member = Staff{
            id: parts[0].clone(),
            maxshifts,
            maxminutes: parts[2].trim().parse::<i32>().unwrap(),
            minminutes: parts[3].trim().parse::<i32>().unwrap(),
            maxconsecutiveshifts: parts[4].trim().parse::<i32>().unwrap(),
            minconsecutiveshifts: parts[5].trim().parse::<i32>().unwrap(),
            minconsecutiveoff: parts[6].trim().parse::<i32>().unwrap(),
            maxweekends: parts[7].trim().parse::<i32>().unwrap(),
        };

        staff.push(member);
        line = get_next_data_entry(&mut reader).0;
        index = index + 1;
    }
    instance.staff = staff;
    read_daysoff(&mut reader, &mut instance);
}

fn read_daysoff(mut reader : &mut BufReader<File>, mut instance : &mut Curtois2014Instance) {
    let mut line = get_next_data_entry(&mut reader).0;
    let mut daysoff = HashMap::<String, Vec<i32>>::new();

    while line.as_str().trim() != "SECTION_SHIFT_ON_REQUESTS" {
        let mut parts : Vec<String> = line.split(",").map(|s| s.trim().to_string()).collect();
        let mut days = vec![];
        days.extend(&parts[1..parts.len()]);
        daysoff.insert(parts[0].clone(),days.iter().map(|s| s.parse::<i32>().unwrap()).collect());

        line = get_next_data_entry(&mut reader).0;
    };
    instance.daysoff = daysoff;
    read_shift_requests(&mut reader, &mut instance);
}

fn read_shift_requests(mut reader : &mut BufReader<File>, mut instance : &mut Curtois2014Instance) {
    let mut line = get_next_data_entry(&mut reader).0;
    let mut requests = vec![];

    while line.as_str().trim() != "SECTION_SHIFT_OFF_REQUESTS" {
        let mut parts : Vec<String>= line.split(",").map(String::from).collect();
        let request = Request {
            staffid: parts[0].clone(),
            day: parts[1].parse().unwrap(),
            shiftid: parts[2].clone(),
            weight: parts[3].trim().parse().unwrap(),
        };
        requests.push(request);

        line = get_next_data_entry(&mut reader).0;
    }
    instance.shiftrequests = requests;
    read_off_requests(&mut reader, &mut instance);
}

fn read_off_requests(mut reader : &mut BufReader<File>, mut instance : &mut Curtois2014Instance) {
    let mut line = get_next_data_entry(&mut reader).0;
    let mut requests = vec![];

    while line.as_str().trim() != "SECTION_COVER" {
        let mut parts : Vec<String>= line.split(",").map(String::from).collect();
        let request = Request {
            staffid: parts[0].clone(),
            day: parts[1].parse().unwrap(),
            shiftid: parts[2].clone(),
            weight: parts[3].trim().parse().unwrap(),
        };
        requests.push(request);

        line = get_next_data_entry(&mut reader).0;
    }
    instance.offrequests = requests;
    read_cover(&mut reader, &mut instance);
}

fn read_cover(mut reader : &mut BufReader<File>, mut instance : &mut Curtois2014Instance) {
    let mut line = get_next_data_entry(&mut reader).0;
    let mut eof = false;

    while !eof {

        let mut parts : Vec<String> = line.split(",").map(String::from).collect();
        let day : i32 = parts[0].parse().unwrap();

        let requirement = Requirement {
            day,
            shift_id: parts[1].clone(),
            required: parts[2].parse().unwrap(),
            cost_under: parts[3].parse().unwrap(),
            cost_over: parts[4].trim().parse().unwrap(),
        };

        instance.cover.push(requirement);

        let tuple = get_next_data_entry(&mut reader);
        line = tuple.0;
        if tuple.1 == 0 { eof = true}
    }
}

fn get_next_data_entry(mut reader : &mut BufReader<File>) -> (String, usize) {

    let mut line = String::new();
    let mut bytes = reader.read_line(&mut line).expect("Panick on read");
    if bytes == 0 { return (line, 0)}
    let mut charvec = line.chars().collect::<Vec<_>>();

    while line.trim().is_empty() | (charvec[0] == '#') {
        line.clear();
        bytes = reader.read_line(&mut line).expect("Panick on read");
        charvec = line.chars().collect::<Vec<_>>();
        if bytes == 0 {break;}
    }
    (line, bytes)
}