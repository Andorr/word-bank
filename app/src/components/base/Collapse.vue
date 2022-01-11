<template>
  <div ref="root" class="root overflow-hidden">
    <slot />
  </div>
</template>

<script lang="ts">
import { AnimationDirection, createAnimation } from "@ionic/vue";
import { defineComponent, onMounted, ref, watch } from "vue";

export default defineComponent({
  name: "Collapse",
  props: {
    visibile: Boolean,
    duration: {
      type: Number,
      default: 1000,
    },
    heightStyle: {
      type: String,
      default: "100%",
    },
  },
  setup(props) {
    const root = ref<InstanceType<any> | null>(null);

    const open = createAnimation()
      .duration(props.duration || 1000)
      .iterations(1)
      // .easing("ease-out")
      .fromTo("height", "0px", props.heightStyle || "100%");

    watch(
      () => props.visibile,
      () => {
        open.direction(props.visibile ? "normal" : "reverse");
        if (props.visibile) {
          root.value.style.height = props.heightStyle || "100%";
        } else {
          root.value.style.height = "0px";
        }
        // open.play();
      }
    );

    onMounted(() => {
      if (props.visibile) {
        root.value.style.height = props.heightStyle || "100%";
      } else {
        root.value.style.height = "0px";
      }

      console.log(root.value);
      open
        .direction(props.visibile ? "normal" : "reverse")
        .addElement(root.value);
    });

    return {
      root,
    };
  },
});
</script>

<style scoped>
.root {
  transition: height 2s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
  height: auto;
}
</style>
