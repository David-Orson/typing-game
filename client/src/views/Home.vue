<script setup lang="ts">
import { ref, onMounted } from "vue";

const test = ref("");
const typed = ref("");

window.addEventListener("keydown", (e) => {
  let allKeys = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

  allKeys.split("").forEach((c) => {
    if (e.key === c) typed.value += c;
  });

  if (e.key === " ") typed.value += " ";

  if (e.key == "Backspace") {
    typed.value = typed.value.slice(0, -1);
  }
});

onMounted(() => {
  let wordlist =
    "anything then the best world will defeat any bother and I have any great interest ground govern anytime major sensitive west time gripe nose";
  const arr = wordlist.split(" ");

  for (let i = 0; i < 25; i++) {
    if (i !== 0) {
      test.value += " ";
    }

    test.value += arr[Math.floor(Math.random() * arr.length)];
  }
});
</script>

<template>
  <div class="container mx-auto">
    <div class="mx-16 text-4xl text-gray-700">{{ test }}</div>
    <div class="m-16 text-4xl flex justify-center flex-wrap">
      <div v-for="(c, i) in typed">
        <span
          class="w-1"
          :class="test[i] == typed[i] ? 'text-gray-700' : 'text-red-500'"
          >{{ c }}</span
        >
        <span v-if="c == ' '">{{ "_" }}</span>
      </div>
    </div>
  </div>
</template>
