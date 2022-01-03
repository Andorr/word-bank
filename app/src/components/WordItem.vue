<template>
  <ion-item :key="word.id" :button="true" :detail-icon="null">
    <div class="w-full my-2">
      <h4 class="mb-0 font-bold">
        {{ word.value }}
      </h4>
      <p class="text-gray-300 mb-0 text-sm">
        {{ word.translations.map((t) => t.value).join(", ") }}
      </p>
      <ion-ripple-effect></ion-ripple-effect>
    </div>
    <p slot="end" class="text-gray-400 text-xs" :class="wordTypeColor">
      {{ word.kind }}
    </p>
  </ion-item>
</template>

<script lang="ts">
import { defineComponent } from "vue";

// Logic
import { Word } from "@/lib/models";

// Components
import { IonItem, IonRippleEffect } from "@ionic/vue";

// Icons
import { language } from "ionicons/icons";

const WORDTYPE_COLOR = {
  NONE: "text-none",
  NOUN: "text-noun",
  PRONOUN: "text-pronoun",
  VERB: "text-verb",
  ADJECTIVE: "text-adjective",
  ADVERB: "text-adverb",
  PREPOSITION: "text-preposition",
  CONJUNCTION: "text-determiner", // TODO
  INTERJECTION: "text-determiner", // TODO
  DETERMINER: "text-determiner",
  OTHER: "text-other",
} as Record<string, string>;

export default defineComponent({
  name: "WordItem",
  components: {
    IonItem,
    IonRippleEffect,
  },
  props: {
    word: Word,
  },
  computed: {
    wordTypeColor(): string {
      return (
        WORDTYPE_COLOR[this.word ? this.word.kind : ""] || WORDTYPE_COLOR.NONE
      );
    },
  },
  data() {
    return {
      icons: { language },
    };
  },
});
</script>

<style scoped>
.root > div {
  padding-left: 0px !important;
}

.ripple-parent {
  position: relative;
  overflow: hidden;
}
</style>
