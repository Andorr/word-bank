import { QuizMode, QuizOptions, QuizQuestionPolicy, Word } from "@/lib/models";
import { alertController } from "@ionic/vue";

export enum QuizStatus {
    NotStarted,
    Ongoing,
    Finished,
}

export enum QuizResult {
    None = "None",
    Correct = "Correct",
    Incorrect = "Incorrect",
}
  
export type Question = {
    question: string;
    answers: string[];
    kind: string;
    numIncorrects: number;
    numCorrects: number;
};

export type QuizStats = {
    numCorrects: number;
    numIncorrects: number;
    scorePercentage: number;
    numQuestionsAnswered: number;
    totalNumQuestions: number;
}

export class QuizState {
    questions: Question[];
    status: QuizStatus;
    questionIndex: number;
    options: QuizOptions;

    stats: QuizStats

    constructor(questions: Word[], options: QuizOptions) {
        this.questions = this.buildQuestions(questions, options.policy);
        this.status = QuizStatus.NotStarted;
        this.questionIndex = 0;
        this.options = options;
        this.stats = {
            numCorrects: 0,
            numIncorrects: 0,
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

    checkAnswer(input: string): QuizResult {
        if(this.questionIndex >= this.questions.length) {
            return QuizResult.None;
        }

        const numCorrect = this.questions[this.questionIndex].answers.filter(answer => answer.trim().toLowerCase() === input.trim().toLowerCase()).length;
        if(numCorrect > 0) {
          this.questions[this.questionIndex].numCorrects++;
          return QuizResult.Correct;
        } else {
          this.questions[this.questionIndex].numIncorrects++;
          return QuizResult.Incorrect;
        }
    }

    passQuestion(): QuizResult {
        this.questions[this.questionIndex].numIncorrects++;
        return QuizResult.Incorrect;
    }

    getStats(): QuizStats {
        const corrects = this.numCorrect;
        const incorrects = this.numIncorrect;
        this.stats.numCorrects = this.numCorrect;
        this.stats.numIncorrects = this.numIncorrect;
        this.stats.scorePercentage = (corrects)/((corrects + incorrects) || 1);
        return this.stats;
    }

    get numCorrect(): number {
        return this.questions.reduce((acc, question) => question.numCorrects + acc, 0);
    }

    get numIncorrect(): number {
        return this.questions.reduce((acc, question) => question.numIncorrects + acc, 0);
    }

    private buildQuestions(words: Word[], policy?: QuizQuestionPolicy): Question[] {
        return words.map((w) => {
            const question: Question = { question: "", answers: [], numCorrects: 0, numIncorrects: 0, kind: w.kind };
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
