<template>
  <ion-app>
    <ion-router-outlet />
  </ion-app>
</template>

<script lang="ts">
import { LocalNotifications } from "@capacitor/local-notifications";
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
    LocalNotifications.requestPermissions().then((status) => {
      initializeStorage()
        .then(() => {
          if (status.display === "granted") {
            return initializeWordReminderNotifications();
          }
        })
        .catch(console.error);
    });
  },
});
</script>
