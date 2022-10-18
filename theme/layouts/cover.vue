<template>
  <div class="slidev-layout cover">
    <slot />
  </div>
</template>
<style scoped lang="postcss">
.cover {
  background-image: url('/assets/bg-cover.svg');
  background-repeat: no-repeat;
  background-size: cover;
  background-position: center center;
}
::v-deep(h1) {
  @apply !text-5xl;
}
::v-deep(p) {
  @apply text-3xl;
}
</style>
