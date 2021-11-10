<template>
    <button
        class="base-btn ion-activatable px-4 py-2 rounded overflow-hidden"
        :class='{"disabled": disabled}'
        v-bind="$attrs"
        :disabled='disabled'
    >
        <slot />
        <ion-ripple-effect v-if='!disabled' />
    </button>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

// Components
import {
    IonRippleEffect
} from '@ionic/vue';

export default defineComponent({
    name: 'Btn',
    components: {
        IonRippleEffect,
    },
    props: {
        disabled: {
            type: Boolean,
            default: false,
        }
    },
})
</script>

<style scoped>
.base-btn {
    background-color: var(--ion-color-primary);
}

.disabled {
    filter: brightness(70%);
}
</style>