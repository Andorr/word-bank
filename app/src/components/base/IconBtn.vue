<template>
    <button
        ion-button
        class="ion-activatable w-12 h-12 text-primary rounded-full flex justify-center items-center overflow-hidden relative"
        v-bind="$attrs"
    >
        <slot />
        <ion-ripple-effect />
    </button>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

import { IonRippleEffect } from '@ionic/vue';

export default defineComponent({
    name: 'IconBtn',
    components: { IonRippleEffect },
})
</script>

<style scoped>

</style>