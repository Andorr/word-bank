import { QuizMode, QuizOptions, QuizQuestionPolicy, QuizResult, QuizWordResults, Word } from "@/lib/models";
import { alertController } from "@ionic/vue";

export enum QuizStatus {
    NotStarted,
    Ongoing,
    Finished,
}

export enum QuizQuestionResult {
    None = "None",
    Correct = "Correct",
    Incorrect = "Incorrect",
}
  
export type Question = {
    wordId: string;
    question: string;
    answers: string[];
    classType: string;
    numIncorrects: number;
    numCorrects: number;
};

export type QuizStats = {
    numCorrects: number;
    numIncorrects: number;
    numQuestionWins: number;
    numQuestionLosses: number;
    scorePercentage: number;
    numQuestionsAnswered: number;
    totalNumQuestions: number;

}

export class QuizState {
    id: string;
    questions: Question[];
    status: QuizStatus;
    questionIndex: number;
    options: QuizOptions;

    stats: QuizStats

    constructor(id: string, questions: Word[], options: QuizOptions) {
        this.id = id;
        this.questions = this.buildQuestions(questions, options.policy);
        this.status = QuizStatus.NotStarted;
        this.questionIndex = 0;
        this.options = options;
        const numQuestionWins = this.questions.filter(q => q.numCorrects >= q.numIncorrects).length;
        const numQuestionLosses = this.questions.filter(q => q.numCorrects < q.numIncorrects).length;
        this.stats = {
            numCorrects: 0,
            numIncorrects: 0,
            numQuestionWins,
            numQuestionLosses,
            scorePercentage: 0,
            numQuestionsAnswered: 0,
            totalNumQuestions: this.questions.length,
        };
    }

    startQuiz(): Question {
        this.status = QuizStatus.Ongoing;
        return this.questions[0];
    }

    endQuiz(): QuizStatus {
        this.status = QuizStatus.Finished;
        return this.status;
    }

    nextQuestion(): Question | null {
        if(this.status !== QuizStatus.Ongoing) {
            return null;
        }

        this.stats.numQuestionsAnswered++;

        if(this.questionIndex + 1 === this.questions.length) {
            if(this.options.mode === QuizMode.Normal) {
                this.status = QuizStatus.Finished;
                return null;
            }
        }
        
        this.questionIndex = (this.questionIndex + 1)%this.questions.length;
        return this.questions[this.questionIndex];
    }

    checkAnswer(input: string): QuizQuestionResult {
        if(this.questionIndex >= this.questions.length) {
            return QuizQuestionResult.None;
        }

        const numCorrect = this.questions[this.questionIndex].answers.filter(answer => answer.trim().toLowerCase() === input.trim().toLowerCase()).length;
        if(numCorrect > 0) {
          this.questions[this.questionIndex].numCorrects++;
          return QuizQuestionResult.Correct;
        } else {
          this.questions[this.questionIndex].numIncorrects++;
          return QuizQuestionResult.Incorrect;
        }
    }

    passQuestion(): QuizQuestionResult {
        this.questions[this.questionIndex].numIncorrects++;
        return QuizQuestionResult.Incorrect;
    }

    getStats(): QuizStats {
        const numQuestionWins = this.questions.filter(q => q.numCorrects >= q.numIncorrects).length;
        const numQuestionLosses = this.questions.filter(q => q.numCorrects < q.numIncorrects).length;
        const numQuestionsAnswered = this.questions.filter(q => q.numCorrects > 0 || q.numIncorrects > 0).length;

        this.stats.numCorrects = this.numCorrect;
        this.stats.numIncorrects = this.numIncorrect;
        this.stats.scorePercentage = (numQuestionWins)/((numQuestionsAnswered) || 1);
        this.stats.numQuestionWins = numQuestionWins;
        this.stats.numQuestionLosses = numQuestionLosses;
        return this.stats;
    }

    get numCorrect(): number {
        return this.questions.reduce((acc, question) => question.numCorrects + acc, 0);
    }

    get numIncorrect(): number {
        return this.questions.reduce((acc, question) => question.numIncorrects + acc, 0);
    }

    toQuizResult(): QuizResult {
        return {
            id: this.id,
            results: this.questions.filter(q => q.numCorrects > 0 || q.numIncorrects > 0).map(q => ({ wordId: q.wordId, numCorrects: q.numCorrects, numIncorrects: q.numIncorrects }) as QuizWordResults ), 
            createdAt: new Date().toISOString(),         
        }
    }

    private buildQuestions(words: Word[], policy?: QuizQuestionPolicy): Question[] {
        return words.map((w) => {
            const question: Question = { question: "", answers: [], numCorrects: 0, numIncorrects: 0, classType: w.classType, wordId: w.id };
            switch (
            policy ||
            QuizQuestionPolicy.WordToTranslations
            ) {
                case QuizQuestionPolicy.WordToTranslations: {
                    question.question = w.value;
                    question.answers = w.translations.map((t) => t.value);
                    break;
                }
                case QuizQuestionPolicy.TranslationsToWord: {
                    question.question = w.translations[0].value;
                    question.answers = [w.value];
                    break;
                }
                case QuizQuestionPolicy.Random: {
                    if (Math.random() >= 0.5) {
                        question.question = w.value;
                        question.answers = w.translations.map((t) => t.value);
                    } else {
                        question.question = w.translations[0].value;
                        question.answers = [w.value];
                    }
                    break;
                }
            }
            return question;
        });
    }
}

export const openEndQuizAlert = async (callback: (success: boolean) => void) => {
    const alert = await alertController.create({
        header: `Are you sure you want to end the quiz?`,
        message: ``,
        buttons: [
          "Cancel",
          {
            text: "Yes",
            role: "yes",
          },
        ],
      });
      await alert.present();

      const { role } = await alert.onDidDismiss();
      callback(role === "yes")
}