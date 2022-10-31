<script setup lang="ts">
import { ref, onMounted } from "vue";

const wpm = ref();
const test = ref("");
const typed = ref("");
const timeLeft = ref(15);
const intervalId = ref();

const wordlist =
  "anything then the best world will defeat any bother and I have any great interest ground govern anytime major sensitive west time gripe nose";

window.addEventListener("keydown", (e) => {
  let allKeys = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

  allKeys.split("").forEach((c) => {
    if (e.key === c) typed.value += c;
  });

  if (e.key === " ") typed.value += " ";

  if (e.key == "Backspace") {
    typed.value = typed.value.slice(0, -1);
  }

  if (e.key == "Enter") {
    reset();
  }
});

const reset = () => {
  clearInterval(intervalId.value);

  timeLeft.value = 15;

  const arr = wordlist.split(" ");

  test.value = "";
  typed.value = "";

  for (let i = 0; i < 35; i++) {
    if (i !== 0) {
      test.value += " ";
    }

    test.value += arr[Math.floor(Math.random() * arr.length)];
  }

  intervalId.value = setInterval(() => {
    timeLeft.value -= 1;
    wpm.value = ((typed.value.length / 5) * 60) / (15 - timeLeft.value);

    if (timeLeft.value === 0) {
      clearInterval(intervalId.value);
    }
  }, 1000);
};

onMounted(() => {
  reset();
});
</script>

<template>
  <div class="container mx-auto">
    <div>{{ timeLeft }} {{ wpm }}</div>
    <div class="mx-16 text-4xl text-gray-700">{{ test }}</div>
    <div class="m-16 text-4xl flex justify-center flex-wrap">
      <div class="flex" v-for="(word, i) in typed.split(' ')">
        <div v-for="(c, j) in word.split('')">
          <span
            class="w-1"
            :class="
              test.split(' ')[i] &&
              test.split(' ')[i][j] &&
              test.split(' ')[i][j] == typed.split(' ')[i][j]
                ? 'text-gray-700'
                : 'text-red-500'
            "
            >{{ c }}</span
          >
        </div>
        <div class="w-2"></div>
      </div>
    </div>
  </div>
</template>
