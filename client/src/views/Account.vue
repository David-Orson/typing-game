<script setup lang="ts">
// npm
import { onMounted, ref } from "vue";

// hooks
import { useStore } from "@/store";
import { useServices } from "@/data/services";

// models
import { Test } from "@/data/models";

const store = useStore();
const { getTests } = useServices();

// reactive
const tests = ref<Test[]>([]);

onMounted(async () => {
  tests.value = await getTests(store.getters.account.id);
});
</script>

<template>
  <div class="container mx-auto">{{ store.getters.account.username }}</div>
  <div>{{ tests }}</div>
</template>
