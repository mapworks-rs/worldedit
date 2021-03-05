use std::collections::HashMap;
use std::convert::TryFrom;
use std::num::ParseIntError;
use quill::BlockKind;

/// Represents a fill/paste pattern. This just contains a set of blocks mapped to a weight in which
/// they should be applied. **WEIGHTS ARE NOT PERCENTAGES**. Having a pattern with 70%glass,50%dirt
/// is completely valid. The values are compressed or expanded into a fraction of 100. For example:
/// 150%glass,50%dirt will become a pattern where there is a 75% (3/4) chance a glass block is chosen
/// and a 25% (1/4) chance a dirt block is chosen. The same would apply if I reduced each by a factor
/// of 10: 15%glass,5%dirt (would still produce 3/4 glass and 1/4 dirt)
#[derive(Debug)]
pub struct Pattern {
    weights: HashMap<BlockKind, u16>
}

impl Pattern {
    pub fn new() -> Pattern {
        let mut map = HashMap::new();
        Pattern {
            weights: map
        }
    }

    /// Will attempt to convert a BlockKind into a pattern if it can be converted. Accepted BlockKinds will
    /// follow the type: `n%block,....` etc. For example: `50%glass,10%dirt,60%stone`
    pub fn from_string(string: String) -> Option<Pattern> {

        let mut pattern = Pattern::new();

        // = "50%stone,10%dirt,15%glass"
        let elements = string.split(',');
        // = "50%stone" "10%dirt" "15%glass"
        for s in elements {
            // = "50" "stone"
            let components = s.split('%').collect::<Vec<&str>>();
            if components.len() != 2 {
                continue;
            }
            let weight_result = components.get(0).unwrap().parse::<u16>();
            if weight_result.is_err() {
                continue;
            }
            let block_kind_option = BlockKind::from_name(components.get(1).unwrap());
            if block_kind_option.is_none() {
                continue;
            }
            let weight = weight_result.unwrap();
            let block_kind = block_kind_option.unwrap();

            pattern.add(block_kind, weight);
        }

        match pattern.weights.is_empty() {
            true => None,
            false => Some(pattern)
        }
    }

    pub fn new_with(elements: &[(BlockKind, u16)]) -> Pattern {
        let mut pat = Pattern::new();
        pat.add_multiple(elements);
        pat
    }

    pub fn add(&mut self, block_kind: BlockKind, weight: u16) {
        self.add_multiple(&[(block_kind, weight)]);
    }

    pub fn add_multiple(&mut self, elements: &[(BlockKind, u16)]) {
        for (bk, u) in elements {
            self.weights.insert(bk.clone(), *u);
        }
    }

    pub fn to_percentages(&self) -> HashMap<BlockKind, u8> {
        let k: f32 = 100.0 / (self.weights.values().sum::<u16>() as f32);
        self.weights.iter().map(|t| (t.0.clone(), (*t.1 as f32 * k).round() as u8)).collect()
    }
}