<template>
  <ion-page>
    <ion-header v-if="header" translucent>
      <ion-toolbar>
        <ion-buttons v-if="backButton" slot="start">
          <ion-back-button></ion-back-button>
        </ion-buttons>
        <ion-title v-if="title"> {{ title }} </ion-title>
        <div v-if="backButton || title" />
        <slot slot="end" name="header-right"></slot>
      </ion-toolbar>
      <slot name="header"></slot>
    </ion-header>
    <ion-content fullscreen>
      <slot />
    </ion-content>
  </ion-page>
</template>

<script lang="ts">
import { defineComponent } from "vue";

// Components
import {
  IonPage,
  IonHeader,
  IonToolbar,
  IonTitle,
  IonButtons,
  IonBackButton,
  IonContent,
} from "@ionic/vue";

export default defineComponent({
  name: "Page",
  components: {
    IonPage,
    IonHeader,
    IonToolbar,
    IonTitle,
    IonButtons,
    IonBackButton,
    IonContent,
  },
  props: {
    title: String,
    header: {
      type: Boolean,
      default: true,
    },
    backButton: {
      type: Boolean,
      default: true,
    },
  },
});
</script>

<style scoped></style>
