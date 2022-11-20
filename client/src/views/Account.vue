<script setup lang="ts">
// npm
import { onMounted, ref } from "vue";

// primevue
import DataTable from "primevue/datatable";
import Column from "primevue/column";

// hooks
import { useStore } from "@/store";
import { useServices } from "@/data/services";

// models
import { Test, Account } from "@/data/models";

const store = useStore();
const { getTests, getAccount } = useServices();

// reactive
const tests = ref<Test[]>([]);
const account = ref<Account>();

onMounted(async () => {
  tests.value = await getTests(store.getters.account.id);
  account.value = await getAccount(store.getters.account.id);
});
</script>

<template>
  <div class="container mx-auto mb-4 flex justify-evenly">
    <div class="text-4xl">{{ account?.username }}</div>
    <div class="text-4xl">Personal Record: {{ account?.pr }}</div>
  </div>
  <div class="container mx-auto">
    <DataTable :value="tests" responsiveLayout="scroll">
      <Column field="wpm" header="WPM"></Column>
      <Column field="accuracy" header="Accuracy"></Column>
    </DataTable>
  </div>
</template>
