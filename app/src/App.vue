<template>
  <ion-app>
    <ion-router-outlet />
  </ion-app>
</template>

<script lang="ts">
import { IonApp, IonRouterOutlet } from "@ionic/vue";
import { defineComponent } from "vue";
import { initializeWordReminderNotifications } from "./native/notifications";
import { initializeStorage } from "./native/storage";

export default defineComponent({
  name: "App",
  components: {
    IonApp,
    IonRouterOutlet,
  },
  mounted() {
    initializeStorage()
      .then(() => {
        return initializeWordReminderNotifications();
      })
      .catch(console.error);
  },
});
</script>
