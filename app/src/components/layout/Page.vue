<template>
    <ion-page>
        <ion-header translucent>
            <ion-toolbar>
                <ion-buttons slot="start">
                    <ion-back-button></ion-back-button>
                </ion-buttons>
                <ion-title v-if='title'> {{ title }} </ion-title>
                <div />
                <slot slot="end" name='header-right'></slot>
            </ion-toolbar>
            <slot name='header'></slot>
        </ion-header>
        <ion-content fullscreen>
            <slot />
        </ion-content>
    </ion-page>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

// Components
import {
    IonPage,
    IonHeader,
    IonToolbar,
    IonTitle,
    IonButtons,
    IonBackButton,
    IonContent,
} from '@ionic/vue';

export default defineComponent({
    name: 'Page',
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
    }
})
</script>

<style scoped>

</style>