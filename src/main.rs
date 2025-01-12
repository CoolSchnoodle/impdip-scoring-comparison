#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(derive_try_from_primitive::TryFromPrimitive)]
#[repr(u8)]
enum Nation {
    Portugal,
    Spain,
    Netherlands,
    England,
    France,
    Ottoman,
    Russia,
    Poland,
    Inuit,
    Ming,
    Mughal,
    Qing,
    Safavid,
    UteShoshone,
    Abyssinia,
    Ajuuraan,
    Athapasca,
    Austria,
    Aymara,
    Ayutthaya,
    Kongo,
    Mali,
    Mapuche,
    Sweden,
    Tokugawa,
}

impl<'a> TryFrom<&'a str> for Nation {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s.to_ascii_lowercase().as_str() {
            "portugal" | "por" => Ok(Nation::Portugal),
            "spain" => Ok(Nation::Spain),
            "netherlands" | "dutch" => Ok(Nation::Netherlands),
            "england" | "eng" => Ok(Nation::England),
            "france" => Ok(Nation::France),
            "ottoman" | "ottomans" => Ok(Nation::Ottoman),
            "russia" => Ok(Nation::Russia),
            "poland" | "poland-lithuania" => Ok(Nation::Poland),
            "inuit" => Ok(Nation::Inuit),
            "ming" => Ok(Nation::Ming),
            "mughal" => Ok(Nation::Mughal),
            "qing" => Ok(Nation::Qing),
            "safavid" => Ok(Nation::Safavid),
            "ute-shoshone" | "ute" | "shoshone" => Ok(Nation::UteShoshone),
            "abyssinia" | "aby" => Ok(Nation::Abyssinia),
            "ajuuraan" | "aju" => Ok(Nation::Ajuuraan),
            "athapasca" | "atha" => Ok(Nation::Athapasca),
            "austria" => Ok(Nation::Austria),
            "aymara" => Ok(Nation::Aymara),
            "ayutthaya" | "ayu" => Ok(Nation::Ayutthaya),
            "kongo" => Ok(Nation::Kongo),
            "mali" => Ok(Nation::Mali),
            "mapuche" => Ok(Nation::Mapuche),
            "sweden" => Ok(Nation::Sweden),
            "tokugawa" | "toku" => Ok(Nation::Tokugawa),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Nation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Nation::Portugal => "portugal".to_string(),
            Nation::Spain => "spain".to_string(),
            Nation::Netherlands => "netherlands".to_string(),
            Nation::England => "england".to_string(),
            Nation::France => "france".to_string(),
            Nation::Ottoman => "ottoman".to_string(),
            Nation::Russia => "russia".to_string(),
            Nation::Poland => "poland".to_string(),
            Nation::Inuit => "inuit".to_string(),
            Nation::Ming => "ming".to_string(),
            Nation::Mughal => "mughal".to_string(),
            Nation::Qing => "qing".to_string(),
            Nation::Safavid => "safavid".to_string(),
            Nation::UteShoshone => "ute-shoshone".to_string(),
            Nation::Abyssinia => "abyssinia".to_string(),
            Nation::Ajuuraan => "ajuuraan".to_string(),
            Nation::Athapasca => "athapasca".to_string(),
            Nation::Austria => "austria".to_string(),
            Nation::Aymara => "ayutthaya".to_string(),
            Nation::Ayutthaya => "kongo".to_string(),
            Nation::Kongo => "kongo".to_string(),
            Nation::Mali => "mali".to_string(),
            Nation::Mapuche => "mapuche".to_string(),
            Nation::Sweden => "sweden".to_string(),
            Nation::Tokugawa => "tokugawa".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl Nation {
    #[inline]
    fn starting_sc_count(&self) -> usize {
        match self {
            Nation::Portugal => 16,
            Nation::Spain => 16,
            Nation::Netherlands => 14,
            Nation::England => 14,
            Nation::France => 14,
            Nation::Ottoman => 10,
            Nation::Russia => 10,
            Nation::Poland => 7,
            Nation::Inuit => 5,
            Nation::Ming => 5,
            Nation::Mughal => 5,
            Nation::Qing => 5,
            Nation::Safavid => 5,
            Nation::UteShoshone => 5,
            Nation::Abyssinia => 4,
            Nation::Ajuuraan => 4,
            Nation::Athapasca => 4,
            Nation::Austria => 4,
            Nation::Aymara => 4,
            Nation::Ayutthaya => 4,
            Nation::Kongo => 4,
            Nation::Mali => 4,
            Nation::Mapuche => 4,
            Nation::Sweden => 4,
            Nation::Tokugawa => 4,
        }
    }

    fn vscc_percent(&self, sc_count: usize) -> f32 {
        if sc_count < self.starting_sc_count() {
            -(1.0 - (sc_count as f32 / self.starting_sc_count() as f32))
        } else {
            (sc_count - self.starting_sc_count()) as f32
                / (starting_scs_to_vscc(self.starting_sc_count()) - self.starting_sc_count()) as f32
        }
    }
}

#[inline]
fn starting_scs_to_vscc(starting_scs: usize) -> usize {
    match starting_scs {
        4 => 32,
        5 => 36,
        7 => 42,
        10 => 48,
        14 => 56,
        16 => 64,
        _ => panic!("Invalid starting scs"),
    }
}

trait Score {
    fn score(&self, vsccs: [f32; 25]) -> [f32; 25];
}

struct Current;
impl Score for Current {
    fn score(&self, vsccs: [f32; 25]) -> [f32; 25] {
        let sorted_vsccs = {
            let mut v = vsccs.clone();
            v.sort_by(|a, b| b.partial_cmp(a).unwrap());
            v
        };
        let mut prev_vscc: f32 = sorted_vsccs[0];
        let impunity_size = sorted_vsccs.into_iter().take_while(|v| {
            let impunities = (prev_vscc - v) <= 0.25 && *v >= 1.0;
            prev_vscc = *v;
            impunities
        }).count();
        let highest_non_impunity_vscc = prev_vscc;
        let scores = vsccs.into_iter().map(|v|
            100.0 * v.max(0.0)
                + if v > highest_non_impunity_vscc { 500.0 / impunity_size as f32 } else { 0.0 }
                + if v > -1.0 { 15.0 } else { 0.0 }
        ).collect::<Vec<f32>>();
        scores.try_into().unwrap()
    }
}

struct Proposed<VsccMap: Fn(f32) -> f32>(VsccMap);
impl<F: Fn(f32) -> f32> Score for Proposed<F> {
    fn score(&self, vsccs: [f32; 25]) -> [f32; 25] {
        let sorted_vsccs = {
            let mut v = vsccs.clone();
            v.sort_by(|a, b| b.partial_cmp(a).unwrap());
            v
        };
        let mut prev_vscc: f32 = sorted_vsccs[0];
        let impunity_size = sorted_vsccs.iter().take_while(|&v| {
            let impunities = (prev_vscc - v) <= 0.25;
            prev_vscc = *v;
            impunities
        }).count();
        let highest_non_impunity_vscc = prev_vscc;
        let total_avscc: f32 = vsccs.into_iter().map(|v| if v > 0.0 { (&self.0)(v) } else { 0.0 }).sum();
        let scores = vsccs.into_iter().map(|v|
            if v > 0.0 { 1000.0 * (&self.0)(v) / total_avscc } else { 0.0 }
                + if v > highest_non_impunity_vscc { 300.0 / impunity_size as f32 } else { 0.0 }
                + if v > -1.0 { 15.0 } else { 0.0 }
        ).collect::<Vec<f32>>();
        scores.try_into().unwrap()
    }
}

fn scores_to_proportions(scores: [f32; 25]) -> [f32; 25] {
    let score_sum: f32 = scores.into_iter().sum();
    scores.into_iter().map(|s| s / score_sum).collect::<Vec<f32>>().try_into().unwrap()
}

const RATING_POOL_PER_GAME: f32 = 2625.0;

fn main() {
    let a2_data: Vec<[usize; 25]> = include_str!("../a2.txt")
        .split('\n')
        .map(|s| s.split('\t')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into().unwrap()
        ).collect();
    let current = Current;
    let proposed_1_5 = Proposed(|v| (v * 100.0).powf(1.5));
    let proposed_2_0 = Proposed(|v| (v * 100.0).powi(2));
    a2_data.into_iter().enumerate().for_each(|(i, sc_counts)| {
        let vsccs = sc_counts.into_iter()
            .enumerate()
            .map(|(i, s)| {
                let n: Nation = (i as u8).try_into().unwrap();
                n.vscc_percent(s)
            }).collect::<Vec<_>>().try_into().unwrap();
        println!("\nA2{}:", (b'A' + i as u8) as char);
        let current_rating_change = scores_to_proportions(current.score(vsccs))
            .into_iter()
            .map(|p| p * RATING_POOL_PER_GAME - 105.0);
        let proposed_1_5_rating_change = scores_to_proportions(proposed_1_5.score(vsccs))
            .into_iter()
            .map(|p| p * RATING_POOL_PER_GAME - 105.0);
        let proposed_2_0_rating_change = scores_to_proportions(proposed_2_0.score(vsccs))
            .into_iter()
            .map(|p| p * RATING_POOL_PER_GAME - 105.0);
        itertools::izip!(current_rating_change, proposed_1_5_rating_change, proposed_2_0_rating_change)
            .enumerate()
            .for_each(|(i, (c, p_1_5, p_2_0))| {
                println!("{:<16}{:.0}\t{:.0}\t{:.0}",
                         Nation::try_from(i as u8).unwrap().to_string(),
                         c,
                         p_1_5,
                         p_2_0);
            })
    })
}
