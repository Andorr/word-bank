<template>
  <page :header="false">
    <div class="flex flex-col h-full" v-if="showQuizView">
      <div
        class="p-safe-area-top bg-primary"
        :class="{ correct: isResultCorrect, incorrect: isResultIncorrect }"
      />
      <div
        class="bg-primary h-64 w-full relative"
        :class="{ correct: isResultCorrect, incorrect: isResultIncorrect }"
      >
        <div
          class="
            absolute
            top-0
            left-0
            w-full
            px-2
            flex
            items-center
            justify-end
            z-10
          "
        >
          <icon-btn @click="tryEndQuiz">
            <ion-icon class="text-2xl text-white" :icon="icons.close2" />
          </icon-btn>
        </div>
        <div
          class="
            w-full
            flex
            items-center
            justify-center
            flex-col
            absolute
            inset-0
          "
        >
          <ion-text>
            <h1 v-if="currentQuestion" class="text-5xl text-center">
              {{ currentQuestion.question }}
            </h1>
          </ion-text>
          <ion-text>
            <h1 v-if="currentQuestion" class="text-xs text-center">
              {{ currentQuestion.classType }}
            </h1>
          </ion-text>
          <ion-text v-if="revealAnswer">
            <h1 v-if="currentQuestion" class="text-2xl text-center">
              {{ currentQuestion.answers[0] }}
            </h1>
          </ion-text>
        </div>
      </div>
      <ion-progress-bar
        :value="stats.numQuestionsAnswered / (stats.totalNumQuestions || 1)"
        color="success"
      />
      <div class="flex flex-col flex-grow">
        <div class="flex justify-center m-4">
          <div class="flex items-center text-red-400 text-3xl flex-grow">
            <ion-icon class="mb-1 mr-2" :icon="icons.close" />
            <span>{{ stats.numIncorrects }}</span>
          </div>
          <btn
            class="mx-auto rounded-full shadow-none bg-medium"
            :disabled="!isResultNone || isQuizFinished"
            @click="passQuestion"
            >Pass</btn
          >
          <div
            class="
              flex
              items-center
              text-green-400 text-3xl
              justify-end
              flex-grow
            "
          >
            <span>{{ stats.numCorrects }}</span>
            <ion-icon class="mb-1 ml-2" :icon="icons.checkmark" />
          </div>
        </div>
        <div class="flex-grow" />
        <div class="px-5 pt-2 mb-5 p-safe-area-bottom">
          <div class="bg-color-light rounded px-2 mb-3">
            <ion-input
              ref="inputElement"
              v-model="input"
              placeholder="Answer"
            />
          </div>
          <btn
            class="w-full"
            :disabled="input.length === 0 || !isResultNone"
            @click="checkAnswer"
            >Check answer</btn
          >
        </div>
      </div>
    </div>
    <QuizResultView v-if="isQuizFinished" :quizState="quizState" />
  </page>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from "vue";
import { useStore } from "vuex";
import { useRouter } from "vue-router";
import { PATHS } from "@/URLS";
import {
  Question,
  QuizQuestionResult,
  QuizState,
  QuizStatus,
  openEndQuizAlert,
} from "./quiz";

// Store and types
import { Quiz } from "@/lib/models";

// Components
import { IonText, IonInput, IonIcon, IonProgressBar } from "@ionic/vue";
import Page from "@/components/layout/Page.vue";
import Btn from "@/components/base/Btn.vue";
import IconBtn from "@/components/base/IconBtn.vue";
import QuizResultView from "@/views/QuizView/components/QuizResultView.vue";

// Icons
import {
  closeCircleOutline,
  checkmarkCircleOutline,
  close,
} from "ionicons/icons";

export default defineComponent({
  name: "QuizView",
  components: {
    IonText,
    IonInput,
    IonIcon,
    IonProgressBar,
    Page,
    Btn,
    IconBtn,
    QuizResultView,
  },
  setup() {
    const router = useRouter();
    const store = useStore();

    const quiz = computed(
      () =>
        store.getters.getQuizById(router.currentRoute.value.params.id) as Quiz
    );
    if (!quiz.value) {
      router.replace(PATHS.quizLanding());
      return;
    }

    const quizState = ref<QuizState>(
      new QuizState(quiz.value.id, quiz.value.words, quiz.value.options)
    );
    const currentQuestion = ref<Question | null>(quizState.value.startQuiz());
    const input = ref("");
    const inputElement = ref<InstanceType<any> | null>(null);

    const questionResult = ref(QuizQuestionResult.None);
    const isResultNone = computed(
      () => questionResult.value === QuizQuestionResult.None
    );
    const isResultCorrect = computed(
      () => questionResult.value === QuizQuestionResult.Correct
    );
    const isResultIncorrect = computed(
      () => questionResult.value === QuizQuestionResult.Incorrect
    );
    const revealAnswer = ref(false);
    const isQuizFinished = computed(
      () => quizState.value.status === QuizStatus.Finished
    );
    const stats = ref(quizState.value.getStats());
    const showQuizView = ref(true);

    const endQuiz = () => {
      quizState.value.endQuiz();
      // TODO: Finish collapse component, and then call it and render stats and wordss

      setTimeout(() => {
        showQuizView.value = false;
      }, 2000);
    };

    const tryEndQuiz = () => {
      const callback = (success: boolean) => {
        if (success) {
          endQuiz();
        } else {
          // TODO: Remove this
          quizState.value.status = QuizStatus.Ongoing;
        }
      };

      openEndQuizAlert(callback);
    };

    const nextQuestion = () => {
      currentQuestion.value = quizState.value.nextQuestion();
      questionResult.value = QuizQuestionResult.None;
      if (inputElement.value !== null) {
        inputElement.value.$el.setFocus();
        input.value = "";
      }

      if (quizState.value.status === QuizStatus.Finished) {
        endQuiz();
      }
    };

    const showResult = (result: QuizQuestionResult, isAPass = false) => {
      questionResult.value = result;

      if (result === QuizQuestionResult.Correct || isAPass) {
        revealAnswer.value = true;
      }

      setTimeout(() => {
        if (result === QuizQuestionResult.Correct || isAPass) {
          nextQuestion();
        } else {
          questionResult.value = QuizQuestionResult.None;
        }

        revealAnswer.value = false;
      }, 3000);
    };

    /*  const refreshState = () => {
      stats.value = quizState.value.getStats();
      // isQuizFinished.value = quizState.value.status === QuizStatus.Finished;
    }; */

    const checkAnswer = () => {
      showResult(quizState.value.checkAnswer(input.value));
      stats.value = quizState.value.getStats();
    };

    const passQuestion = () => {
      quizState.value.passQuestion();
      stats.value = quizState.value.getStats();
      showResult(QuizQuestionResult.Incorrect, true);
    };

    return {
      quiz,
      currentQuestion,
      input,
      inputElement,
      checkAnswer,
      tryEndQuiz,
      isResultNone,
      isResultCorrect,
      isResultIncorrect,
      isQuizFinished,
      passQuestion,
      revealAnswer,
      stats,
      showQuizView,
      quizState,
      icons: {
        checkmark: checkmarkCircleOutline,
        close: closeCircleOutline,
        close2: close,
      },
    };
  },
});
</script>

<style scoped>
.correct {
  background-color: var(--ion-color-success) !important;
}
.incorrect {
  background-color: var(--ion-color-danger) !important;
}
</style>

<style>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 4s;
}
.fade-enter, .fade-leave-to /* .fade-leave-active below version 2.1.8 */ {
  opacity: 0;
}
</style>
