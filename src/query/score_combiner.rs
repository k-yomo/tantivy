use Score;
use query::Scorer;


/// The `ScoreCombiner` trait defines how to compute
/// an overall score given a list of scores.
pub trait ScoreCombiner: Default + Clone + Copy {
    fn update<TScorer: Scorer>(&mut self, scorer: &mut TScorer);
    fn clear(&mut self);
    fn score(&self) -> Score;
}

/// Just ignores scores. The `DoNothingCombiner` does not
/// even call the scorers `.score()` function.
///
/// It is useful to optimize the case when scoring is disabled.
///
#[derive(Default, Clone, Copy)] //< these should not be too much work :)
pub struct DoNothingCombiner;

impl ScoreCombiner for DoNothingCombiner {
    fn update<TScorer: Scorer>(&mut self, _scorer: &mut TScorer) {}

    fn clear(&mut self) {}

    fn score(&self) -> Score {
        1f32
    }
}


/// Sums the score of different scorers.
#[derive(Default, Clone, Copy)]
pub struct SumCombiner {
    score: Score
}


impl ScoreCombiner for SumCombiner {
    fn update<TScorer: Scorer>(&mut self, scorer: &mut TScorer) {
        self.score += scorer.score();
    }

    fn clear(&mut self) {
        self.score = 0f32;
    }

    fn score(&self) -> Score {
        self.score
    }
}


/// Sums the score of different scorers and keeps the count
/// of scorers which matched.
#[derive(Default, Clone, Copy)]
pub struct SumWithCoordsCombiner {
    num_fields: usize,
    score: Score,
}

impl ScoreCombiner for SumWithCoordsCombiner {
    fn update<TScorer: Scorer>(&mut self, scorer: &mut TScorer) {
        self.score += scorer.score();
        self.num_fields += 1;
    }

    fn clear(&mut self) {
        self.score = 0f32;
        self.num_fields = 0;
    }

    fn score(&self) -> Score {
        self.score
    }
}
