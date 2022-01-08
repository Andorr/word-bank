<template>
  <ion-page>
    <ion-content>
      <div class="h-64 bg-primary w-full">
        <div class="h-full w-full flex justify-center items-center">
          <ion-text class="text-center text-4xl">안녕하세요!</ion-text>
        </div>
      </div>
      <div class="grid grid-cols-1 gap-4 mt-4 px-4">
        <div
          @click="goToQuiz"
          class="
            ion-activatable
            bg-gradient-to-r
            from-green-400
            to-blue-500
            w-full
            h-20
            rounded-lg
            p-4
            flex
            items-center
            relative
            overflow-hidden
          "
        >
          <ion-text class="font-bold">Quiz</ion-text>
          <ion-ripple-effect />
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script lang="ts">
import { defineComponent } from "vue";

// Components
import { IonPage, IonContent, IonText, IonRippleEffect } from "@ionic/vue";
import { PATHS } from "@/URLS";

export default defineComponent({
  name: "Landing",
  components: {
    IonPage,
    IonContent,
    IonText,
    IonRippleEffect,
  },
  methods: {
    goToQuiz() {
      this.$router.push(PATHS.quizLanding());
    },
  },
});
</script>

<style scoped></style>
