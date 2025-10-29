//! Quiz state management for flashcard practice

/// State of the current quiz
#[derive(Debug, Clone, PartialEq)]
pub enum QuizState {
    Question,
    AnswerCorrect,
    AnswerIncorrect { selected: usize, correct: usize },
}

impl QuizState {
    /// Check if an answer has been submitted
    pub fn is_answered(&self) -> bool {
        !matches!(self, QuizState::Question)
    }

    /// Check if the answer was correct
    pub fn is_correct(&self) -> bool {
        matches!(self, QuizState::AnswerCorrect)
    }
}
