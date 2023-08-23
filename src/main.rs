use std::cmp::Ordering;
use rand::Rng;
use std::thread;


const TEAM_AMOUNT: usize = 16;
const ROUNDS: usize = 5;
const ADVANCE: usize = 3;

#[allow(dead_code)]
const PLAYER: i32 = 2; //0: Alex, 1: An, 2: Fe
const OUTPUT: bool = false;

#[derive(Copy, Clone)]
enum MatchResult {
    Won,
    Lost
}

#[derive(Copy, Clone)]
enum PickEmBet {
    ThreeZero,
    Advance,
    ZeroThree,
    Nothing
}


struct Team {
    name: String,
    seeding: u32,

    enemies: Vec<usize>,
    results: Vec<MatchResult>,
    buchholz: i32,

    active: bool, // Indicates that a Team has not yet advanced or been eliminated

    pickems: Vec<PickEmBet>
}

impl Team {
    /*
    #[allow(dead_code)]
    fn new(name: &str, seeding: u32, pickem: PickEmBet) -> Team {
        let name = String::from(name);
        Team {
            name,
            seeding,
            enemies: Vec::new(),
            results: Vec::new(),
            buchholz: 0,
            active: true,
            pickem
        }
    }
    */
    fn new_with_matches(name: &str, seeding: u32, pickems: Vec<PickEmBet>, results: Vec<MatchResult>) -> Team {
        let name = String::from(name);
        Team {
            name,
            seeding,
            enemies: Vec::new(),
            results,
            buchholz: 0,
            active: true,
            pickems
        }
    }
    /*
    #[allow(dead_code)]
    fn get_all_challenger_teams() -> [Team; TEAM_AMOUNT] {
        [
            Team::new("ENCE", 9, PickEmBet::Advance),
            Team::new("G2", 10, PickEmBet::ThreeZero),
            Team::new("Forze", 11, PickEmBet::Nothing),
            Team::new("Astralis", 12, PickEmBet::Advance),
            Team::new("Vitality", 13, PickEmBet::Advance),
            Team::new("mibr", 14, PickEmBet::Advance),
            Team::new("Imperial", 15, PickEmBet::Nothing),
            Team::new("Bad News Eagles", 16, PickEmBet::Advance),
            Team::new("Eternal Fire", 17, PickEmBet::Nothing),
            Team::new("Team Spirit", 18, PickEmBet::Nothing),
            Team::new("Outsiders", 19, PickEmBet::Advance),
            Team::new("Complexity", 20, PickEmBet::Nothing),
            Team::new("IHC", 21, PickEmBet::Nothing),
            Team::new("Renegades", 22, PickEmBet::Nothing),
            Team::new("Team Liquid", 23, PickEmBet::Advance),
            Team::new("9z", 24, PickEmBet::ZeroThree),
        ]
    }

    #[allow(dead_code)]
    fn get_all_challenger_teams_with_matches() -> [Team; TEAM_AMOUNT] {
        [
            Team::new_with_matches("ENCE", 9, PickEmBet::Advance, vec![MatchResult::Won, MatchResult::Won, MatchResult::Lost, MatchResult::Won]),
            Team::new_with_matches("G2", 10, PickEmBet::ThreeZero, vec![MatchResult::Won, MatchResult::Won, MatchResult::Won]),
            Team::new_with_matches("Forze", 11, PickEmBet::Nothing, vec![MatchResult::Won, MatchResult::Won, MatchResult::Lost, MatchResult::Lost, MatchResult::Lost]),
            Team::new_with_matches("Astralis", 12, PickEmBet::Advance, vec![MatchResult::Won, MatchResult::Lost, MatchResult::Won, MatchResult::Lost, MatchResult::Lost]),
            Team::new_with_matches("Vitality", 13, PickEmBet::Advance, vec![MatchResult::Won, MatchResult::Won, MatchResult::Won]),
            Team::new_with_matches("mibr", 14, PickEmBet::Advance, vec![MatchResult::Lost, MatchResult::Won, MatchResult::Lost, MatchResult::Won, MatchResult::Lost]),
            Team::new_with_matches("Imperial", 15, PickEmBet::Nothing, vec![MatchResult::Lost, MatchResult::Won, MatchResult::Lost, MatchResult::Won, MatchResult::Won]),
            Team::new_with_matches("Bad News Eagles", 16, PickEmBet::Advance, vec![MatchResult::Won, MatchResult::Lost, MatchResult::Won, MatchResult::Lost, MatchResult::Won]),
            Team::new_with_matches("Eternal Fire", 17, PickEmBet::Nothing, vec![MatchResult::Lost, MatchResult::Won, MatchResult::Lost, MatchResult::Lost]),
            Team::new_with_matches("Team Spirit", 18, PickEmBet::Nothing, vec![MatchResult::Won, MatchResult::Lost, MatchResult::Won, MatchResult::Won]),
            Team::new_with_matches("Outsiders", 19, PickEmBet::Advance, vec![MatchResult::Won, MatchResult::Lost, MatchResult::Won, MatchResult::Won]),
            Team::new_with_matches("Complexity", 20, PickEmBet::Nothing, vec![MatchResult::Lost, MatchResult::Won, MatchResult::Lost, MatchResult::Lost]),
            Team::new_with_matches("IHC", 21, PickEmBet::Nothing, vec![MatchResult::Lost, MatchResult::Lost, MatchResult::Won, MatchResult::Lost]),
            Team::new_with_matches("Renegades", 22, PickEmBet::Nothing, vec![MatchResult::Lost, MatchResult::Lost, MatchResult::Lost]),
            Team::new_with_matches("Team Liquid", 23, PickEmBet::Advance, vec![MatchResult::Lost, MatchResult::Lost, MatchResult::Won, MatchResult::Won, MatchResult::Won]),
            Team::new_with_matches("9z", 24, PickEmBet::ZeroThree, vec![MatchResult::Lost, MatchResult::Lost, MatchResult::Lost]),
        ]
    }
    */

    fn get_all_legends_team() -> [Team; TEAM_AMOUNT] {

        let navi_matches: Vec<MatchResult> =     vec![MatchResult::Won , MatchResult::Lost, MatchResult::Won , MatchResult::Lost];
        let nine_matches: Vec<MatchResult> =     vec![MatchResult::Lost, MatchResult::Lost, MatchResult::Lost];
        let furia_matches: Vec<MatchResult> =    vec![MatchResult::Lost, MatchResult::Lost, MatchResult::Lost];
        let fnatic_matches: Vec<MatchResult> =   vec![MatchResult::Won , MatchResult::Lost, MatchResult::Lost, MatchResult::Won ];
        let heroic_matches: Vec<MatchResult> =   vec![MatchResult::Won , MatchResult::Won , MatchResult::Won ];
        let itb_matches: Vec<MatchResult> =      vec![MatchResult::Lost, MatchResult::Won , MatchResult::Won , MatchResult::Lost];
        let vitality_matches: Vec<MatchResult> = vec![MatchResult::Won , MatchResult::Won , MatchResult::Won ];
        let bne_matches: Vec<MatchResult> =      vec![MatchResult::Lost, MatchResult::Won , MatchResult::Lost, MatchResult::Lost];
        let ence_matches: Vec<MatchResult> =     vec![MatchResult::Won , MatchResult::Lost, MatchResult::Lost, MatchResult::Lost];
        let g2_matches: Vec<MatchResult> =       vec![MatchResult::Lost, MatchResult::Lost, MatchResult::Won , MatchResult::Lost];
        let apeks_matches: Vec<MatchResult> =    vec![MatchResult::Won , MatchResult::Lost, MatchResult::Won ];
        let faze_matches: Vec<MatchResult> =     vec![MatchResult::Lost, MatchResult::Lost, MatchResult::Won , MatchResult::Won ];
        let nip_matches: Vec<MatchResult> =      vec![MatchResult::Lost, MatchResult::Won , MatchResult::Lost, MatchResult::Won ];
        let monte_matches: Vec<MatchResult> =    vec![MatchResult::Won , MatchResult::Won , MatchResult::Lost, MatchResult::Won ];
        let liquid_matches: Vec<MatchResult> =   vec![MatchResult::Won , MatchResult::Won , MatchResult::Lost, MatchResult::Won ];
        let gl_matches: Vec<MatchResult> =       vec![MatchResult::Lost, MatchResult::Won , MatchResult::Won ];

        //let heroic_matches: Vec<MatchResult> = vec![MatchResult::Won, MatchResult::Lost, MatchResult::Won, MatchResult::Lost, MatchResult::Won];
        
        let navi_picks: Vec<PickEmBet> =     vec![PickEmBet::Advance,   PickEmBet::Advance,   PickEmBet::Advance];
        let nine_picks: Vec<PickEmBet> =     vec![PickEmBet::Nothing,   PickEmBet::Advance,   PickEmBet::ThreeZero];
        let furia_picks: Vec<PickEmBet> =    vec![PickEmBet::Advance,   PickEmBet::Advance,   PickEmBet::Advance];
        let fnatic_picks: Vec<PickEmBet> =   vec![PickEmBet::Nothing,   PickEmBet::Nothing,   PickEmBet::Nothing];
        let heroic_picks: Vec<PickEmBet> =   vec![PickEmBet::Advance,   PickEmBet::Advance,   PickEmBet::Advance];
        let itb_picks: Vec<PickEmBet> =      vec![PickEmBet::ZeroThree, PickEmBet::Nothing,   PickEmBet::ZeroThree];
        let vitality_picks: Vec<PickEmBet> = vec![PickEmBet::Advance,   PickEmBet::Advance,   PickEmBet::Advance];
        let bne_picks: Vec<PickEmBet> =      vec![PickEmBet::Nothing,   PickEmBet::Nothing,   PickEmBet::Nothing];
        let ence_picks: Vec<PickEmBet> =     vec![PickEmBet::ThreeZero, PickEmBet::ThreeZero, PickEmBet::Advance];
        let g2_picks: Vec<PickEmBet> =       vec![PickEmBet::Advance,   PickEmBet::Advance,   PickEmBet::Advance];
        let apeks_picks: Vec<PickEmBet> =    vec![PickEmBet::Nothing,   PickEmBet::Nothing,   PickEmBet::Nothing];
        let faze_picks: Vec<PickEmBet> =     vec![PickEmBet::Advance,   PickEmBet::Advance,   PickEmBet::Advance];
        let nip_picks: Vec<PickEmBet> =      vec![PickEmBet::Nothing,   PickEmBet::Nothing,   PickEmBet::Nothing];
        let monte_picks: Vec<PickEmBet> =    vec![PickEmBet::Nothing,   PickEmBet::Nothing,   PickEmBet::Nothing];
        let liquid_picks: Vec<PickEmBet> =   vec![PickEmBet::Advance,   PickEmBet::Nothing,   PickEmBet::Nothing];
        let gl_picks: Vec<PickEmBet> =       vec![PickEmBet::Nothing,   PickEmBet::ZeroThree, PickEmBet::Nothing];

        return [
                Team::new_with_matches("NAVI", 1, navi_picks, navi_matches),
                Team::new_with_matches("9INE", 2, nine_picks, nine_matches),
                Team::new_with_matches("Team Furia", 3, furia_picks, furia_matches),
                Team::new_with_matches("Fnatic", 4, fnatic_picks, fnatic_matches),
                Team::new_with_matches("Heroic", 5, heroic_picks, heroic_matches),
                Team::new_with_matches("Into The Breach", 6, itb_picks, itb_matches),
                Team::new_with_matches("Vitality", 7, vitality_picks, vitality_matches),
                Team::new_with_matches("Bad News Eagles", 8, bne_picks, bne_matches),
                Team::new_with_matches("ENCE", 9, ence_picks, ence_matches),
                Team::new_with_matches("G2", 10, g2_picks, g2_matches),
                Team::new_with_matches("Apeks", 11, apeks_picks, apeks_matches),
                Team::new_with_matches("Faze", 12, faze_picks, faze_matches),
                Team::new_with_matches("NIP", 13, nip_picks, nip_matches),
                Team::new_with_matches("Monte", 14, monte_picks, monte_matches),
                Team::new_with_matches("Team Liquid", 15, liquid_picks, liquid_matches),
                Team::new_with_matches("Gamer Legion", 16, gl_picks, gl_matches),
            ]
        /*
        if player == 0 {
            return [
                Team::new_with_matches("NAVI", 1, PickEmBet::Advance, navi_matches),
                Team::new_with_matches("9INE", 2, PickEmBet::Nothing, nine_matches),
                Team::new_with_matches("Team Furia", 3, PickEmBet::Advance, furia_matches),
                Team::new_with_matches("Fnatic", 4, PickEmBet::Nothing, fnatic_matches),
                Team::new_with_matches("Heroic", 5, PickEmBet::Advance, heroic_matches),
                Team::new_with_matches("Into The Breach", 6, PickEmBet::ZeroThree, itb_matches),
                Team::new_with_matches("Vitality", 7, PickEmBet::Advance, vitality_matches),
                Team::new_with_matches("Bad News Eagles", 8, PickEmBet::Nothing, bne_matches),
                Team::new_with_matches("ENCE", 9, PickEmBet::ThreeZero, ence_matches),
                Team::new_with_matches("G2", 10, PickEmBet::Advance, g2_matches),
                Team::new_with_matches("Apeks", 11, PickEmBet::Nothing, apeks_matches),
                Team::new_with_matches("Faze", 12, PickEmBet::Advance, faze_matches),
                Team::new_with_matches("NIP", 13, PickEmBet::Nothing, nip_matches),
                Team::new_with_matches("Monte", 14, PickEmBet::Nothing, monte_matches),
                Team::new_with_matches("Team Liquid", 15, PickEmBet::Advance, liquid_matches),
                Team::new_with_matches("Gamer Legion", 16, PickEmBet::Nothing, gl_matches),
            ]
        } else if player == 1 {
            return [
                Team::new_with_matches("NAVI", 1, PickEmBet::Advance, navi_matches),
                Team::new_with_matches("9INE", 2, PickEmBet::Advance, nine_matches),
                Team::new_with_matches("Team Furia", 3, PickEmBet::Advance, furia_matches),
                Team::new_with_matches("Fnatic", 4, PickEmBet::Nothing, fnatic_matches),
                Team::new_with_matches("Heroic", 5, PickEmBet::Advance, heroic_matches),
                Team::new_with_matches("Into The Breach", 6, PickEmBet::Nothing, itb_matches),
                Team::new_with_matches("Vitality", 7, PickEmBet::Advance, vitality_matches),
                Team::new_with_matches("Bad News Eagles", 8, PickEmBet::Nothing, bne_matches),
                Team::new_with_matches("ENCE", 9, PickEmBet::ThreeZero, ence_matches),
                Team::new_with_matches("G2", 10, PickEmBet::Advance, g2_matches),
                Team::new_with_matches("Apeks", 11, PickEmBet::Nothing, apeks_matches),
                Team::new_with_matches("Faze", 12, PickEmBet::Advance, faze_matches),
                Team::new_with_matches("NIP", 13, PickEmBet::Nothing, nip_matches),
                Team::new_with_matches("Monte", 14, PickEmBet::Nothing, monte_matches),
                Team::new_with_matches("Team Liquid", 15, PickEmBet::Nothing, liquid_matches),
                Team::new_with_matches("Gamer Legion", 16, PickEmBet::ZeroThree, gl_matches),
            ]
        } else {
            return [
                Team::new_with_matches("NAVI", 1, PickEmBet::Advance, navi_matches),
                Team::new_with_matches("9INE", 2, PickEmBet::ThreeZero, nine_matches),
                Team::new_with_matches("Team Furia", 3, PickEmBet::Advance, furia_matches),
                Team::new_with_matches("Fnatic", 4, PickEmBet::Nothing, fnatic_matches),
                Team::new_with_matches("Heroic", 5, PickEmBet::Advance, heroic_matches),
                Team::new_with_matches("Into The Breach", 6, PickEmBet::ZeroThree, itb_matches),
                Team::new_with_matches("Vitality", 7, PickEmBet::Advance, vitality_matches),
                Team::new_with_matches("Bad News Eagles", 8, PickEmBet::Nothing, bne_matches),
                Team::new_with_matches("ENCE", 9, PickEmBet::Advance, ence_matches),
                Team::new_with_matches("G2", 10, PickEmBet::Advance, g2_matches),
                Team::new_with_matches("Apeks", 11, PickEmBet::Nothing, apeks_matches),
                Team::new_with_matches("Faze", 12, PickEmBet::Advance, faze_matches),
                Team::new_with_matches("NIP", 13, PickEmBet::Nothing, nip_matches),
                Team::new_with_matches("Monte", 14, PickEmBet::Nothing, monte_matches),
                Team::new_with_matches("Team Liquid", 15, PickEmBet::Nothing, liquid_matches),
                Team::new_with_matches("Gamer Legion", 16, PickEmBet::Nothing, gl_matches),
            ]
        }
        */
        
    }

    /*
    #[allow(dead_code)]
    fn get_all_legends_team_someone() -> [Team; TEAM_AMOUNT] {
        [
            Team::new("Heroic", 1, PickEmBet::Advance),
            Team::new("Copenhagen Flames", 2, PickEmBet::Nothing),
            Team::new("BIG", 3, PickEmBet::Nothing),
            Team::new("Cloud 9", 4, PickEmBet::Advance),
            Team::new("Team Furia", 5, PickEmBet::ThreeZero),
            Team::new("Faze", 6, PickEmBet::Advance),
            Team::new("NIP", 7, PickEmBet::ZeroThree),
            Team::new("NAVI", 8, PickEmBet::Advance),
            Team::new("G2", 9, PickEmBet::Advance),
            Team::new("Vitality", 10, PickEmBet::Nothing),
            Team::new("ENCE", 11, PickEmBet::Advance),
            Team::new("Team Spirit", 12, PickEmBet::Advance),
            Team::new("Outsiders", 13, PickEmBet::Nothing),
            Team::new("Imperial", 14, PickEmBet::Nothing),
            Team::new("Bad News Eagles", 15, PickEmBet::Nothing),
            Team::new("Team Liquid", 16, PickEmBet::Nothing),
        ]
    }

    #[allow(dead_code)]
    fn get_all_challengers_team_pickem() -> [Team; TEAM_AMOUNT] {

        let ence_matches: Vec<MatchResult> = vec![MatchResult::Won, MatchResult::Won, MatchResult::Lost];
        let g2_matches: Vec<MatchResult> = vec![MatchResult::Won, MatchResult::Won, MatchResult::Won];
        let forze_matches: Vec<MatchResult> = vec![MatchResult::Won, MatchResult::Won, MatchResult::Lost, MatchResult::Lost];
        let astralis_matches: Vec<MatchResult> = vec![MatchResult::Won, MatchResult::Lost, MatchResult::Won, MatchResult::Lost];
        let vitality_matches: Vec<MatchResult> = vec![MatchResult::Won, MatchResult::Won, MatchResult::Won];
        let mibr_matches: Vec<MatchResult> = vec![MatchResult::Lost, MatchResult::Won, MatchResult::Lost];
        let imperial_matches: Vec<MatchResult> = vec![MatchResult::Lost, MatchResult::Won, MatchResult::Lost, MatchResult::Won];
        let bne_matches: Vec<MatchResult> = vec![MatchResult::Won, MatchResult::Lost, MatchResult::Won];
        let eternal_fire_matches: Vec<MatchResult> = vec![MatchResult::Lost, MatchResult::Won, MatchResult::Lost];
        let spirit_matches: Vec<MatchResult> = vec![MatchResult::Won, MatchResult::Lost, MatchResult::Won, MatchResult::Won];
        let outsiders_matches: Vec<MatchResult> = vec![MatchResult::Won, MatchResult::Lost, MatchResult::Won, MatchResult::Won];
        let complexity_matches: Vec<MatchResult> = vec![MatchResult::Lost, MatchResult::Won, MatchResult::Lost, MatchResult::Lost];
        let ihc_matches: Vec<MatchResult> = vec![MatchResult::Lost, MatchResult::Lost, MatchResult::Won, MatchResult::Lost];
        let renegades_matches: Vec<MatchResult> = vec![MatchResult::Lost, MatchResult::Lost, MatchResult::Lost];
        let liquid_matches: Vec<MatchResult> = vec![MatchResult::Lost, MatchResult::Lost, MatchResult::Won, MatchResult::Won];
        let ninez_matches: Vec<MatchResult> = vec![MatchResult::Lost, MatchResult::Lost, MatchResult::Lost];

        if PLAYER == 0 {
            return [
                Team::new_with_matches("ENCE", 9, PickEmBet::Advance, ence_matches),
                Team::new_with_matches("G2", 10, PickEmBet::ThreeZero, g2_matches),
                Team::new_with_matches("Forze", 11, PickEmBet::Nothing, forze_matches),
                Team::new_with_matches("Astralis", 12, PickEmBet::Advance, astralis_matches),
                Team::new_with_matches("Vitality", 13, PickEmBet::Advance, vitality_matches),
                Team::new_with_matches("mibr", 14, PickEmBet::Advance, mibr_matches),
                Team::new_with_matches("Imperial", 15, PickEmBet::Nothing, imperial_matches),
                Team::new_with_matches("Bad News Eagles", 16, PickEmBet::Advance, bne_matches),
                Team::new_with_matches("Eternal Fire", 17, PickEmBet::Nothing, eternal_fire_matches),
                Team::new_with_matches("Team Spirit", 18, PickEmBet::Nothing, spirit_matches),
                Team::new_with_matches("Outsiders", 19, PickEmBet::Advance, outsiders_matches),
                Team::new_with_matches("Complexity", 20, PickEmBet::Nothing, complexity_matches),
                Team::new_with_matches("IHC", 21, PickEmBet::Nothing, ihc_matches),
                Team::new_with_matches("Renegades", 22, PickEmBet::Nothing, renegades_matches),
                Team::new_with_matches("Team Liquid", 23, PickEmBet::Advance, liquid_matches),
                Team::new_with_matches("9z", 24, PickEmBet::ZeroThree, ninez_matches),
            ]
        } else {
            return [
                Team::new_with_matches("ENCE", 9, PickEmBet::Advance, ence_matches),
                Team::new_with_matches("G2", 10, PickEmBet::ThreeZero, g2_matches),
                Team::new_with_matches("Forze", 11, PickEmBet::Nothing, forze_matches),
                Team::new_with_matches("Astralis", 12, PickEmBet::Advance, astralis_matches),
                Team::new_with_matches("Vitality", 13, PickEmBet::Advance, vitality_matches),
                Team::new_with_matches("mibr", 14, PickEmBet::Advance, mibr_matches),
                Team::new_with_matches("Imperial", 15, PickEmBet::Advance, imperial_matches),
                Team::new_with_matches("Bad News Eagles", 16, PickEmBet::Nothing, bne_matches),
                Team::new_with_matches("Eternal Fire", 17, PickEmBet::Nothing, eternal_fire_matches),
                Team::new_with_matches("Team Spirit", 18, PickEmBet::ZeroThree, spirit_matches),
                Team::new_with_matches("Outsiders", 19, PickEmBet::Advance, outsiders_matches),
                Team::new_with_matches("Complexity", 20, PickEmBet::Nothing, complexity_matches),
                Team::new_with_matches("IHC", 21, PickEmBet::Nothing, ihc_matches),
                Team::new_with_matches("Renegades", 22, PickEmBet::Nothing, renegades_matches),
                Team::new_with_matches("Team Liquid", 23, PickEmBet::Advance, liquid_matches),
                Team::new_with_matches("9z", 24, PickEmBet::Nothing, ninez_matches),
            ]
        }

        
    }
    */
    fn wins(&self, mut round: usize) -> usize {
        if round > self.results.len() {
            round = self.results.len();
        }

        let mut win_count = 0;
        for result in &self.results[0..round] {
            if matches!(result, MatchResult::Won) {
                win_count += 1;
            }
        }
        win_count
    }

    fn cmp(&self, other: &Self) -> Ordering {
        if self.buchholz > other.buchholz {
            return Ordering::Greater;
        } else if self.buchholz < other.buchholz {
            return Ordering::Less;
        }

        if self.seeding < other.seeding {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }

    fn already_played(&self, enemy: usize) -> bool {
        self.enemies.contains(&enemy)
    }

    fn check_pickem(&self, player: usize) -> bool {
        if self.enemies.len() == ADVANCE {
            if matches!(self.results[0], MatchResult::Won) && (matches!(self.pickems[player], PickEmBet::ThreeZero) || matches!(self.pickems[player], PickEmBet::Advance)) {
                return true;
            } else if matches!(self.results[0], MatchResult::Lost) && matches!(self.pickems[player], PickEmBet::ZeroThree) {
                return true;
            } else {
                return false;
            }
        }

        let wins = self.wins(self.results.len());
        let losses = self.results.len() - wins;
        //println!("Wins / Losses: {} {}", wins, losses);

        if wins > losses && matches!(self.pickems[player], PickEmBet::Advance) {
            return true;
        } 

        false
    }
}


fn main() {

    if !OUTPUT {
    
        let mut thread_vector: Vec<thread::JoinHandle<Vec<f64>>> = Vec::new();
    
        for _i in 0..24 {
            thread_vector.push(thread::spawn(move || thread_simulation()));
        }
    
        let mut result_alex: f64 = 0.0;
        let mut result_an: f64 = 0.0;
        let mut result_fe: f64 = 0.0;

        let mut result_alex_an: f64 = 0.0;
        let mut result_alex_fe: f64 = 0.0;
        let mut result_an_fe: f64 = 0.0;
        let mut result_all: f64 = 0.0;

        while thread_vector.len() > 0 {
            let cur_thread = thread_vector.remove(0);
            let results = cur_thread.join().unwrap();

            result_alex += results[0];
            result_an += results[1];
            result_fe += results[2];

            result_alex_an += results[3];
            result_alex_fe += results[4];
            result_an_fe += results[5];
            result_all += results[6];

            //result += cur_thread.join().unwrap();
        }
    
        result_alex = result_alex / 24.0;
        result_an = result_an / 24.0;
        result_fe = result_fe / 24.0;
        result_alex_an = result_alex_an / 24.0;
        result_alex_fe = result_alex_fe / 24.0;
        result_an_fe = result_an_fe / 24.0;
        result_all = result_all / 24.0;

        println!("Alex won the pickem   {:.2}% of times\t({})", result_alex * 100.0, result_alex);
        println!("An won the pickem {:.2}% of times\t({})", result_an * 100.0, result_an);
        println!("Fe won the pickem  {:.2}% of times\t({})", result_fe * 100.0, result_fe);
        println!("");
        println!("Alex + An won the pickem   {:.2}% of times\t({})", result_alex_an * 100.0, result_alex_an);
        println!("Alex + Fe won the pickem    {:.2}% of times\t({})", result_alex_fe * 100.0, result_alex_fe);
        println!("An + Fe won the pickem  {:.2}% of times\t({})", result_an_fe * 100.0, result_an_fe);
        println!("");
        println!("Everyone won the pickem  {:.2}% of times\t({})", result_all * 100.0, result_all);

    } else {
        run_one_simulation();
    }


}

fn thread_simulation() -> Vec<f64> {
    const TIMES_RUN: u64 = 1_000_000;
    let mut times_pickem: Vec<u64> = vec![0, 0, 0, 0, 0, 0, 0];

    for _x in 0..TIMES_RUN {
        let pickem = run_one_simulation();
        for n in 0..3 {
            if pickem[n] {times_pickem[n] += 1;}
        }
        if pickem[0] && pickem[1] {times_pickem[3] += 1;};
        if pickem[0] && pickem[2] {times_pickem[4] += 1;};
        if pickem[1] && pickem[2] {times_pickem[5] += 1;};
        if pickem[0] && pickem[1] && pickem[2] {times_pickem[6] += 1;};

    }
    let mut times_pickem_ratio: Vec<f64> = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];


    for n in 0..7 {
        times_pickem_ratio[n] = times_pickem[n] as f64 / TIMES_RUN as f64
    }

    times_pickem_ratio
}


fn run_one_simulation() -> Vec<bool> {
    let mut teams = Team::get_all_legends_team();
    //let mut teams = Team::get_all_legends_team_someone();

    for i in 0..5 {
        //Check if teams are still active
        for team_index in 0..TEAM_AMOUNT {
            let mut team = &mut teams[team_index];
            if team.active {
                let wins = team.wins(i);
                if wins == 3 || (i - wins) == 3 {
                    team.active = false;
                }
            }
        }

        //Recalculate Buchholz
        for team_index in 0..TEAM_AMOUNT {
            if teams[team_index].active {
                
                teams[team_index].buchholz = 0;

                for enemy_index in 0..teams[team_index].enemies.len() {

                    let enemy = teams[team_index].enemies[enemy_index];
                    
                    let wins = teams[enemy].wins(i) as i32;
                    let buchholz_change = 2 * wins - (teams[enemy].enemies.len() as i32);

                    //println!("Change: {}: {}", teams[enemy].name, buchholz_change);

                    teams[team_index].buchholz += buchholz_change;
                }

                //println!("Buchholz {}: {}", teams[team_index].name, teams[team_index].buchholz);


            }
        }

        if OUTPUT {
            println!("ROUND {} -------------------", i);
        }

        seed_teams(&mut teams, i);
        
        // Do matches:
        for team_index in 0..TEAM_AMOUNT {
            if teams[team_index].active {
                let enemy_index = teams[team_index].enemies[i];



                if !(teams[team_index].results.len() > i) {

                    let result: f32 = rand::thread_rng().gen::<f32>();
                    if result < 0.5 {
                        teams[team_index].results.push(MatchResult::Lost);
                        teams[enemy_index].results.push(MatchResult::Won);
                        //println!("{} won randomly against {}", teams[team_index].name, teams[enemy_index].name);
                    } else {
                        teams[team_index].results.push(MatchResult::Won);
                        teams[enemy_index].results.push(MatchResult::Lost);
                        //println!("{} lost randomly against {}", teams[team_index].name, teams[enemy_index].name);
                    }

                }
                //println!("{} already played against {}", teams[team_index].name, teams[enemy_index].name);


            }
        }

    }

    let mut pick_em_results: Vec<bool> = vec![false, false, false];

    for n in 0..3 {
        let mut pickems = 0;

        for team_index in 0..TEAM_AMOUNT {
            let team = &mut teams[team_index];
            if team.check_pickem(n) {
                pickems += 1;
                //println!("Correct Pickem: {}", team.name);
            } else {
                //println!("Incorrect Pickem: {}", team.name);
            }
        }

        if pickems > 4 {
            pick_em_results[n] = true;
        }
    }

    //println!("Correct Pickems: {}", pickems);
    pick_em_results
}

fn seed_teams(teams: &mut [Team; TEAM_AMOUNT], round: usize) {

    let mut baskets: [Vec<usize>; 5] = Default::default();

    for team_index in 0..TEAM_AMOUNT {
        let team = &teams[team_index];

        if team.active {
            let index = team.wins(round);
            baskets[index].push(team_index);
        }
    }


    for i in 0..ROUNDS {
        let mut basket_copy: Vec<usize> = Vec::new();
        for x in 0..baskets[i].len() {
            basket_copy.push(baskets[i][x])
        }

        basket_copy.sort_by(|b, a| teams[*a].cmp(&teams[*b])); // Sort array in reverse order

        let mut basket_names: Vec<&str> = Vec::new();
        for y in 0..basket_copy.len() {
            basket_names.push(&teams[basket_copy[y]].name);
        }

        //println!("Basket {}: {:?}", i, &basket_names);

        let successful = find_matches(&basket_copy, teams);
        if !successful {
            set_default_matches(&basket_copy, teams);
        }
        

    }
}


fn find_matches(basket: &Vec<usize>, teams: &mut [Team; TEAM_AMOUNT]) -> bool {

    if basket.len() > 0 {

        let current_team = basket[0];
        let mut enemy_index = basket.len() - 1;
    
        loop {
            let enemy_team = basket[enemy_index];
    
            if !teams[current_team].already_played(enemy_team) {
    
                let mut basket_copy: Vec<usize> = Vec::new();
                for x in 0..basket.len() {
                    basket_copy.push(basket[x])
                }
    
                basket_copy.remove(0);
                basket_copy.remove(enemy_index - 1);
    
                let successful = find_matches(&basket_copy, teams);
    
                if successful {
    
                    teams[current_team].enemies.push(enemy_team);
                    teams[enemy_team].enemies.push(current_team);
                    
                    if OUTPUT {
                        println!("Playing against each other (Advanced): {} - {}", teams[current_team].name, teams[enemy_team].name);
                    }
    
                    break;
                }
            }
    
            enemy_index -= 1;
            if enemy_index == 0 {
                return false;
            }
    
        }
    }

    true
}


fn set_default_matches(basket: &Vec<usize>, teams: &mut [Team; TEAM_AMOUNT]) {

    let mut basket_copy: Vec<usize> = Vec::new();
    for x in 0..basket.len() {
        basket_copy.push(basket[x])
    }

    while basket.len() > 0 {
        let current_team = basket_copy[0];
        let mut enemy_team = basket_copy[1];
        let mut enemy_index = 1;

        for other_team in 2..basket_copy.len() {
            if !teams[current_team].already_played(basket_copy[other_team]) {
                enemy_team = basket_copy[other_team];
                enemy_index = other_team;
            }
        }

        basket_copy.remove(0);
        basket_copy.remove(enemy_index - 1); // Minus 1, because we remove the 0 before and it pushes everything to the left

        teams[current_team].enemies.push(enemy_team);
        teams[enemy_team].enemies.push(current_team);

        if OUTPUT {
            println!("Playing against each other (Default): {} - {}", teams[current_team].name, teams[enemy_team].name);
        }

    }

}

