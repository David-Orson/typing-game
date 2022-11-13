<script setup lang="ts">
import { useStore } from "@/store";
const store = useStore();
</script>

<template>
  <div class="container mx-auto">{{ store.getters.account.username }}</div>
</template>
