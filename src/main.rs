use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::io;
use std::io::Write;
use std::fs;
use std::fs::File;

const RADIO_COUNT: usize = 3;
const PUZZLE_IN_FILEPATH: &str = "musor.txt";
const SEARCH_HITS_FILEPATH: &str = "keres.txt";

#[derive(Copy, Clone, PartialEq)]
struct Time {
    hour: u8,
    min: u8,
    sec: u8,
}

impl Time {
    fn from_sec(sec: u32) -> Self {
        Self {
            hour: (sec / 3600) as u8,
            min: (sec % 3600 / 60) as u8,
            sec: (sec % 60) as u8,
        }
    }

    fn to_sec(self) -> u32 {
        (self.hour as u32) * 3600 + (self.min as u32) * 60 + (self.sec as u32)
    }

    fn add(self, other: Self) -> Self {
        Self::from_sec(self.to_sec() + other.to_sec())
    }

    fn sub(self, other: Self) -> Self {
        Self::from_sec(self.to_sec() - other.to_sec())
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.min, self.sec)
    }
}

#[derive(PartialEq, Debug)]
struct Broadcast {
    radio_id: u8,
    start_time: Time,
    dur_time: Time,
    author: String,
    title: String,
}

fn does_match(text: &str, search_term: &str) -> bool {
    if search_term.len() == 0 {
        return true;
    }

    let text = text.to_lowercase();
    let search_term = search_term.to_lowercase();

    let mut search_term_it = search_term.chars();
    let mut sc = search_term_it.next().unwrap();

    for c in text.chars() {
        if c == sc {
            match search_term_it.next() {
                Some(next_sc) => sc = next_sc,
                None => return true,
            };
        }
    }

    false
}

fn main() {
    let puzzle_in = fs::read_to_string(PUZZLE_IN_FILEPATH).unwrap();
    let mut lines = puzzle_in.trim().split("\n");

    let broadcast_count: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut broadcasts = Vec::<Broadcast>::with_capacity(broadcast_count);

    let mut start_times = [Time { hour: 0, min: 0, sec: 0, }; RADIO_COUNT];

    for line in lines {
        let attrs: Vec<&str> = line.split(" ").collect();

        let third_space_index: usize = attrs[..3].iter().map(|x| x.len() + 1).sum();
        let raw_music = &line[third_space_index..];

        let mut music = raw_music.split_at(raw_music.find(":").unwrap());
        music.1 = music.1[1..].trim();

        let radio_id = attrs[0].parse().unwrap();
        let radio_index = (radio_id as usize) - 1;

        let dur_time = Time {
            hour: 0,
            min: attrs[1].parse().unwrap(),
            sec: attrs[2].parse().unwrap(),
        };

        broadcasts.push(Broadcast {
            radio_id,
            start_time: start_times[radio_index],
            dur_time,
            author: String::from(music.0),
            title: String::from(music.1),
        });

        start_times[radio_index] = start_times[radio_index].add(dur_time);
    }

    let mut broadcast_count_per_radio = [0_usize; RADIO_COUNT];

    for broadcast in broadcasts.iter() {
        broadcast_count_per_radio[(broadcast.radio_id as usize) - 1] += 1;
    }

    println!("2. {:#?}", broadcast_count_per_radio);

    let mut first_eric: Option<&Broadcast> = None;
    let mut last_eric: Option<&Broadcast> = None;

    for broadcast in broadcasts.iter() {
        if broadcast.radio_id != 1 || broadcast.author != "Eric Clapton" { continue; }

        if first_eric == None { first_eric = Some(&broadcast); }
        last_eric = Some(&broadcast);
    }

    print!("3. ");

    match first_eric {
        Some(first_eric_broadcast) => {
            let last_eric_broadcast = last_eric.unwrap();
            let last_eric_end_time = last_eric_broadcast.start_time.add(last_eric_broadcast.dur_time);

            let eric_time_diff = last_eric_end_time.sub(first_eric_broadcast.start_time);

            println!("{:?}", eric_time_diff);
        },
        None => println!("no eric clapton music found"),
    };

    let mut current_broadcasts = [Option::<&Broadcast>::None; RADIO_COUNT];

    for broadcast in broadcasts.iter() {
        current_broadcasts[(broadcast.radio_id as usize) - 1] = Some(&broadcast);

        if broadcast.author != "Omega" || broadcast.title != "Legenda" { continue; }

        print!("4. {}", broadcast.radio_id);

        for current_broadcast in current_broadcasts {
            let current_broadcast = current_broadcast.unwrap();
            if current_broadcast.radio_id == broadcast.radio_id { continue; }
            print!("; {}:{}", current_broadcast.author, current_broadcast.title);
        }

        println!();
        break;
    }

    let mut search_term = String::new();
    io::stdin().read_line(&mut search_term).unwrap();

    let search_term = search_term.trim();

    {
        let mut search_hits_file = File::create(SEARCH_HITS_FILEPATH).unwrap();

        writeln!(&mut search_hits_file, "{}", search_term).unwrap();

        for broadcast in broadcasts.iter() {
            if !does_match(&format!("{}{}", &broadcast.author, &broadcast.title), search_term) { continue; }
            writeln!(&mut search_hits_file, "{}:{}", broadcast.author, broadcast.title).unwrap();
        }
    }

    let mut new_start_time = Time { hour: 0, min: 0, sec: 0, };

    for broadcast in broadcasts.iter() {
        if broadcast.radio_id != 1 { continue; }

        let delta_time = broadcast.dur_time.add(Time { hour: 0, min: 1, sec: 0, });
        let new_start_time_candidate = new_start_time.add(delta_time);

        new_start_time = if new_start_time_candidate.hour > new_start_time.hour {
            delta_time.add(Time {
                hour: new_start_time_candidate.hour,
                min: 3,
                sec: 0,
            })
        }
        else {
            new_start_time_candidate
        };
    }

    println!("6. {:?}", new_start_time);
}

