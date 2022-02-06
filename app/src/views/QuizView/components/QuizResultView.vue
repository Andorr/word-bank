<template>
  <div ref="root" class="absolute inset-0 h-full w-full opacity-0 z-20">
    <div class="w-full h-screen flex flex-col" :class="styles.bgColor">
      <div class="flex-grow flex flex-col justify-center items-center">
        <p class="text-7xl text-center mb-6">{{ styles.emoji }}</p>
        <p v-if="stats && finishedAnyQuestion" class="text-4xl text-center">
          {{ percentage.toFixed(0) }} %
        </p>
        <p v-if="stats && finishedAnyQuestion" class="text-xl text-center">
          {{ stats.numCorrects }} /
          {{ stats.numCorrects + stats.numIncorrects }}
        </p>
        <p v-if="!finishedAnyQuestion" class="text-xl text-center">
          What was that?
        </p>
      </div>
      <div class="px-4 my-4 w-full">
        <btn
          class="w-full shadow-md bg-white text-gray-700"
          :disabled="isLoading"
          @click="exitQuiz"
        >
          Continue
        </btn>
      </div>
    </div>
    <div v-if="questions.length > 0" class="m-5 mb-6">
      <div class="ml-5 mb-6">
        <ion-text class="text-2xl font-bold">Results:</ion-text>
      </div>
      <ion-item v-for="(q, i) in questions" :key="i">
        <ion-label>
          {{ q.question }}
          <p class="text-gray-300 mb-0 text-sm">
            {{ q.answers.join(", ") }}
          </p>
        </ion-label>

        <div slot="end">
          <div
            v-if="q.numCorrects > 0"
            class="flex items-center text-green-400 text-xl flex-grow"
          >
            <ion-icon class="mr-2" :icon="icons.checkmark" />
            <span>{{ q.numCorrects }}</span>
          </div>
          <div
            v-if="q.numIncorrects > 0"
            class="flex items-center text-red-400 text-xl flex-grow"
          >
            <ion-icon class="mr-2" :icon="icons.close" />
            <span>{{ q.numIncorrects }}</span>
          </div>
          <div
            v-if="stats.numIncorrects === 0 && stats.numCorrects === 0"
            class="flex items-center text-primary text-xl flex-grow"
          >
            <ion-icon class="mr-2" :icon="icons.minus" />
          </div>
        </div>
      </ion-item>
    </div>
    <div class="m-6"></div>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, ref } from "vue";

import { createAnimation } from "@ionic/vue";
import { QuizState } from "../quiz";

// Components
import { IonItem, IonLabel, IonIcon, IonText } from "@ionic/vue";
import Btn from "@/components/base/Btn.vue";

// Icons
import {
  closeCircleOutline,
  checkmarkCircleOutline,
  removeCircleOutline,
} from "ionicons/icons";
import { useRouter } from "vue-router";
import { PATHS } from "@/URLS";
import { useStore } from "vuex";
import { ACTIONS } from "@/store/actions";

export default defineComponent({
  name: "QuizResultView",
  components: { IonItem, IonLabel, IonIcon, IonText, Btn },
  props: {
    quizState: {
      type: QuizState,
      required: true,
    },
  },
  setup(props) {
    const root = ref();
    const router = useRouter();
    const store = useStore();

    // Data
    const isLoading = ref(false);

    const anim = createAnimation()
      .duration(1400)
      .fromTo("opacity", "0.0", "1.0");

    const stats = props.quizState.getStats();
    const percentage = ref(0.0);
    const finishedAnyQuestion = props.quizState.questions.some(
      (q) => q.numCorrects > 0 || q.numIncorrects > 0
    );
    const emoji = finishedAnyQuestion
      ? stats.scorePercentage >= 0.5
        ? "🎉"
        : "🙈"
      : "😅";
    const bgColor = finishedAnyQuestion
      ? stats.scorePercentage >= 0.5
        ? "bg-green-500"
        : "bg-red-500"
      : "bg-primary";

    const lerpPercentage = (step = 0.0) => {
      percentage.value = stats.scorePercentage * step * 100;

      if (step < 1.0) {
        setTimeout(() => lerpPercentage(step + 0.02), 36);
      }
    };

    const saveQuizResult = () => {
      isLoading.value = true;

      store
        .dispatch(ACTIONS.QUIZ_INSERT_RESULT, props.quizState.toQuizResult())
        .catch(() => {
          // TODO: Show pop-up error
        })
        .finally(() => {
          isLoading.value = false;
        });
    };

    onMounted(() => {
      anim.addElement(root.value);
      setTimeout(() => {
        anim.play();
      }, 100);

      if (finishedAnyQuestion) {
        setTimeout(() => {
          lerpPercentage(0.0);
        }, 1000);
      }

      saveQuizResult();
    });

    const exitQuiz = () => {
      router.replace(PATHS.home());
    };

    return {
      root,
      percentage,
      questions: props.quizState.questions,
      stats: props.quizState.getStats(),
      isLoading,
      exitQuiz,
      icons: {
        checkmark: checkmarkCircleOutline,
        close: closeCircleOutline,
        minus: removeCircleOutline,
      },
      styles: {
        emoji,
        bgColor,
      },
      finishedAnyQuestion,
    };
  },
});
</script>

<style scoped></style>
